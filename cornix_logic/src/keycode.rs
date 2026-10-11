//! キーコード。.vil の文字列（か整数）を、種類に分ける。
//!
//! 表は `docs/design.md` の 3.1節から手で写した物。読めない入力は無く、表に無い物は
//! [`Keycode::Unsupported`] になって、元の文字列を保つ。

/// 文字のキー（3.1節）。出す文字は、配列で変わる（`chars` が決める）。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CharKey {
    /// `KC_A`〜`KC_Z`。中身は、小文字の `a`〜`z`。
    Letter(char),
    /// `KC_1`〜`KC_9` と `KC_0`。中身は、`0`〜`9`。
    Digit(char),
    /// `KC_MINUS`
    Minus,
    /// `KC_EQUAL`
    Equal,
    /// `KC_LBRACKET`
    LBracket,
    /// `KC_RBRACKET`
    RBracket,
    /// `KC_BSLASH`
    Bslash,
    /// `KC_NONUS_HASH`
    NonusHash,
    /// `KC_SCOLON`
    Scolon,
    /// `KC_QUOTE`
    Quote,
    /// `KC_GRAVE`
    Grave,
    /// `KC_COMMA`
    Comma,
    /// `KC_DOT`
    Dot,
    /// `KC_SLASH`
    Slash,
    /// `KC_RO`
    Ro,
    /// `KC_JYEN`
    Jyen,
}

/// 欄を変えるキー（3.1節）。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EditKey {
    /// `KC_ENTER`
    Enter,
    /// `KC_BSPACE`
    Bspace,
    /// `KC_TAB`
    Tab,
    /// `KC_SPACE`
    Space,
}

impl EditKey {
    /// 図に出す名前。
    #[must_use]
    pub const fn label(self) -> &'static str {
        match self {
            Self::Enter => "Enter",
            Self::Bspace => "Bksp",
            Self::Tab => "Tab",
            Self::Space => "Space",
        }
    }
}

/// 名前だけのキー（3.1節）。押すと光り、名前が出るだけで、欄は変えない。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NamedKey {
    label: &'static str,
}

impl NamedKey {
    /// 図に出す名前。
    #[must_use]
    pub const fn label(self) -> &'static str {
        self.label
    }
}

/// 修飾キー（3.1節の8つ）。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Modifier {
    /// `KC_LCTRL`
    LCtrl,
    /// `KC_LSHIFT`
    LShift,
    /// `KC_LALT`
    LAlt,
    /// `KC_LGUI`
    LGui,
    /// `KC_RCTRL`
    RCtrl,
    /// `KC_RSHIFT`
    RShift,
    /// `KC_RALT`
    RAlt,
    /// `KC_RGUI`
    RGui,
}

impl Modifier {
    /// 8つ全部。
    pub const ALL: [Self; 8] = [
        Self::LCtrl,
        Self::LShift,
        Self::LAlt,
        Self::LGui,
        Self::RCtrl,
        Self::RShift,
        Self::RAlt,
        Self::RGui,
    ];

    /// 図に出す名前。
    #[must_use]
    pub const fn label(self) -> &'static str {
        match self {
            Self::LCtrl => "LCtrl",
            Self::LShift => "LShift",
            Self::LAlt => "LAlt",
            Self::LGui => "LGUI",
            Self::RCtrl => "RCtrl",
            Self::RShift => "RShift",
            Self::RAlt => "RAlt",
            Self::RGui => "RGUI",
        }
    }
}

/// Shift 付きのキーコードの、中に入れられるキー（文字のキー・欄を変えるキー・名前だけのキー）。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BaseKey {
    /// 文字のキー。
    Char(CharKey),
    /// 欄を変えるキー。
    Edit(EditKey),
    /// 名前だけのキー。
    Named(NamedKey),
}

/// キーコード（種類と中身）。
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Keycode {
    /// 割り当てなし（`KC_NO`。キーのある位置の -1 も同じ）。
    No,
    /// 透明（`KC_TRNS`）。
    Transparent,
    /// 文字のキー。
    Char(CharKey),
    /// 欄を変えるキー。
    Edit(EditKey),
    /// 名前だけのキー。
    Named(NamedKey),
    /// 修飾キー。
    Modifier(Modifier),
    /// `MO(n)`。n は 0〜31。
    Mo(u8),
    /// Shift 付き。`source` は、.vil に書いてあった文字列。
    Shifted {
        /// 中のキー。
        base: BaseKey,
        /// 元の文字列（例: `KC_UNDS`・`LSFT(KC_RO)`）。
        source: String,
    },
    /// 未対応。元の文字列（-1 以外の整数は、10進の数字）を保つ。
    Unsupported(String),
}

/// 名前だけのキーの表（.vil の文字列と、図に出す名前）。
const NAMED: [(&str, &str); 33] = [
    ("KC_ESCAPE", "Esc"),
    ("KC_CAPSLOCK", "Caps"),
    ("KC_F1", "F1"),
    ("KC_F2", "F2"),
    ("KC_F3", "F3"),
    ("KC_F4", "F4"),
    ("KC_F5", "F5"),
    ("KC_F6", "F6"),
    ("KC_F7", "F7"),
    ("KC_F8", "F8"),
    ("KC_F9", "F9"),
    ("KC_F10", "F10"),
    ("KC_F11", "F11"),
    ("KC_F12", "F12"),
    ("KC_PSCREEN", "PrtSc"),
    ("KC_SCROLLLOCK", "ScrLk"),
    ("KC_PAUSE", "Pause"),
    ("KC_INSERT", "Ins"),
    ("KC_HOME", "Home"),
    ("KC_PGUP", "PgUp"),
    ("KC_DELETE", "Del"),
    ("KC_END", "End"),
    ("KC_PGDOWN", "PgDn"),
    ("KC_RIGHT", "→"),
    ("KC_LEFT", "←"),
    ("KC_DOWN", "↓"),
    ("KC_UP", "↑"),
    ("KC_APPLICATION", "Menu"),
    ("KC_KANA", "かな"),
    ("KC_HENK", "変換"),
    ("KC_MHEN", "無変換"),
    ("KC_LANG1", "LANG1"),
    ("KC_LANG2", "LANG2"),
];

/// Shift 付きで、名前が決まっている21個（名前と、中のキーの名前）。
const SHIFTED: [(&str, &str); 21] = [
    ("KC_TILD", "KC_GRAVE"),
    ("KC_EXLM", "KC_1"),
    ("KC_AT", "KC_2"),
    ("KC_HASH", "KC_3"),
    ("KC_DLR", "KC_4"),
    ("KC_PERC", "KC_5"),
    ("KC_CIRC", "KC_6"),
    ("KC_AMPR", "KC_7"),
    ("KC_ASTR", "KC_8"),
    ("KC_LPRN", "KC_9"),
    ("KC_RPRN", "KC_0"),
    ("KC_UNDS", "KC_MINUS"),
    ("KC_PLUS", "KC_EQUAL"),
    ("KC_LCBR", "KC_LBRACKET"),
    ("KC_RCBR", "KC_RBRACKET"),
    ("KC_PIPE", "KC_BSLASH"),
    ("KC_COLN", "KC_SCOLON"),
    ("KC_DQUO", "KC_QUOTE"),
    ("KC_LT", "KC_COMMA"),
    ("KC_GT", "KC_DOT"),
    ("KC_QUES", "KC_SLASH"),
];

impl Keycode {
    /// .vil の文字列を読む。表に無い文字列は、未対応になる（大文字と小文字は区別する）。
    ///
    /// ```rust
    /// use cornix_logic::keycode::Keycode;
    ///
    /// assert_eq!(Keycode::parse("MO(1)"), Keycode::Mo(1));
    /// assert_eq!(Keycode::parse("USER00"), Keycode::Unsupported("USER00".to_owned()));
    /// ```
    #[must_use]
    pub fn parse(text: &str) -> Self {
        if let Some(base) = parse_base(text) {
            return match base {
                BaseKey::Char(key) => Self::Char(key),
                BaseKey::Edit(key) => Self::Edit(key),
                BaseKey::Named(key) => Self::Named(key),
            };
        }
        let shifted_inner = SHIFTED
            .iter()
            .find(|(name, _)| *name == text)
            .map(|(_, inner)| *inner)
            .or_else(|| {
                text.strip_prefix("LSFT(")
                    .or_else(|| text.strip_prefix("RSFT("))
                    .and_then(|rest| rest.strip_suffix(')'))
            });
        if let Some(base) = shifted_inner.and_then(parse_base) {
            return Self::Shifted {
                base,
                source: text.to_owned(),
            };
        }
        match text {
            "KC_NO" => Self::No,
            "KC_TRNS" => Self::Transparent,
            "KC_LCTRL" => Self::Modifier(Modifier::LCtrl),
            "KC_LSHIFT" => Self::Modifier(Modifier::LShift),
            "KC_LALT" => Self::Modifier(Modifier::LAlt),
            "KC_LGUI" => Self::Modifier(Modifier::LGui),
            "KC_RCTRL" => Self::Modifier(Modifier::RCtrl),
            "KC_RSHIFT" => Self::Modifier(Modifier::RShift),
            "KC_RALT" => Self::Modifier(Modifier::RAlt),
            "KC_RGUI" => Self::Modifier(Modifier::RGui),
            _ => parse_mo(text).map_or_else(|| Self::Unsupported(text.to_owned()), Self::Mo),
        }
    }

    /// .vil の整数を読む。-1 は `KC_NO` と同じ。ほかは未対応で、10進の数字を保つ。
    #[must_use]
    pub fn from_int(value: i64) -> Self {
        if value == -1 {
            Self::No
        } else {
            Self::Unsupported(value.to_string())
        }
    }
}

/// 文字のキー・欄を変えるキー・名前だけのキーを読む。
fn parse_base(text: &str) -> Option<BaseKey> {
    let char_key = match text {
        "KC_MINUS" => Some(CharKey::Minus),
        "KC_EQUAL" => Some(CharKey::Equal),
        "KC_LBRACKET" => Some(CharKey::LBracket),
        "KC_RBRACKET" => Some(CharKey::RBracket),
        "KC_BSLASH" => Some(CharKey::Bslash),
        "KC_NONUS_HASH" => Some(CharKey::NonusHash),
        "KC_SCOLON" => Some(CharKey::Scolon),
        "KC_QUOTE" => Some(CharKey::Quote),
        "KC_GRAVE" => Some(CharKey::Grave),
        "KC_COMMA" => Some(CharKey::Comma),
        "KC_DOT" => Some(CharKey::Dot),
        "KC_SLASH" => Some(CharKey::Slash),
        "KC_RO" => Some(CharKey::Ro),
        "KC_JYEN" => Some(CharKey::Jyen),
        _ => single_char(text),
    };
    if let Some(key) = char_key {
        return Some(BaseKey::Char(key));
    }
    let edit = match text {
        "KC_ENTER" => Some(EditKey::Enter),
        "KC_BSPACE" => Some(EditKey::Bspace),
        "KC_TAB" => Some(EditKey::Tab),
        "KC_SPACE" => Some(EditKey::Space),
        _ => None,
    };
    if let Some(key) = edit {
        return Some(BaseKey::Edit(key));
    }
    NAMED
        .iter()
        .find(|(name, _)| *name == text)
        .map(|(_, label)| BaseKey::Named(NamedKey { label }))
}

/// `KC_A`〜`KC_Z` と `KC_0`〜`KC_9`。
fn single_char(text: &str) -> Option<CharKey> {
    let mut rest = text.strip_prefix("KC_")?.chars();
    let (first, None) = (rest.next()?, rest.next()) else {
        return None;
    };
    if first.is_ascii_uppercase() {
        Some(CharKey::Letter(first.to_ascii_lowercase()))
    } else if first.is_ascii_digit() {
        Some(CharKey::Digit(first))
    } else {
        None
    }
}

/// `MO(n)`。n は 0〜31 の10進で、先頭に余分な 0 を付けない。
fn parse_mo(text: &str) -> Option<u8> {
    let digits = text.strip_prefix("MO(")?.strip_suffix(')')?;
    let plain = !digits.is_empty()
        && digits.bytes().all(|byte| byte.is_ascii_digit())
        && (digits == "0" || !digits.starts_with('0'));
    let layer: u8 = if plain {
        digits.parse().ok()?
    } else {
        return None;
    };
    (layer <= 31).then_some(layer)
}

#[cfg(test)]
mod tests {
    use super::{BaseKey, CharKey, EditKey, Keycode, Modifier};

    fn unsupported(text: &str) -> Keycode {
        Keycode::Unsupported(text.to_owned())
    }

    fn named_label(text: &str) -> Result<&'static str, String> {
        match Keycode::parse(text) {
            Keycode::Named(key) => Ok(key.label()),
            other => Err(format!("{text} が、名前だけのキーにならない: {other:?}")),
        }
    }

    fn shifted_base(text: &str) -> Result<BaseKey, String> {
        match Keycode::parse(text) {
            Keycode::Shifted { base, source } if source == text => Ok(base),
            other => Err(format!("{text} が、Shift 付きにならない: {other:?}")),
        }
    }

    #[test]
    fn item2_no_and_transparent() {
        assert_eq!(Keycode::parse("KC_NO"), Keycode::No);
        assert_eq!(Keycode::parse("KC_TRNS"), Keycode::Transparent);
    }

    #[test]
    fn item2_char_keys() {
        assert_eq!(Keycode::parse("KC_A"), Keycode::Char(CharKey::Letter('a')));
        assert_eq!(Keycode::parse("KC_Z"), Keycode::Char(CharKey::Letter('z')));
        assert_eq!(Keycode::parse("KC_1"), Keycode::Char(CharKey::Digit('1')));
        assert_eq!(Keycode::parse("KC_0"), Keycode::Char(CharKey::Digit('0')));
        assert_eq!(
            Keycode::parse("KC_NONUS_HASH"),
            Keycode::Char(CharKey::NonusHash)
        );
        assert_eq!(Keycode::parse("KC_RO"), Keycode::Char(CharKey::Ro));
        assert_eq!(Keycode::parse("KC_JYEN"), Keycode::Char(CharKey::Jyen));
    }

    #[test]
    fn item2_edit_keys_all_four() {
        assert_eq!(Keycode::parse("KC_ENTER"), Keycode::Edit(EditKey::Enter));
        assert_eq!(Keycode::parse("KC_BSPACE"), Keycode::Edit(EditKey::Bspace));
        assert_eq!(Keycode::parse("KC_TAB"), Keycode::Edit(EditKey::Tab));
        assert_eq!(Keycode::parse("KC_SPACE"), Keycode::Edit(EditKey::Space));
        assert_eq!(EditKey::Enter.label(), "Enter");
        assert_eq!(EditKey::Bspace.label(), "Bksp");
        assert_eq!(EditKey::Tab.label(), "Tab");
        assert_eq!(EditKey::Space.label(), "Space");
    }

    #[test]
    fn item2_named_keys() -> Result<(), String> {
        assert_eq!(named_label("KC_ESCAPE")?, "Esc");
        assert_eq!(named_label("KC_F1")?, "F1");
        assert_eq!(named_label("KC_F12")?, "F12");
        assert_eq!(named_label("KC_UP")?, "↑");
        assert_eq!(named_label("KC_MHEN")?, "無変換");
        assert_eq!(named_label("KC_LANG2")?, "LANG2");
        // CapsLock は、名前だけのキー（状態は持たない）
        assert_eq!(named_label("KC_CAPSLOCK")?, "Caps");
        assert_eq!(named_label("KC_DELETE")?, "Del");
        assert_eq!(named_label("KC_LEFT")?, "←");
        assert_eq!(named_label("KC_HENK")?, "変換");
        Ok(())
    }

    #[test]
    fn item2_modifiers_all_eight() {
        let table = [
            ("KC_LCTRL", Modifier::LCtrl, "LCtrl"),
            ("KC_LSHIFT", Modifier::LShift, "LShift"),
            ("KC_LALT", Modifier::LAlt, "LAlt"),
            ("KC_LGUI", Modifier::LGui, "LGUI"),
            ("KC_RCTRL", Modifier::RCtrl, "RCtrl"),
            ("KC_RSHIFT", Modifier::RShift, "RShift"),
            ("KC_RALT", Modifier::RAlt, "RAlt"),
            ("KC_RGUI", Modifier::RGui, "RGUI"),
        ];
        for (text, modifier, label) in table {
            assert_eq!(Keycode::parse(text), Keycode::Modifier(modifier), "{text}");
            assert_eq!(modifier.label(), label);
        }
    }

    #[test]
    fn item2_mo_layers() {
        assert_eq!(Keycode::parse("MO(0)"), Keycode::Mo(0));
        assert_eq!(Keycode::parse("MO(1)"), Keycode::Mo(1));
        assert_eq!(Keycode::parse("MO(31)"), Keycode::Mo(31));
    }

    #[test]
    fn item2_shifted_names_all_21() -> Result<(), String> {
        let table = [
            ("KC_TILD", CharKey::Grave),
            ("KC_EXLM", CharKey::Digit('1')),
            ("KC_AT", CharKey::Digit('2')),
            ("KC_HASH", CharKey::Digit('3')),
            ("KC_DLR", CharKey::Digit('4')),
            ("KC_PERC", CharKey::Digit('5')),
            ("KC_CIRC", CharKey::Digit('6')),
            ("KC_AMPR", CharKey::Digit('7')),
            ("KC_ASTR", CharKey::Digit('8')),
            ("KC_LPRN", CharKey::Digit('9')),
            ("KC_RPRN", CharKey::Digit('0')),
            ("KC_UNDS", CharKey::Minus),
            ("KC_PLUS", CharKey::Equal),
            ("KC_LCBR", CharKey::LBracket),
            ("KC_RCBR", CharKey::RBracket),
            ("KC_PIPE", CharKey::Bslash),
            ("KC_COLN", CharKey::Scolon),
            ("KC_DQUO", CharKey::Quote),
            ("KC_LT", CharKey::Comma),
            ("KC_GT", CharKey::Dot),
            ("KC_QUES", CharKey::Slash),
        ];
        assert_eq!(table.len(), 21);
        for (text, key) in table {
            assert_eq!(shifted_base(text)?, BaseKey::Char(key), "{text}");
        }
        Ok(())
    }

    #[test]
    fn item2_shifted_forms() -> Result<(), String> {
        assert_eq!(
            shifted_base("LSFT(KC_2)")?,
            BaseKey::Char(CharKey::Digit('2'))
        );
        assert_eq!(
            shifted_base("RSFT(KC_A)")?,
            BaseKey::Char(CharKey::Letter('a'))
        );
        assert_eq!(
            shifted_base("LSFT(KC_ENTER)")?,
            BaseKey::Edit(EditKey::Enter)
        );
        assert_eq!(shifted_base("LSFT(KC_RO)")?, BaseKey::Char(CharKey::Ro));
        match shifted_base("LSFT(KC_F1)")? {
            BaseKey::Named(key) => assert_eq!(key.label(), "F1"),
            other => return Err(format!("LSFT(KC_F1) の中身が違う: {other:?}")),
        }
        Ok(())
    }

    #[test]
    fn item2_unsupported_keeps_the_text() {
        let texts = [
            "",
            "KC_",
            "kc_a",
            "KC_a",
            "KC_z",
            "Kc_A",
            " KC_A",
            "KC_A ",
            "KC_BSPC",
            "KC_TRANSPARENT",
            "MO(32)",
            "MO(-1)",
            "MO(01)",
            "MO()",
            "MO(1",
            "MO(+1)",
            "MO(256)",
            "LSFT()",
            "LSFT(KC_LSHIFT)",
            "LSFT(MO(1))",
            "LSFT(LSFT(KC_A))",
            "LSFT(KC_TRNS)",
            "LSFT(KC_UNDS)",
            "LCTL(KC_C)",
            "USER00",
            "KC_MUTE",
            "KC_BTN3",
            "LT1(KC_SPACE)",
            "TG(1)",
            "0x7c79",
            "KC_NONUS_BSLASH",
            "KC_KP_1",
            "KC_AA",
            "KC_é",
        ];
        for text in texts {
            assert_eq!(Keycode::parse(text), unsupported(text), "{text:?}");
        }
    }

    #[test]
    fn item2_integers() {
        assert_eq!(Keycode::from_int(-1), Keycode::No);
        assert_eq!(Keycode::from_int(5), unsupported("5"));
        assert_eq!(Keycode::from_int(65535), unsupported("65535"));
        assert_eq!(Keycode::from_int(0), unsupported("0"));
        assert_eq!(Keycode::from_int(-2), unsupported("-2"));
    }
}
