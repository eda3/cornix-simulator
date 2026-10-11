//! 計測ページ（keytest）の、DOM とイベントの配線。wasm32 のときだけ。
//!
//! 表を作る判断は [`crate::keylog`] に置き、ここは DOM との受け渡しだけをする。
//! 振る舞いは `docs/design.md` の B-46〜B-52。

use std::cell::RefCell;
use std::rc::Rc;

use js_sys::{Function, Promise, Reflect};
use wasm_bindgen::convert::FromWasmAbi;
use wasm_bindgen::prelude::*;
use web_sys::{
    Document, Element, Event, EventTarget, HtmlElement, HtmlInputElement, KeyboardEvent, Window,
};

use crate::keylog::{EventKind, KeyEvent, KeyLog, SummaryRow};

/// 全画面でないときの、ボタンの文。
const ENTER_LABEL: &str = "全画面＋Keyboard Lock";
/// 全画面のときの、ボタンの文（B-42）。
const EXIT_LABEL: &str = "全画面をやめる";
/// Keyboard Lock の様子として、ページに出す文（B-50）。
const LOCK_OFF: &str = "Keyboard Lock は掛けていません";
const LOCK_ON: &str = "Keyboard Lock が掛かりました";
const LOCK_REFUSED: &str = "Keyboard Lock を掛けられませんでした";
const LOCK_MISSING: &str =
    "Keyboard Lock を掛けられませんでした（このブラウザには navigator.keyboard がありません）";

/// ページの中の、読み書きする部品。
struct Page {
    window: Window,
    document: Document,
    /// 全画面にする要素（ページの全体）。
    root: Element,
    /// 「preventDefault する」のチェック。
    prevent_default: HtmlInputElement,
    /// 「全画面＋Keyboard Lock」のボタン。
    fullscreen: HtmlElement,
    /// 「記録を消す」のボタン。
    clear: HtmlElement,
    /// Keyboard Lock の様子を出す所。
    lock_status: Element,
    /// まとめの表の本体。
    summary_body: Element,
    /// 直近のイベントの表の本体。
    recent_body: Element,
    /// Markdown の表を出す所。
    markdown: Element,
}

impl Page {
    /// `keytest.html` の中から、部品を探す。
    fn find(window: Window) -> Result<Self, JsValue> {
        let document = window
            .document()
            .ok_or_else(|| JsValue::from_str("document がありません"))?;
        let root = document
            .document_element()
            .ok_or_else(|| JsValue::from_str("ページの全体の要素がありません"))?;
        Ok(Self {
            root,
            prevent_default: element(&document, "prevent-default")?,
            fullscreen: element(&document, "fullscreen")?,
            clear: element(&document, "clear")?,
            lock_status: element(&document, "lock-status")?,
            summary_body: element(&document, "summary-body")?,
            recent_body: element(&document, "recent-body")?,
            markdown: element(&document, "markdown")?,
            window,
            document,
        })
    }

    /// まとめの表・直近のイベント・Markdown の表を、記録のとおりに書き直す（B-47・B-48・B-52）。
    fn render(&self, log: &KeyLog) -> Result<(), JsValue> {
        self.fill(&self.summary_body, log.rows().iter().map(SummaryRow::cells))?;
        self.fill(&self.recent_body, log.recent().map(KeyEvent::cells))?;
        self.markdown.set_text_content(Some(&log.to_markdown()));
        Ok(())
    }

    /// 表の本体を、渡した行で作り直す。
    fn fill<const N: usize>(
        &self,
        body: &Element,
        rows: impl Iterator<Item = [String; N]>,
    ) -> Result<(), JsValue> {
        body.set_text_content(None);
        for cells in rows {
            let tr = self.document.create_element("tr")?;
            for cell in cells {
                let td = self.document.create_element("td")?;
                td.set_text_content(Some(&cell));
                tr.append_child(&td)?;
            }
            body.append_child(&tr)?;
        }
        Ok(())
    }

    /// ページが（JavaScript から頼んだ）全画面になっているか。
    fn is_fullscreen(&self) -> bool {
        self.document.fullscreen_element().is_some()
    }

    /// `navigator.keyboard`。Keyboard Lock の無いブラウザ（値が undefined か null）では `None`。
    ///
    /// web-sys に Keyboard Lock の型が無いので、`js_sys::Reflect` で読む。
    fn keyboard(&self) -> Result<Option<JsValue>, JsValue> {
        let keyboard = Reflect::get(&self.window.navigator(), &JsValue::from_str("keyboard"))?;
        Ok((!keyboard.is_undefined() && !keyboard.is_null()).then_some(keyboard))
    }

    /// 全画面でないときの表示にする。
    fn show_windowed(&self) {
        self.fullscreen.set_text_content(Some(ENTER_LABEL));
        self.lock_status.set_text_content(Some(LOCK_OFF));
    }
}

/// 計測ページを動かす。`static/keytest.html` が、読み込みのあとに1回だけ呼ぶ。
///
/// # Errors
///
/// ページに要る部品（id の付いた要素）が見つからないときと、ブラウザがイベントの登録を断ったとき。
#[wasm_bindgen]
pub fn run_keytest() -> Result<(), JsValue> {
    let window = web_sys::window().ok_or_else(|| JsValue::from_str("window がありません"))?;
    let page = Rc::new(Page::find(window)?);
    let log = Rc::new(RefCell::new(KeyLog::default()));
    page.show_windowed();
    page.render(&log.borrow())?;

    // 届いた keydown と keyup を、全部記録する（B-47〜B-49）
    for kind in [EventKind::KeyDown, EventKind::KeyUp] {
        let (shown, shared) = (Rc::clone(&page), Rc::clone(&log));
        listen(&page.window, kind.name(), move |event: KeyboardEvent| {
            if shown.prevent_default.checked() {
                event.prevent_default();
            }
            shared.borrow_mut().record(KeyEvent {
                kind,
                code: event.code(),
                key: event.key(),
                location: event.location(),
                repeat: event.repeat(),
            });
            shown.render(&shared.borrow())
        })?;
    }

    // 「記録を消す」（B-51）。押したあとは、フォーカスを外す（手元の Space や Enter で、また押されないように）
    let (shown, shared) = (Rc::clone(&page), Rc::clone(&log));
    listen(&page.clear, "click", move |_: Event| {
        shared.borrow_mut().clear();
        shown.render(&shared.borrow())?;
        shown.clear.blur()
    })?;

    // 「preventDefault する」を切り替えたあとも、フォーカスを外す（手元の Space で、また切り替わらないように）
    let shown = Rc::clone(&page);
    listen(&page.prevent_default, "change", move |_: Event| {
        shown.prevent_default.blur()
    })?;

    // 「全画面＋Keyboard Lock」（B-50。動きは B-42 と同じ）。押すたびに、全画面に入る・抜ける
    let shown = Rc::clone(&page);
    listen(&page.fullscreen, "click", move |_: Event| {
        if shown.is_fullscreen() {
            shown.document.exit_fullscreen();
        } else {
            shown.root.request_fullscreen()?;
        }
        shown.fullscreen.blur()
    })?;

    // lock() の結果を受ける関数。許可を待つ間に全画面を抜けていたら、掛かっていないので、文は変えない
    let shown = Rc::clone(&page);
    let on_locked = Closure::<dyn FnMut(JsValue)>::new(move |_: JsValue| {
        if shown.is_fullscreen() {
            shown.lock_status.set_text_content(Some(LOCK_ON));
        }
    });
    let shown = Rc::clone(&page);
    let on_refused = Closure::<dyn FnMut(JsValue)>::new(move |reason: JsValue| {
        let text = format!("{LOCK_REFUSED}（{}）", describe(&reason));
        shown.lock_status.set_text_content(Some(&text));
    });

    // 全画面に入ったら Keyboard Lock を掛け、抜けたら外す（Esc の長押しなど、どの抜け方でも、ここに来る）
    let shown = Rc::clone(&page);
    listen(&page.document, "fullscreenchange", move |_: Event| {
        let keyboard = shown.keyboard()?;
        if shown.is_fullscreen() {
            shown.fullscreen.set_text_content(Some(EXIT_LABEL));
            if let Some(keyboard) = keyboard {
                let promise: Promise = call(&keyboard, "lock")?.dyn_into()?;
                // then2 が返す Promise は使わない（結果は、2つの関数が受ける）
                let _ = promise.then2(&on_locked, &on_refused);
            } else {
                shown.lock_status.set_text_content(Some(LOCK_MISSING));
            }
        } else {
            if let Some(keyboard) = keyboard {
                call(&keyboard, "unlock")?;
            }
            shown.show_windowed();
        }
        Ok(())
    })
}

/// id で要素を取る。無いとき・種類が違うときは、id を入れた文のエラーにする。
fn element<T: JsCast>(document: &Document, id: &str) -> Result<T, JsValue> {
    document
        .get_element_by_id(id)
        .and_then(|element| element.dyn_into().ok())
        .ok_or_else(|| JsValue::from_str(&format!("keytest.html に #{id} がありません")))
}

/// イベントを受ける関数を登録する。関数は、ページが閉じるまで生かしておく。
fn listen<E>(
    target: &EventTarget,
    kind: &str,
    handler: impl FnMut(E) -> Result<(), JsValue> + 'static,
) -> Result<(), JsValue>
where
    E: FromWasmAbi + 'static,
{
    let closure = Closure::<dyn FnMut(E) -> Result<(), JsValue>>::new(handler);
    target.add_event_listener_with_callback(kind, closure.as_ref().unchecked_ref())?;
    closure.forget();
    Ok(())
}

/// JavaScript のオブジェクトのメソッドを、引数なしで呼ぶ。
///
/// web-sys に型の無い物（`navigator.keyboard` の `lock`・`unlock`）を呼ぶのに使う。
fn call(target: &JsValue, method: &str) -> Result<JsValue, JsValue> {
    let function: Function = Reflect::get(target, &JsValue::from_str(method))?.dyn_into()?;
    function.call0(target)
}

/// JavaScript の側から届いた、断られた理由を、文字にする。
fn describe(reason: &JsValue) -> String {
    reason.dyn_ref::<js_sys::Error>().map_or_else(
        || format!("{reason:?}"),
        |error| String::from(error.to_string()),
    )
}
