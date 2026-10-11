//! 文字のキーが出す文字（JIS と US）。表は `docs/design.md` の 3.2節から手で写した物（B-37）。

use crate::keycode::CharKey;

/// PC の側のキーボード配列。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Layout {
    /// 日本語配列。
    Jis,
    /// US 配列。
    Us,
}

/// 文字のキーが出す文字。`None` は、文字が出ない（3.2節の「なし」）。
///
/// ```rust
/// use cornix_logic::chars::{Layout, char_of};
/// use cornix_logic::keycode::CharKey;
///
/// assert_eq!(char_of(CharKey::Digit('2'), true, Layout::Jis), Some('"'));
/// assert_eq!(char_of(CharKey::Digit('2'), true, Layout::Us), Some('@'));
/// ```
#[must_use]
pub const fn char_of(key: CharKey, shift: bool, layout: Layout) -> Option<char> {
    let (plain, shifted) = match layout {
        Layout::Us => us(key),
        Layout::Jis => jis(key),
    };
    if shift { shifted } else { plain }
}

/// 素が「なし」のキーの名前（JIS の `KC_GRAVE`、US の `KC_RO`・`KC_JYEN`）。ほかは `None`。
#[must_use]
pub const fn name_of(key: CharKey, layout: Layout) -> Option<&'static str> {
    match (layout, key) {
        (Layout::Jis, CharKey::Grave) => Some("半角/全角"),
        (Layout::Us, CharKey::Ro) => Some("Ro"),
        (Layout::Us, CharKey::Jyen) => Some("Yen"),
        _ => None,
    }
}

/// US の列（素・Shift）。
const fn us(key: CharKey) -> (Option<char>, Option<char>) {
    match key {
        CharKey::Letter(letter) => (Some(letter), Some(letter.to_ascii_uppercase())),
        CharKey::Digit(digit) => (
            Some(digit),
            match digit {
                '1' => Some('!'),
                '2' => Some('@'),
                '3' => Some('#'),
                '4' => Some('$'),
                '5' => Some('%'),
                '6' => Some('^'),
                '7' => Some('&'),
                '8' => Some('*'),
                '9' => Some('('),
                '0' => Some(')'),
                _ => None,
            },
        ),
        CharKey::Minus => (Some('-'), Some('_')),
        CharKey::Equal => (Some('='), Some('+')),
        CharKey::LBracket => (Some('['), Some('{')),
        CharKey::RBracket => (Some(']'), Some('}')),
        CharKey::Bslash | CharKey::NonusHash => (Some('\\'), Some('|')),
        CharKey::Scolon => (Some(';'), Some(':')),
        CharKey::Quote => (Some('\''), Some('"')),
        CharKey::Grave => (Some('`'), Some('~')),
        CharKey::Comma => (Some(','), Some('<')),
        CharKey::Dot => (Some('.'), Some('>')),
        CharKey::Slash => (Some('/'), Some('?')),
        CharKey::Ro | CharKey::Jyen => (None, None),
    }
}

/// JIS の列（素・Shift）。
const fn jis(key: CharKey) -> (Option<char>, Option<char>) {
    match key {
        CharKey::Letter(letter) => (Some(letter), Some(letter.to_ascii_uppercase())),
        CharKey::Digit(digit) => (
            Some(digit),
            match digit {
                '1' => Some('!'),
                '2' => Some('"'),
                '3' => Some('#'),
                '4' => Some('$'),
                '5' => Some('%'),
                '6' => Some('&'),
                '7' => Some('\''),
                '8' => Some('('),
                '9' => Some(')'),
                _ => None,
            },
        ),
        CharKey::Minus => (Some('-'), Some('=')),
        CharKey::Equal => (Some('^'), Some('~')),
        CharKey::LBracket => (Some('@'), Some('`')),
        CharKey::RBracket => (Some('['), Some('{')),
        CharKey::Bslash | CharKey::NonusHash => (Some(']'), Some('}')),
        CharKey::Scolon => (Some(';'), Some('+')),
        CharKey::Quote => (Some(':'), Some('*')),
        CharKey::Grave => (None, None),
        CharKey::Comma => (Some(','), Some('<')),
        CharKey::Dot => (Some('.'), Some('>')),
        CharKey::Slash => (Some('/'), Some('?')),
        CharKey::Ro => (Some('\\'), Some('_')),
        CharKey::Jyen => (Some('\\'), Some('|')),
    }
}

#[cfg(test)]
mod tests {
    use super::{Layout, char_of, name_of};
    use crate::keycode::CharKey;

    /// 4通り（US 素・US Shift・JIS 素・JIS Shift）。
    fn four(key: CharKey) -> [Option<char>; 4] {
        [
            char_of(key, false, Layout::Us),
            char_of(key, true, Layout::Us),
            char_of(key, false, Layout::Jis),
            char_of(key, true, Layout::Jis),
        ]
    }

    #[test]
    fn item9_keys_that_differ_between_jis_and_us() {
        // design 3.2節の表から、手で写した（US 素・US Shift・JIS 素・JIS Shift）
        let table = [
            (
                CharKey::Digit('2'),
                [Some('2'), Some('@'), Some('2'), Some('"')],
            ),
            (
                CharKey::Digit('6'),
                [Some('6'), Some('^'), Some('6'), Some('&')],
            ),
            (
                CharKey::Digit('7'),
                [Some('7'), Some('&'), Some('7'), Some('\'')],
            ),
            (
                CharKey::Digit('8'),
                [Some('8'), Some('*'), Some('8'), Some('(')],
            ),
            (
                CharKey::Digit('9'),
                [Some('9'), Some('('), Some('9'), Some(')')],
            ),
            (CharKey::Digit('0'), [Some('0'), Some(')'), Some('0'), None]),
            (CharKey::Minus, [Some('-'), Some('_'), Some('-'), Some('=')]),
            (CharKey::Equal, [Some('='), Some('+'), Some('^'), Some('~')]),
            (
                CharKey::LBracket,
                [Some('['), Some('{'), Some('@'), Some('`')],
            ),
            (
                CharKey::RBracket,
                [Some(']'), Some('}'), Some('['), Some('{')],
            ),
            (
                CharKey::Bslash,
                [Some('\\'), Some('|'), Some(']'), Some('}')],
            ),
            (
                CharKey::NonusHash,
                [Some('\\'), Some('|'), Some(']'), Some('}')],
            ),
            (
                CharKey::Scolon,
                [Some(';'), Some(':'), Some(';'), Some('+')],
            ),
            (
                CharKey::Quote,
                [Some('\''), Some('"'), Some(':'), Some('*')],
            ),
            (CharKey::Grave, [Some('`'), Some('~'), None, None]),
            (CharKey::Ro, [None, None, Some('\\'), Some('_')]),
            (CharKey::Jyen, [None, None, Some('\\'), Some('|')]),
        ];
        assert_eq!(table.len(), 17);
        for (key, expected) in table {
            assert_eq!(four(key), expected, "{key:?}");
        }
    }

    #[test]
    fn item9_keys_that_are_the_same_in_both() {
        let table = [
            (
                CharKey::Letter('a'),
                [Some('a'), Some('A'), Some('a'), Some('A')],
            ),
            (
                CharKey::Letter('z'),
                [Some('z'), Some('Z'), Some('z'), Some('Z')],
            ),
            (
                CharKey::Digit('1'),
                [Some('1'), Some('!'), Some('1'), Some('!')],
            ),
            (
                CharKey::Digit('3'),
                [Some('3'), Some('#'), Some('3'), Some('#')],
            ),
            (
                CharKey::Digit('4'),
                [Some('4'), Some('$'), Some('4'), Some('$')],
            ),
            (
                CharKey::Digit('5'),
                [Some('5'), Some('%'), Some('5'), Some('%')],
            ),
            (CharKey::Comma, [Some(','), Some('<'), Some(','), Some('<')]),
            (CharKey::Dot, [Some('.'), Some('>'), Some('.'), Some('>')]),
            (CharKey::Slash, [Some('/'), Some('?'), Some('/'), Some('?')]),
        ];
        for (key, expected) in table {
            assert_eq!(four(key), expected, "{key:?}");
        }
    }

    #[test]
    fn item9_all_the_cells_without_a_character() {
        assert_eq!(char_of(CharKey::Digit('0'), true, Layout::Jis), None);
        assert_eq!(char_of(CharKey::Grave, false, Layout::Jis), None);
        assert_eq!(char_of(CharKey::Grave, true, Layout::Jis), None);
        assert_eq!(char_of(CharKey::Ro, false, Layout::Us), None);
        assert_eq!(char_of(CharKey::Ro, true, Layout::Us), None);
        assert_eq!(char_of(CharKey::Jyen, false, Layout::Us), None);
        assert_eq!(char_of(CharKey::Jyen, true, Layout::Us), None);
        // 「なし」は、この7つだけ
        let keys = [
            CharKey::Letter('a'),
            CharKey::Letter('m'),
            CharKey::Letter('z'),
            CharKey::Digit('0'),
            CharKey::Digit('1'),
            CharKey::Digit('2'),
            CharKey::Digit('3'),
            CharKey::Digit('4'),
            CharKey::Digit('5'),
            CharKey::Digit('6'),
            CharKey::Digit('7'),
            CharKey::Digit('8'),
            CharKey::Digit('9'),
            CharKey::Minus,
            CharKey::Equal,
            CharKey::LBracket,
            CharKey::RBracket,
            CharKey::Bslash,
            CharKey::NonusHash,
            CharKey::Scolon,
            CharKey::Quote,
            CharKey::Grave,
            CharKey::Comma,
            CharKey::Dot,
            CharKey::Slash,
            CharKey::Ro,
            CharKey::Jyen,
        ];
        let none: usize = keys
            .iter()
            .map(|key| four(*key).iter().filter(|cell| cell.is_none()).count())
            .sum();
        assert_eq!(none, 7);
    }

    #[test]
    fn item9_names_of_keys_without_a_plain_character() {
        assert_eq!(name_of(CharKey::Grave, Layout::Jis), Some("半角/全角"));
        assert_eq!(name_of(CharKey::Ro, Layout::Us), Some("Ro"));
        assert_eq!(name_of(CharKey::Jyen, Layout::Us), Some("Yen"));
        // 素の文字があるキーには、名前が無い
        assert_eq!(name_of(CharKey::Grave, Layout::Us), None);
        assert_eq!(name_of(CharKey::Ro, Layout::Jis), None);
        assert_eq!(name_of(CharKey::Jyen, Layout::Jis), None);
        assert_eq!(name_of(CharKey::Letter('a'), Layout::Jis), None);
        assert_eq!(name_of(CharKey::Digit('0'), Layout::Jis), None);
    }
}
