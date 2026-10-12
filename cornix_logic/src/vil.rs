//! .vil（Vial の保存ファイル）の読み取り。
//!
//! 決まりは `docs/design.md` の B-09（読める形）・B-10（知らせの数）・3.6節（エラー）。
//! `serde_json::Value` で受けて、形は自分で確かめる（エラーに、場所を入れるため）。

use std::fmt;

use serde_json::Value;

use crate::keycode::Keycode;

/// 1レイヤーの行の数。
pub const ROWS: usize = 8;
/// 1行の列の数。
pub const COLS: usize = 7;

/// その位置に、キー（か丸い部品）があるか。50の位置で真になる。
///
/// 列6にあるのは、丸い部品の (2,6) と (5,6) だけ。
#[must_use]
pub const fn has_key(row: usize, col: usize) -> bool {
    row < ROWS && (col < 6 || is_knob(row, col))
}

/// 丸い部品の位置（(2,6) と (5,6)）か。
#[must_use]
pub const fn is_knob(row: usize, col: usize) -> bool {
    col == 6 && (row == 2 || row == 5)
}

/// .vil が読めないときのエラー（3.6節）。レイヤー・行・列の番号は、0 から数える。
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VilError {
    /// JSON として読めない（空の入力を含む）。
    NotJson,
    /// いちばん外側が、オブジェクトでない。
    NotObject,
    /// `layout` が無い。
    NoLayout,
    /// `layout` が、配列でない。
    LayoutNotArray,
    /// `layout` が、空の配列。
    NoLayers,
    /// レイヤーが、配列でない。
    LayerNotArray {
        /// レイヤーの番号。
        layer: usize,
    },
    /// レイヤーの行の数が、8 でない。
    BadRowCount {
        /// レイヤーの番号。
        layer: usize,
        /// 見つかった行の数。
        found: usize,
    },
    /// 行が、配列でない。
    RowNotArray {
        /// レイヤーの番号。
        layer: usize,
        /// 行の番号。
        row: usize,
    },
    /// 行の列の数が、7 でない。
    BadColCount {
        /// レイヤーの番号。
        layer: usize,
        /// 行の番号。
        row: usize,
        /// 見つかった列の数。
        found: usize,
    },
    /// マスの値が、文字列でも整数でもない。
    BadValue {
        /// レイヤーの番号。
        layer: usize,
        /// 行の番号。
        row: usize,
        /// 列の番号。
        col: usize,
    },
}

impl fmt::Display for VilError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NotJson => write!(f, "JSON として読めません"),
            Self::NotObject => write!(f, "いちばん外側がオブジェクトではありません"),
            Self::NoLayout => write!(f, "layout がありません"),
            Self::LayoutNotArray => write!(f, "layout が配列ではありません"),
            Self::NoLayers => write!(f, "レイヤーが1つもありません"),
            Self::LayerNotArray { layer } => {
                write!(f, "レイヤー {layer} が配列ではありません")
            }
            Self::BadRowCount { layer, found } => write!(
                f,
                "レイヤー {layer} の行の数が 8 ではありません（{found} 行）"
            ),
            Self::RowNotArray { layer, row } => {
                write!(f, "レイヤー {layer} の行 {row} が配列ではありません")
            }
            Self::BadColCount { layer, row, found } => write!(
                f,
                "レイヤー {layer} の行 {row} の列の数が 7 ではありません（{found} 列）"
            ),
            Self::BadValue { layer, row, col } => write!(
                f,
                "レイヤー {layer} の ({row},{col}) の値が、文字列でも整数でもありません"
            ),
        }
    }
}

impl std::error::Error for VilError {}

/// 読み込んだときの知らせの数（B-10）。
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct Notices {
    /// 48キーの位置で、未対応になったマスの数（全レイヤー。丸い部品は数えない）。
    pub unsupported_keys: usize,
    /// コンボの数。
    pub combos: usize,
    /// タップダンスの数。
    pub tap_dances: usize,
    /// マクロの数。
    pub macros: usize,
}

impl Notices {
    /// 読み込んだときの知らせの文（B-07）。0 個の物は省く。全部 0 なら `None`。
    ///
    /// ```rust
    /// use cornix_logic::vil::Notices;
    ///
    /// let notices = Notices {
    ///     unsupported_keys: 6,
    ///     ..Notices::default()
    /// };
    /// assert_eq!(
    ///     notices.message().as_deref(),
    ///     Some("動かない物: 未対応のキー 6 個")
    /// );
    /// assert_eq!(Notices::default().message(), None);
    /// ```
    #[must_use]
    pub fn message(&self) -> Option<String> {
        let parts: Vec<String> = [
            ("未対応のキー", self.unsupported_keys),
            ("コンボ", self.combos),
            ("タップダンス", self.tap_dances),
            ("マクロ", self.macros),
        ]
        .into_iter()
        .filter(|(_, count)| *count > 0)
        .map(|(name, count)| format!("{name} {count} 個"))
        .collect();
        (!parts.is_empty()).then(|| format!("動かない物: {}", parts.join("、")))
    }
}

/// 読み込んだキーマップ（レイヤー × 8 × 7 のキーコードと、知らせの数）。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Keymap {
    layers: Vec<Vec<Vec<Keycode>>>,
    notices: Notices,
}

impl Keymap {
    /// レイヤーの数（1以上）。
    #[must_use]
    pub const fn layer_count(&self) -> usize {
        self.layers.len()
    }

    /// その位置のキーコード。レイヤー・行・列が範囲の外なら `None`。
    ///
    /// キーの無い6つの位置は、中身を見ないので、いつも [`Keycode::No`]。
    #[must_use]
    pub fn key(&self, layer: usize, row: usize, col: usize) -> Option<&Keycode> {
        self.layers.get(layer)?.get(row)?.get(col)
    }

    /// 知らせの数。
    #[must_use]
    pub const fn notices(&self) -> Notices {
        self.notices
    }

    /// 読めたときの案内の文（B-07）。`file_name` は、選んだファイルの名前。
    #[must_use]
    pub fn loaded_message(&self, file_name: &str) -> String {
        format!(
            "読み込みました: {file_name}（レイヤー {} 個）",
            self.layer_count()
        )
    }
}

impl VilError {
    /// 読めないときの案内の文（B-08）。
    #[must_use]
    pub fn failed_message(&self) -> String {
        format!("読めませんでした: {self}")
    }
}

/// .vil の中身（文字列）を読む。
///
/// 先頭の BOM と、前後の空白・改行は許す。`layout` のほかの項目は、あっても無くてもよい。
///
/// ```rust
/// use cornix_logic::keycode::Keycode;
/// use cornix_logic::{sample, vil};
///
/// let keymap = vil::parse(sample::SAMPLE_VIL)?;
/// assert_eq!(keymap.layer_count(), 10);
/// assert_eq!(keymap.key(0, 3, 3), Some(&Keycode::Mo(1)));
/// # Ok::<(), vil::VilError>(())
/// ```
///
/// # Errors
///
/// 形が B-09 に合わないとき。最初に見つけた1つを返す（レイヤー・行・列の小さい順）。
pub fn parse(text: &str) -> Result<Keymap, VilError> {
    let text = text.strip_prefix('\u{feff}').unwrap_or(text);
    let value: Value = serde_json::from_str(text).map_err(|_| VilError::NotJson)?;
    let object = value.as_object().ok_or(VilError::NotObject)?;
    let layout = object
        .get("layout")
        .ok_or(VilError::NoLayout)?
        .as_array()
        .ok_or(VilError::LayoutNotArray)?;
    if layout.is_empty() {
        return Err(VilError::NoLayers);
    }
    let layers = layout
        .iter()
        .enumerate()
        .map(|(layer, value)| parse_layer(layer, value))
        .collect::<Result<Vec<_>, _>>()?;
    let unsupported_keys = layers
        .iter()
        .flat_map(|rows| rows.iter().enumerate())
        .flat_map(|(row, cols)| {
            cols.iter()
                .enumerate()
                .map(move |(col, key)| (row, col, key))
        })
        .filter(|(row, col, key)| {
            has_key(*row, *col) && !is_knob(*row, *col) && matches!(key, Keycode::Unsupported(_))
        })
        .count();
    let notices = Notices {
        unsupported_keys,
        combos: count_named(object.get("combo")),
        tap_dances: count_named(object.get("tap_dance")),
        macros: object
            .get("macro")
            .and_then(Value::as_array)
            .map_or(0, |items| {
                items
                    .iter()
                    .filter(|item| item.as_array().is_some_and(|steps| !steps.is_empty()))
                    .count()
            }),
    };
    Ok(Keymap { layers, notices })
}

/// 1レイヤー（8行 × 7列）を読む。
fn parse_layer(layer: usize, value: &Value) -> Result<Vec<Vec<Keycode>>, VilError> {
    let rows = value.as_array().ok_or(VilError::LayerNotArray { layer })?;
    if rows.len() != ROWS {
        return Err(VilError::BadRowCount {
            layer,
            found: rows.len(),
        });
    }
    rows.iter()
        .enumerate()
        .map(|(row, value)| {
            let cols = value
                .as_array()
                .ok_or(VilError::RowNotArray { layer, row })?;
            if cols.len() != COLS {
                return Err(VilError::BadColCount {
                    layer,
                    row,
                    found: cols.len(),
                });
            }
            cols.iter()
                .enumerate()
                .map(|(col, value)| {
                    let key = parse_cell(value).ok_or(VilError::BadValue { layer, row, col })?;
                    // キーの無い位置は、値の種類だけ確かめて、中身は見ない
                    Ok(if has_key(row, col) { key } else { Keycode::No })
                })
                .collect()
        })
        .collect()
}

/// マスの値（文字列か整数）を、キーコードにする。どちらでもなければ `None`。
fn parse_cell(value: &Value) -> Option<Keycode> {
    match value {
        Value::String(text) => Some(Keycode::parse(text)),
        Value::Number(number) if number.is_i64() || number.is_u64() => {
            Some(number.as_i64().map_or_else(
                || Keycode::Unsupported(number.to_string()),
                Keycode::from_int,
            ))
        }
        _ => None,
    }
}

/// コンボとタップダンスの数え方: 要素（配列）のうち、文字列の値に "`KC_NO`" でない物がある物の数。
fn count_named(value: Option<&Value>) -> usize {
    value.and_then(Value::as_array).map_or(0, |items| {
        items
            .iter()
            .filter(|item| {
                item.as_array().is_some_and(|values| {
                    values
                        .iter()
                        .any(|value| value.as_str().is_some_and(|text| text != "KC_NO"))
                })
            })
            .count()
    })
}

#[cfg(test)]
mod tests {
    use super::{Keymap, Notices, VilError, parse};
    use crate::keycode::{CharKey, EditKey, Keycode, Modifier};
    use crate::sample::SAMPLE_VIL;
    use crate::testkit::VilBuilder;

    fn key(keymap: &Keymap, layer: usize, row: usize, col: usize) -> Result<Keycode, String> {
        keymap
            .key(layer, row, col)
            .cloned()
            .ok_or_else(|| format!("レイヤー {layer} の ({row},{col}) がありません"))
    }

    fn named(
        keymap: &Keymap,
        layer: usize,
        row: usize,
        col: usize,
    ) -> Result<&'static str, String> {
        match key(keymap, layer, row, col)? {
            Keycode::Named(named) => Ok(named.label()),
            other => Err(format!("名前だけのキーでない: {other:?}")),
        }
    }

    fn unsupported(text: &str) -> Keycode {
        Keycode::Unsupported(text.to_owned())
    }

    #[test]
    fn item3_sample_is_read_as_written_in_design() -> Result<(), Box<dyn std::error::Error>> {
        let keymap = parse(SAMPLE_VIL)?;

        assert_eq!(keymap.layer_count(), 10);
        // レイヤー0
        assert_eq!(key(&keymap, 0, 0, 0)?, Keycode::Edit(EditKey::Tab));
        assert_eq!(key(&keymap, 0, 3, 3)?, Keycode::Mo(1));
        assert_eq!(key(&keymap, 0, 7, 3)?, Keycode::Mo(2));
        assert_eq!(key(&keymap, 0, 7, 5)?, Keycode::Edit(EditKey::Enter));
        // レイヤー1
        assert_eq!(key(&keymap, 1, 7, 0)?, Keycode::No);
        assert_eq!(key(&keymap, 1, 7, 3)?, Keycode::Transparent);
        assert_eq!(key(&keymap, 1, 5, 1)?, Keycode::Char(CharKey::Jyen));
        // レイヤー2
        assert!(matches!(
            key(&keymap, 2, 6, 0)?,
            Keycode::Shifted { base: crate::keycode::BaseKey::Char(CharKey::Ro), source } if source == "LSFT(KC_RO)"
        ));
        assert!(matches!(
            key(&keymap, 2, 6, 1)?,
            Keycode::Shifted { base: crate::keycode::BaseKey::Char(CharKey::Minus), source } if source == "KC_UNDS"
        ));
        assert_eq!(key(&keymap, 2, 4, 1)?, Keycode::No);
        // いちばん上のレイヤー
        assert_eq!(key(&keymap, 9, 0, 0)?, Keycode::Transparent);
        assert_eq!(keymap.notices(), Notices::default());
        // 範囲の外
        assert_eq!(keymap.key(10, 0, 0), None);
        assert_eq!(keymap.key(0, 8, 0), None);
        assert_eq!(keymap.key(0, 0, 7), None);
        Ok(())
    }

    #[test]
    fn item3_bom_crlf_and_spaces_do_not_change_the_result() -> Result<(), VilError> {
        let plain = parse(SAMPLE_VIL)?;

        assert_eq!(parse(&format!("{SAMPLE_VIL}\r\n"))?, plain);
        assert_eq!(parse(&format!("\u{feff}{SAMPLE_VIL}"))?, plain);
        assert_eq!(parse(&format!("  \n\t{SAMPLE_VIL} \n "))?, plain);
        assert_eq!(parse(&format!("\u{feff} {SAMPLE_VIL}\r\n"))?, plain);
        Ok(())
    }

    #[test]
    fn item3_layer_counts_1_4_and_11() -> Result<(), VilError> {
        for layers in [1, 4, 11] {
            let keymap = parse(&VilBuilder::new(layers).build())?;
            assert_eq!(keymap.layer_count(), layers);
            assert_eq!(keymap.key(layers, 0, 0), None);
            assert_eq!(keymap.key(layers - 1, 7, 5), Some(&Keycode::Transparent));
        }
        Ok(())
    }

    #[test]
    fn item3_other_fields_are_not_looked_at() -> Result<(), VilError> {
        let expected = parse(&VilBuilder::new(2).key(0, 0, 1, "KC_Q").build())?;
        let variants = [
            VilBuilder::new(2).top("uid", "0"),
            VilBuilder::new(2).without("uid"),
            VilBuilder::new(2).top("uid", "\"abc\""),
            VilBuilder::new(2).top("version", "2"),
            VilBuilder::new(2).without("uid").without("version"),
        ];
        for builder in variants {
            let text = builder.key(0, 0, 1, "KC_Q").build();
            assert_eq!(parse(&text)?, expected, "{text}");
        }
        // layout だけの .vil
        let only_layout = VilBuilder::new(2).without("uid").without("version").build();
        assert!(only_layout.starts_with("{\"layout\":"));
        assert_eq!(parse(&only_layout)?.layer_count(), 2);
        Ok(())
    }

    #[test]
    fn item3_integers_at_key_positions() -> Result<(), Box<dyn std::error::Error>> {
        let text = VilBuilder::new(1)
            .raw(0, 0, 0, "-1")
            .raw(0, 0, 1, "5")
            .raw(0, 7, 5, "65535")
            .build();
        let keymap = parse(&text)?;

        assert_eq!(key(&keymap, 0, 0, 0)?, Keycode::No);
        assert_eq!(key(&keymap, 0, 0, 1)?, unsupported("5"));
        assert_eq!(key(&keymap, 0, 7, 5)?, unsupported("65535"));
        Ok(())
    }

    #[test]
    fn item3_positions_without_keys_are_not_looked_at() -> Result<(), Box<dyn std::error::Error>> {
        let mut builder = VilBuilder::new(1);
        for (row, raw) in [
            (0, "\"KC_A\""),
            (1, "0"),
            (3, "\"USER00\""),
            (4, "\"\""),
            (6, "5"),
            (7, "\"KC_A\""),
        ] {
            builder = builder.raw(0, row, 6, raw);
        }
        let keymap = parse(&builder.build())?;

        for row in [0, 1, 3, 4, 6, 7] {
            assert_eq!(key(&keymap, 0, row, 6)?, Keycode::No, "行 {row}");
        }
        // 中身を見ないので、未対応の数にも入らない
        assert_eq!(keymap.notices().unsupported_keys, 0);
        Ok(())
    }

    #[test]
    fn item3_unknown_keycodes_become_unsupported() -> Result<(), Box<dyn std::error::Error>> {
        let text = VilBuilder::new(1)
            .key(0, 0, 0, "USER00")
            .key(0, 1, 0, "LT1(KC_SPACE)")
            .key(0, 2, 0, "0x7c79")
            .build();
        let keymap = parse(&text)?;

        assert_eq!(key(&keymap, 0, 0, 0)?, unsupported("USER00"));
        assert_eq!(key(&keymap, 0, 1, 0)?, unsupported("LT1(KC_SPACE)"));
        assert_eq!(key(&keymap, 0, 2, 0)?, unsupported("0x7c79"));
        Ok(())
    }

    #[test]
    fn item3_official_keymap_if_present() -> Result<(), Box<dyn std::error::Error>> {
        let path = concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../local/cornix-default-keymap.vil"
        );
        let Ok(text) = std::fs::read_to_string(path) else {
            return Ok(()); // 公式の .vil は git の管理の外。無ければ、何もしない
        };
        let keymap = parse(&text)?;

        assert_eq!(keymap.layer_count(), 10);
        assert_eq!(key(&keymap, 0, 3, 3)?, Keycode::Mo(1));
        assert_eq!(key(&keymap, 0, 2, 0)?, Keycode::Modifier(Modifier::LShift));
        assert_eq!(named(&keymap, 0, 1, 0)?, "Caps");
        assert_eq!(key(&keymap, 2, 1, 0)?, unsupported("USER00"));
        assert_eq!(
            keymap.notices(),
            Notices {
                unsupported_keys: 6,
                combos: 0,
                tap_dances: 0,
                macros: 0
            }
        );
        Ok(())
    }
}
