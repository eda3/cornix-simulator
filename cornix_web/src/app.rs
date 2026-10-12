//! シミュレータ本体（`static/index.html`）の、DOM との受け渡し。wasm32 のときだけ。
//!
//! 図に出す中身を決める判断は `cornix_logic` に置き、ここは、それを描くだけにする。
//! 振る舞いは `docs/design.md` の B-01〜B-04・B-11・B-12。

use cornix_logic::chars::Layout;
use cornix_logic::engine::Engine;
use cornix_logic::geometry::{KeyShape, SHAPES};
use cornix_logic::sample::SAMPLE_VIL;
use cornix_logic::view::{Cell, Shown, Tone, View, render};
use cornix_logic::vil::parse;
use wasm_bindgen::prelude::*;
use web_sys::{Document, Element};

const SVG_NS: &str = "http://www.w3.org/2000/svg";
/// キー1つ分の間隔（図の中の長さ）。
const UNIT: f64 = 60.0;
/// キーの四角の一辺（丸い部品の直径）。間隔より少し小さくして、すき間を空ける。
const SIDE: f64 = 56.0;
/// 刻印に使える幅。これより長い刻印は、幅に合わせて詰める。
const LEGEND_WIDTH: f64 = 50.0;

/// シミュレータを動かす。`static/index.html` が、読み込みのあとに1回だけ呼ぶ。
///
/// 最初は、見本のキーマップ・JIS・「自動」・手元のキーの名前を出す、で図を描く（B-01・B-04）。
///
/// # Errors
///
/// ページに要る部品（id の付いた要素）が見つからないときと、見本のキーマップが読めないとき。
#[wasm_bindgen]
pub fn run_simulator() -> Result<(), JsValue> {
    let document = web_sys::window()
        .and_then(|window| window.document())
        .ok_or_else(|| JsValue::from_str("document がありません"))?;
    let figure = document
        .get_element_by_id("figure")
        .ok_or_else(|| JsValue::from_str("index.html に #figure がありません"))?;
    let keymap = parse(SAMPLE_VIL).map_err(|error| JsValue::from_str(&error.to_string()))?;
    let engine = Engine::new(keymap);
    draw(
        &document,
        &figure,
        &render(&engine, Layout::Jis, Shown::Auto, true),
    )
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
