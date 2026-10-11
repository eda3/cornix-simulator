//! 図に出す中身。位置ごとの刻印・濃さ・押されているか・手元の刻印と、有効なレイヤーの番号を作る。
//!
//! 決まりは `docs/design.md` の B-12〜B-19。DOM には触らない。

use crate::chars::{Layout, char_of, name_of};
use crate::engine::{Decided, Engine};
use crate::geometry::SHAPES;
use crate::hostmap::label_of;
use crate::keycode::{BaseKey, CharKey, Keycode};

/// 表示するレイヤーの選び方（B-18）。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Shown {
    /// 自動（いま押したら決まるキーコードを出す。B-13）。
    Auto,
    /// 番号で選んだレイヤーの中身を、そのまま出す。
    Layer(usize),
}

/// 刻印の濃さ。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Tone {
    /// ふつう。
    Normal,
    /// 薄い（`KC_TRNS` を通って、下のレイヤーから来た。B-14）。
    Dim,
    /// 未対応（小さく薄く）。
    Unsupported,
    /// 空（刻印が無い）。
    Empty,
}

/// 1つの位置の、図に出す中身。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Cell {
    /// 位置（行, 列）。
    pub position: (usize, usize),
    /// 下の文字（刻印が1つだけのときは、ここ）。
    pub lower: String,
    /// 上の文字（Shift 側。無ければ空）。
    pub upper: String,
    /// 濃さ。
    pub tone: Tone,
    /// 押されているか（B-17）。
    pub pressed: bool,
    /// 手元の刻印（B-16）。出さない設定のときと、丸い部品は `None`。
    pub host_label: Option<String>,
}

/// 図の全体。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct View {
    /// 50の位置の中身（`geometry::SHAPES` と同じ順）。
    pub cells: Vec<Cell>,
    /// 有効なレイヤーの番号（小さい順。B-19）。
    pub active_layers: Vec<usize>,
}

impl View {
    /// 位置の中身。キーの無い位置は `None`。
    #[must_use]
    pub fn cell(&self, row: usize, col: usize) -> Option<&Cell> {
        self.cells.iter().find(|cell| cell.position == (row, col))
    }
}

/// 図に出す中身を作る。
#[must_use]
pub fn render(engine: &Engine, layout: Layout, shown: Shown, host_labels: bool) -> View {
    let active_layers = engine.active_layers();
    let top = active_layers.last().copied().unwrap_or(0);
    let cells = SHAPES
        .iter()
        .map(|shape| {
            let (row, col) = shape.position;
            if shape.knob {
                // 丸い部品は、何が割り当ててあっても「未対応」で、押せない（B-12）
                return Cell {
                    position: shape.position,
                    lower: "未対応".to_owned(),
                    upper: String::new(),
                    tone: Tone::Unsupported,
                    pressed: false,
                    host_label: None,
                };
            }
            let held = engine.held(row, col);
            let (legend, tone) = match shown {
                Shown::Auto => auto_legend(engine, held, (row, col), top, layout),
                Shown::Layer(layer) => engine.keymap().key(layer, row, col).map_or_else(
                    || (Legend::default(), Tone::Empty),
                    |keycode| legend_of(keycode, layout, "▽"),
                ),
            };
            Cell {
                position: shape.position,
                lower: legend.lower,
                upper: legend.upper,
                tone,
                pressed: held.is_some(),
                host_label: host_labels
                    .then(|| label_of(row, col).unwrap_or_else(|| "手元のキー無し".to_owned())),
            }
        })
        .collect();
    View {
        cells,
        active_layers,
    }
}

/// 刻印（下と上）。
#[derive(Debug, Default)]
struct Legend {
    lower: String,
    upper: String,
}

impl Legend {
    fn one(text: &str) -> Self {
        Self {
            lower: text.to_owned(),
            upper: String::new(),
        }
    }
}

/// 「自動」のときの刻印と濃さ（B-13・B-14）。
fn auto_legend(
    engine: &Engine,
    held: Option<&Decided>,
    position: (usize, usize),
    top: usize,
    layout: Layout,
) -> (Legend, Tone) {
    // 押されているキーは、押したときに決まったキーコードを、ふつうの濃さで出す
    let pressed = held.is_some();
    let decided = held
        .cloned()
        .unwrap_or_else(|| engine.resolve(position.0, position.1));
    match &decided {
        Decided::Unassigned => (Legend::default(), Tone::Empty),
        Decided::Key { keycode, layer } => {
            let (legend, tone) = legend_of(keycode, layout, "");
            let tone = if tone == Tone::Normal && !pressed && *layer != top {
                Tone::Dim
            } else {
                tone
            };
            (legend, tone)
        }
    }
}

/// キーコードごとの刻印（B-15）。`transparent` は、`KC_TRNS` のときに出す文字。
fn legend_of(keycode: &Keycode, layout: Layout, transparent: &str) -> (Legend, Tone) {
    let legend = match keycode {
        Keycode::No => return (Legend::default(), Tone::Empty),
        Keycode::Transparent if transparent.is_empty() => {
            return (Legend::default(), Tone::Empty);
        }
        Keycode::Transparent => Legend::one(transparent),
        Keycode::Unsupported(text) => return (Legend::one(text), Tone::Unsupported),
        Keycode::Char(key) => char_legend(*key, layout),
        Keycode::Edit(key) => Legend::one(key.label()),
        Keycode::Named(key) => Legend::one(key.label()),
        Keycode::Modifier(modifier) => Legend::one(modifier.label()),
        Keycode::Mo(layer) => Legend::one(&format!("MO({layer})")),
        Keycode::Shifted { base, source } => match base {
            // Shift 側の文字1つ。「なし」なら、キーコードの文字列
            BaseKey::Char(key) => char_of(*key, true, layout).map_or_else(
                || Legend::one(source),
                |typed| Legend::one(&typed.to_string()),
            ),
            BaseKey::Edit(_) | BaseKey::Named(_) => Legend::one(source),
        },
    };
    (legend, Tone::Normal)
}

/// 文字のキーの刻印: 下に素の文字、上に Shift 側の文字。英字は、大文字1つだけ。
fn char_legend(key: CharKey, layout: Layout) -> Legend {
    if let CharKey::Letter(letter) = key {
        return Legend::one(&letter.to_ascii_uppercase().to_string());
    }
    char_of(key, false, layout).map_or_else(
        || Legend::one(name_of(key, layout).unwrap_or_default()),
        |plain| Legend {
            lower: plain.to_string(),
            upper: char_of(key, true, layout).map_or_else(String::new, |typed| typed.to_string()),
        },
    )
}
