//! シミュレータ本体（`static/index.html`）の、DOM との受け渡し。wasm32 のときだけ。
//!
//! 図に出す中身を決める判断は `cornix_logic` に置き、ここは、それを描くだけにする。
//! 振る舞いは `docs/design.md` の B-01〜B-08・B-11〜B-45。

use std::cell::RefCell;
use std::rc::Rc;

use cornix_logic::caution;
use cornix_logic::chars::Layout;
use cornix_logic::engine::Engine;
use cornix_logic::geometry::{KeyShape, SHAPES};
use cornix_logic::hostmap::position_of;
use cornix_logic::sample::SAMPLE_VIL;
use cornix_logic::text::TextField;
use cornix_logic::view::{Cell, Shown, Tone, View, render};
use cornix_logic::vil::{READ_FAILED_MESSAGE, parse};
use js_sys::Promise;
use wasm_bindgen::prelude::*;
use web_sys::{
    Document, Element, Event, FileReader, HtmlElement, HtmlInputElement, KeyboardEvent, Window,
};

use crate::dom::{call, keyboard, listen};

const SVG_NS: &str = "http://www.w3.org/2000/svg";
/// キー1つ分の間隔（図の中の長さ）。
const UNIT: f64 = 60.0;
/// キーの四角の一辺（丸い部品の直径）。間隔より少し小さくして、すき間を空ける。
const SIDE: f64 = 56.0;
/// 刻印に使える幅。これより長い刻印は、幅に合わせて詰める。
const LEGEND_WIDTH: f64 = 50.0;

/// 打った結果として持つ物。判断は `cornix_logic` の部品がして、ここは、持って渡すだけ。
struct State {
    engine: Engine,
    /// 配列（JIS・US）。
    layout: Layout,
    /// 表示するレイヤー（「自動」か番号）。
    shown: Shown,
    /// 手元のキーの名前を出すか。
    host_labels: bool,
    /// Keyboard Lock が掛かっているか（全画面の間だけ `true` になる）。
    keyboard_locked: bool,
    /// 出た文字の欄。
    field: TextField,
    /// 「直前のキー」の文。
    last_key: String,
}

/// 読み取りの結果を受ける関数。
type ReadHandler = Closure<dyn FnMut(Event) -> Result<(), JsValue>>;

/// ファイル1つの読み取り。読む物と、結果を受ける関数を、いっしょに持つ
/// （次のファイルを選んだときに、まとめて捨てるため）。
struct Reading {
    reader: FileReader,
    _on_load: ReadHandler,
    _on_error: ReadHandler,
}

/// ページの中の、読み書きする部品と、いまの状態。
struct App {
    window: Window,
    document: Document,
    /// 全画面にする要素（ページの全体）。
    root: Element,
    /// 「全画面で試す」のボタン。
    fullscreen: HtmlElement,
    /// 注意の文を出す所。
    caution: Element,
    /// 案内のうち、いま出ているキーマップの文。
    guide: Element,
    /// 案内のうち、知らせ（動かない物の数）。
    notice: Element,
    /// 案内のうち、読めなかった理由。
    load_error: Element,
    /// 「.vil を選ぶ」のボタン。
    choose_vil: HtmlElement,
    /// ファイルを選ぶ部品（画面には出さず、ボタンから開く）。
    vil_file: HtmlInputElement,
    /// いちばん後に選んだファイルの、読み取り。
    reading: RefCell<Option<Reading>>,
    /// 配列の選択（JIS・US の順）。
    layouts: [HtmlInputElement; 2],
    /// 「手元のキーの名前」のチェック。
    host_labels: HtmlInputElement,
    /// レイヤーの列の、「自動」と番号のボタンを置く所。
    layer_buttons: Element,
    /// 配列の図（SVG）。
    figure: Element,
    /// 有効なレイヤーの番号を出す所。
    active_layers: Element,
    /// 修飾キーの札（Shift・Ctrl・Alt・GUI の順）。
    badges: [Element; 4],
    /// 「直前のキー」を出す所。
    last_key: Element,
    /// 欄。
    field: Element,
    /// 「キーを全部離す」のボタン。
    release_all: HtmlElement,
    /// 「全部消す」のボタン。
    clear: HtmlElement,
    state: RefCell<State>,
}

impl App {
    /// `index.html` の中から部品を探し、見本のキーマップで、何も押していない状態を作る。
    fn find(window: Window) -> Result<Self, JsValue> {
        let document = window
            .document()
            .ok_or_else(|| JsValue::from_str("document がありません"))?;
        let root = document
            .document_element()
            .ok_or_else(|| JsValue::from_str("ページの全体の要素がありません"))?;
        let keymap = parse(SAMPLE_VIL).map_err(|error| JsValue::from_str(&error.to_string()))?;
        Ok(Self {
            root,
            fullscreen: element(&document, "fullscreen")?,
            caution: element(&document, "caution")?,
            guide: element(&document, "guide-text")?,
            notice: element(&document, "notice")?,
            load_error: element(&document, "load-error")?,
            choose_vil: element(&document, "choose-vil")?,
            vil_file: element(&document, "vil-file")?,
            reading: RefCell::new(None),
            layouts: [
                element(&document, "layout-jis")?,
                element(&document, "layout-us")?,
            ],
            host_labels: element(&document, "host-labels")?,
            layer_buttons: element(&document, "layer-buttons")?,
            figure: element(&document, "figure")?,
            active_layers: element(&document, "active-layers")?,
            badges: [
                element(&document, "mod-shift")?,
                element(&document, "mod-ctrl")?,
                element(&document, "mod-alt")?,
                element(&document, "mod-gui")?,
            ],
            last_key: element(&document, "last-key")?,
            field: element(&document, "field")?,
            release_all: element(&document, "release-all")?,
            clear: element(&document, "clear")?,
            window,
            document,
            state: RefCell::new(State {
                engine: Engine::new(keymap),
                layout: Layout::Jis,
                shown: Shown::Auto,
                host_labels: true,
                keyboard_locked: false,
                field: TextField::default(),
                last_key: String::new(),
            }),
        })
    }

    /// 選んだ .vil の中身（文字列）を読む。
    ///
    /// 読めたら、案内と知らせを変え、キーの状態を、何も押していない状態に、表示するレイヤーを
    /// 「自動」に戻す。欄・「直前のキー」・配列の選択は変えない（B-06・B-07）。
    /// 読めないときは、理由を出すだけで、図と状態は前のまま（B-08）。
    fn load(&self, name: &str, text: &str) -> Result<(), JsValue> {
        match parse(text) {
            Ok(keymap) => {
                self.guide
                    .set_text_content(Some(&keymap.loaded_message(name)));
                self.notice
                    .set_text_content(keymap.notices().message().as_deref());
                self.load_error.set_text_content(None);
                let mut state = self.state.borrow_mut();
                state.engine = Engine::new(keymap);
                state.shown = Shown::Auto;
            }
            Err(error) => {
                self.load_error
                    .set_text_content(Some(&error.failed_message()));
            }
        }
        self.refresh()
    }

    /// ページが（JavaScript から頼んだ）全画面になっているか。
    fn is_fullscreen(&self) -> bool {
        self.document.fullscreen_element().is_some()
    }

    /// 全画面のボタンの文と、注意の文を、いまの状態のとおりに書き直す（B-42・B-43）。
    fn show_screen(&self) {
        let fullscreen = self.is_fullscreen();
        self.fullscreen
            .set_text_content(Some(caution::button_label(fullscreen)));
        let locked = self.state.borrow().keyboard_locked;
        self.caution
            .set_text_content(Some(caution::text(fullscreen, locked)));
    }

    /// 図・レイヤーの列・修飾キーの札・「直前のキー」・欄を、いまの状態のとおりに書き直す。
    fn refresh(&self) -> Result<(), JsValue> {
        let state = self.state.borrow();
        let engine = &state.engine;
        let view = render(engine, state.layout, state.shown, state.host_labels);
        draw(&self.document, &self.figure, &view)?;
        // レイヤーの列は、「自動」と、読み込んだキーマップのレイヤーの番号（B-18）
        self.layer_buttons.set_text_content(None);
        let layers = (0..engine.keymap().layer_count()).map(Shown::Layer);
        for choice in std::iter::once(Shown::Auto).chain(layers) {
            let (label, value) = match choice {
                Shown::Auto => ("自動".to_owned(), "auto".to_owned()),
                Shown::Layer(layer) => (layer.to_string(), layer.to_string()),
            };
            let button = self.document.create_element("button")?;
            button.set_attribute("type", "button")?;
            button.set_attribute("data-layer", &value)?;
            if choice == state.shown {
                button.set_attribute("class", "on")?;
            }
            button.set_text_content(Some(&label));
            self.layer_buttons.append_child(&button)?;
        }
        // 「有効なレイヤー: 0・1」の形（B-19）
        self.active_layers
            .set_text_content(Some(&view.active_layers_text()));
        // 札は、左右のどちらかが入っていれば点く（B-32）
        let modifiers = [engine.shift(), engine.ctrl(), engine.alt(), engine.gui()];
        for (badge, on) in self.badges.iter().zip(modifiers) {
            badge.set_attribute("class", if on { "badge on" } else { "badge" })?;
        }
        self.last_key.set_text_content(Some(&state.last_key));
        self.field.set_text_content(Some(state.field.text()));
        Ok(())
    }

    /// 押されているキーを、全部離した扱いにする（B-24）。
    fn release_all(&self) -> Result<(), JsValue> {
        self.state.borrow_mut().engine.release_all();
        self.refresh()
    }
}

/// シミュレータを動かす。`static/index.html` が、読み込みのあとに1回だけ呼ぶ。
///
/// 最初は、見本のキーマップ・JIS・「自動」・手元のキーの名前を出す、で図を描く（B-01・B-04）。
///
/// # Errors
///
/// ページに要る部品（id の付いた要素）が見つからないとき、見本のキーマップが読めないとき、
/// ブラウザがイベントの登録を断ったとき。
#[wasm_bindgen]
pub fn run_simulator() -> Result<(), JsValue> {
    let window = web_sys::window().ok_or_else(|| JsValue::from_str("window がありません"))?;
    let app = Rc::new(App::find(window)?);
    app.show_screen();
    app.refresh()?;
    listen_keys(&app)?;
    listen_file(&app)?;
    listen_switches(&app)?;
    listen_screen(&app)?;

    // ボタンを押したあとは、フォーカスを外す（手元の Space や Enter で、また押されないように）
    let shared = Rc::clone(&app);
    listen(&app.release_all, "click", move |_: Event| {
        shared.release_all()?;
        shared.release_all.blur()
    })?;

    // 「全部消す」は、欄と「直前のキー」を空にする。キーの状態は変えない（B-40）
    let shared = Rc::clone(&app);
    listen(&app.clear, "click", move |_: Event| {
        {
            let mut state = shared.state.borrow_mut();
            state.field.clear();
            state.last_key.clear();
        }
        shared.refresh()?;
        shared.clear.blur()
    })
}

/// 手元のキーと、全部離すきっかけ（フォーカスを失う・ページが隠れる）を受ける。
fn listen_keys(app: &Rc<App>) -> Result<(), JsValue> {
    // 手元のキーの keydown・keyup を、対応表の位置の「押す・押し続け・離す」にする（B-20・B-23）。
    // 対応表にあるキーだけ、ブラウザの既定の動きを止める（B-21）
    for (kind, down) in [("keydown", true), ("keyup", false)] {
        let shared = Rc::clone(app);
        listen(&app.window, kind, move |event: KeyboardEvent| {
            let Some((row, col)) = position_of(&event.code()) else {
                return Ok(());
            };
            event.prevent_default();
            {
                let mut state = shared.state.borrow_mut();
                let layout = state.layout;
                if !down {
                    state.engine.release(row, col);
                } else if event.repeat() {
                    let action = state.engine.repeat(row, col, layout);
                    state.field.apply(action);
                } else if let Some(outcome) = state.engine.press(row, col, layout) {
                    state.field.apply(outcome.action);
                    state.last_key = outcome.last_key;
                }
            }
            shared.refresh()
        })?;
    }

    // 窓がフォーカスを失ったときと、ページが隠れたときは、全部離す（B-24）
    let shared = Rc::clone(app);
    listen(&app.window, "blur", move |_: Event| shared.release_all())?;
    let shared = Rc::clone(app);
    listen(&app.document, "visibilitychange", move |_: Event| {
        if shared.document.hidden() {
            shared.release_all()?;
        }
        Ok(())
    })
}

/// 「.vil を選ぶ」を受ける（B-05）。
fn listen_file(app: &Rc<App>) -> Result<(), JsValue> {
    // ボタンは、ファイルを選ぶ部品を開く。選んだファイルは、文字列として読む
    let shared = Rc::clone(app);
    listen(&app.choose_vil, "click", move |_: Event| {
        shared.vil_file.click();
        shared.choose_vil.blur()
    })?;
    let shared = Rc::clone(app);
    listen(&app.vil_file, "change", move |_: Event| {
        let Some(file) = shared.vil_file.files().and_then(|files| files.get(0)) else {
            return Ok(());
        };
        // 読む物は、選ぶたびに作る（前のファイルを読んでいる途中でも、名前と中身が食い違わないように）。
        // 前の読み取りは、やめる（いちばん後に選んだファイルが出るように）
        let reader = FileReader::new()?;
        let name = file.name();
        let loaded = reader.clone();
        let app = Rc::clone(&shared);
        let on_load = ReadHandler::new(move |_: Event| {
            let text = loaded.result()?.as_string().unwrap_or_default();
            app.load(&name, &text)
        });
        // 読み取り自体が失敗したら、理由を出すだけで、図と状態は前のまま（B-08）
        let app = Rc::clone(&shared);
        let on_error = ReadHandler::new(move |_: Event| {
            app.load_error.set_text_content(Some(READ_FAILED_MESSAGE));
            Ok(())
        });
        reader.set_onload(Some(on_load.as_ref().unchecked_ref()));
        reader.set_onerror(Some(on_error.as_ref().unchecked_ref()));
        reader.read_as_text(&file)?;
        // 関数は、次のファイルを選ぶまで生かしておく。前の物は、外してから捨てる
        let previous = shared.reading.replace(Some(Reading {
            reader,
            _on_load: on_load,
            _on_error: on_error,
        }));
        if let Some(previous) = previous {
            previous.reader.set_onload(None);
            previous.reader.set_onerror(None);
            previous.reader.abort();
        }
        // 同じファイルを選び直しても、change が届くようにする
        shared.vil_file.set_value("");
        Ok(())
    })
}

/// 切り替え（配列・手元のキーの名前・レイヤーの列）を受ける。
fn listen_switches(app: &Rc<App>) -> Result<(), JsValue> {
    // 配列の切り替え。刻印と、そのあとに出る文字が変わる。欄とキーの状態は変えない（B-36）
    for (index, layout) in [Layout::Jis, Layout::Us].into_iter().enumerate() {
        let shared = Rc::clone(app);
        listen(&app.layouts[index], "change", move |_: Event| {
            shared.state.borrow_mut().layout = layout;
            shared.refresh()?;
            shared.layouts[index].blur()
        })?;
    }

    // 「手元のキーの名前」のチェック（B-16）
    let shared = Rc::clone(app);
    listen(&app.host_labels, "change", move |_: Event| {
        shared.state.borrow_mut().host_labels = shared.host_labels.checked();
        shared.refresh()?;
        shared.host_labels.blur()
    })?;

    // レイヤーの列のボタン（B-18）。ボタンは描き直すたびに作り直すので、置く所でまとめて受ける
    let shared = Rc::clone(app);
    listen(&app.layer_buttons, "click", move |event: Event| {
        let Some(value) = event
            .target()
            .and_then(|target| target.dyn_into::<Element>().ok())
            .and_then(|button| button.get_attribute("data-layer"))
        else {
            return Ok(());
        };
        // 番号でない物（"auto"）は、「自動」
        shared.state.borrow_mut().shown = value.parse().map_or(Shown::Auto, Shown::Layer);
        shared.refresh()
    })
}

/// 全画面と Keyboard Lock を受ける（B-42〜B-44）。
fn listen_screen(app: &Rc<App>) -> Result<(), JsValue> {
    // 「全画面で試す」（B-42）。押すたびに、全画面に入る・抜ける
    let shared = Rc::clone(app);
    listen(&app.fullscreen, "click", move |_: Event| {
        if shared.is_fullscreen() {
            shared.document.exit_fullscreen();
        } else {
            shared.root.request_fullscreen()?;
        }
        shared.fullscreen.blur()
    })?;

    // lock() の結果を受ける関数。許可を待つ間に全画面を抜けていたら、掛かっていないので、何もしない。
    // 断られたときは、掛かっていないままなので、何もしない（注意の文は、掛かっていないときの文のまま）
    let shared = Rc::clone(app);
    let on_locked = Closure::<dyn FnMut(JsValue)>::new(move |_: JsValue| {
        if shared.is_fullscreen() {
            shared.state.borrow_mut().keyboard_locked = true;
            shared.show_screen();
        }
    });
    let on_refused = Closure::<dyn FnMut(JsValue)>::new(|_: JsValue| {});

    // 全画面に入ったら Keyboard Lock を掛け、抜けたら外して、全部離す（B-44。どの抜け方でも、ここに来る）。
    // 呼び方は、計測ページで通った方法（design 6節の M-8）
    let shared = Rc::clone(app);
    listen(&app.document, "fullscreenchange", move |_: Event| {
        let keyboard = keyboard(&shared.window)?;
        if shared.is_fullscreen() {
            if let Some(keyboard) = keyboard {
                let promise: Promise = call(&keyboard, "lock")?.dyn_into()?;
                // then2 が返す Promise は使わない（結果は、2つの関数が受ける）
                let _ = promise.then2(&on_locked, &on_refused);
            }
            shared.show_screen();
            Ok(())
        } else {
            if let Some(keyboard) = keyboard {
                call(&keyboard, "unlock")?;
            }
            shared.state.borrow_mut().keyboard_locked = false;
            shared.show_screen();
            shared.release_all()
        }
    })
}

/// id で要素を取る。無いとき・種類が違うときは、id を入れた文のエラーにする。
fn element<T: JsCast>(document: &Document, id: &str) -> Result<T, JsValue> {
    document
        .get_element_by_id(id)
        .and_then(|element| element.dyn_into().ok())
        .ok_or_else(|| JsValue::from_str(&format!("index.html に #{id} がありません")))
}

/// 配列の図を、渡した中身で描き直す（B-11）。
fn draw(document: &Document, figure: &Element, view: &View) -> Result<(), JsValue> {
    figure.set_text_content(None);
    for (shape, cell) in SHAPES.iter().zip(&view.cells) {
        let key = draw_key(document, shape, cell)?;
        figure.append_child(&key)?;
    }
    Ok(())
}

/// キー1つ（か、丸い部品1つ）を描く。回転は、中心のまわりに掛けてから、中に形を置く（3.3節）。
fn draw_key(document: &Document, shape: &KeyShape, cell: &Cell) -> Result<Element, JsValue> {
    let group = document.create_element_ns(Some(SVG_NS), "g")?;
    let tone = match cell.tone {
        Tone::Normal => "normal",
        Tone::Dim => "dim",
        Tone::Unsupported => "unsupported",
        Tone::Empty => "empty",
    };
    let pressed = if cell.pressed { " pressed" } else { "" };
    group.set_attribute("class", &format!("key {tone}{pressed}"))?;
    if let Some((x, y)) = shape.pivot {
        let rotate = format!("rotate({} {} {})", shape.rotation, x * UNIT, y * UNIT);
        group.set_attribute("transform", &rotate)?;
    }

    let (left, top) = (shape.x * UNIT, shape.y * UNIT);
    let center = left + UNIT / 2.0;
    let outline = if shape.knob {
        let circle = document.create_element_ns(Some(SVG_NS), "circle")?;
        set(&circle, "cx", center)?;
        set(&circle, "cy", top + UNIT / 2.0)?;
        set(&circle, "r", SIDE / 2.0)?;
        circle
    } else {
        let gap = (UNIT - SIDE) / 2.0;
        let rect = document.create_element_ns(Some(SVG_NS), "rect")?;
        set(&rect, "x", left + gap)?;
        set(&rect, "y", top + gap)?;
        set(&rect, "width", SIDE)?;
        set(&rect, "height", SIDE)?;
        set(&rect, "rx", 6.0)?;
        rect
    };
    group.append_child(&outline)?;

    // 刻印: 上（Shift 側）・下・隅の手元の刻印。（class、文字、上の端からの高さ、文字の大きさ）
    let lower_y = if cell.upper.is_empty() { 34.0 } else { 39.0 };
    let lower_size = if cell.tone == Tone::Unsupported {
        8.0
    } else {
        14.0
    };
    let host = cell.host_label.as_deref().unwrap_or_default();
    for (class, text, y, size) in [
        ("upper", cell.upper.as_str(), 19.0, 12.0),
        ("lower", cell.lower.as_str(), lower_y, lower_size),
        ("host", host, 53.0, 7.5),
    ] {
        if text.is_empty() {
            continue;
        }
        let legend = document.create_element_ns(Some(SVG_NS), "text")?;
        legend.set_attribute("class", class)?;
        set(&legend, "x", center)?;
        set(&legend, "y", top + y)?;
        set(&legend, "font-size", size)?;
        // 幅に収まらない刻印は、幅に合わせて詰める（半角は、全角の 0.55 倍の幅と見る）
        let width: f64 = text
            .chars()
            .map(|letter| if letter.is_ascii() { 0.55 } else { 1.0 })
            .sum();
        if width * size > LEGEND_WIDTH {
            set(&legend, "textLength", LEGEND_WIDTH)?;
            legend.set_attribute("lengthAdjust", "spacingAndGlyphs")?;
        }
        legend.set_text_content(Some(text));
        group.append_child(&legend)?;
    }
    Ok(group)
}

/// 数の属性を付ける。
fn set(element: &Element, name: &str, value: f64) -> Result<(), JsValue> {
    element.set_attribute(name, &value.to_string())
}
