//! 手元のキーとの対応表。手元のキーの code から、Cornix の位置を引く（B-20）。
//!
//! 表は `docs/design.md` の 3.4節から手で写した物（109配列の日本語配列のキーボード向け）。

/// 対応表の1行。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Entry {
    /// 手元のキーの code（`KeyboardEvent.code`）。
    pub code: &'static str,
    /// Cornix の位置（行, 列）。
    pub position: (usize, usize),
    /// 手元の刻印。
    pub label: &'static str,
}

const fn entry(code: &'static str, row: usize, col: usize, label: &'static str) -> Entry {
    Entry {
        code,
        position: (row, col),
        label,
    }
}

/// 対応表（3.4節）。上の3段の36行、下段の11行、2つ目の code の2行（決め方6）の順。
pub const ENTRIES: [Entry; 49] = [
    entry("Tab", 0, 0, "Tab"),
    entry("KeyQ", 0, 1, "Q"),
    entry("KeyW", 0, 2, "W"),
    entry("KeyE", 0, 3, "E"),
    entry("KeyR", 0, 4, "R"),
    entry("KeyT", 0, 5, "T"),
    entry("CapsLock", 1, 0, "英数"),
    entry("KeyA", 1, 1, "A"),
    entry("KeyS", 1, 2, "S"),
    entry("KeyD", 1, 3, "D"),
    entry("KeyF", 1, 4, "F"),
    entry("KeyG", 1, 5, "G"),
    entry("ShiftLeft", 2, 0, "Shift"),
    entry("KeyZ", 2, 1, "Z"),
    entry("KeyX", 2, 2, "X"),
    entry("KeyC", 2, 3, "C"),
    entry("KeyV", 2, 4, "V"),
    entry("KeyB", 2, 5, "B"),
    entry("KeyY", 4, 5, "Y"),
    entry("KeyU", 4, 4, "U"),
    entry("KeyI", 4, 3, "I"),
    entry("KeyO", 4, 2, "O"),
    entry("KeyP", 4, 1, "P"),
    entry("BracketLeft", 4, 0, "@"),
    entry("KeyH", 5, 5, "H"),
    entry("KeyJ", 5, 4, "J"),
    entry("KeyK", 5, 3, "K"),
    entry("KeyL", 5, 2, "L"),
    entry("Semicolon", 5, 1, ";"),
    entry("Quote", 5, 0, ":"),
    entry("KeyN", 6, 5, "N"),
    entry("KeyM", 6, 4, "M"),
    entry("Comma", 6, 3, ","),
    entry("Period", 6, 2, "."),
    entry("Slash", 6, 1, "/"),
    entry("IntlRo", 6, 0, "ろ"),
    // 下段（(3,2) は、手元のキー無し）
    entry("ControlLeft", 3, 0, "Ctrl"),
    entry("MetaLeft", 3, 1, "Win"),
    entry("AltLeft", 3, 3, "Alt"),
    entry("NonConvert", 3, 4, "無変換"),
    entry("Space", 3, 5, "Space"),
    entry("Enter", 7, 5, "Enter"),
    entry("KanaMode", 7, 4, "カタカナひらがな"),
    entry("AltRight", 7, 3, "Alt"),
    entry("MetaRight", 7, 2, "Win"),
    entry("ContextMenu", 7, 1, "アプリケーション"),
    entry("ControlRight", 7, 0, "Ctrl"),
    // 2つ目の code（決め方6）
    entry("Backspace", 7, 4, "Backspace"),
    entry("Escape", 1, 0, "Esc"),
];

/// 手元のキーの code から、Cornix の位置を引く。表に無い code は `None`。
///
/// ```rust
/// use cornix_logic::hostmap::position_of;
///
/// assert_eq!(position_of("KeyQ"), Some((0, 1)));
/// assert_eq!(position_of("F1"), None);
/// ```
#[must_use]
pub fn position_of(code: &str) -> Option<(usize, usize)> {
    ENTRIES
        .iter()
        .find(|entry| entry.code == code)
        .map(|entry| entry.position)
}

/// 位置から、手元の刻印を引く。手元のキーが当たっていない位置は `None`。
///
/// 2つの code が当たる位置は、1つ目と2つ目の刻印を「／」で並べる（決め方6）。
#[must_use]
pub fn label_of(row: usize, col: usize) -> Option<String> {
    let labels: Vec<&str> = ENTRIES
        .iter()
        .filter(|entry| entry.position == (row, col))
        .map(|entry| entry.label)
        .collect();
    (!labels.is_empty()).then(|| labels.join("／"))
}

#[cfg(test)]
mod tests {
    use super::{ENTRIES, label_of, position_of};

    /// design 3.4節の表から、手で写した行（code・位置・刻印）。
    const UPPER: [(&str, (usize, usize), &str); 36] = [
        ("Tab", (0, 0), "Tab"),
        ("KeyQ", (0, 1), "Q"),
        ("KeyW", (0, 2), "W"),
        ("KeyE", (0, 3), "E"),
        ("KeyR", (0, 4), "R"),
        ("KeyT", (0, 5), "T"),
        ("CapsLock", (1, 0), "英数／Esc"),
        ("KeyA", (1, 1), "A"),
        ("KeyS", (1, 2), "S"),
        ("KeyD", (1, 3), "D"),
        ("KeyF", (1, 4), "F"),
        ("KeyG", (1, 5), "G"),
        ("ShiftLeft", (2, 0), "Shift"),
        ("KeyZ", (2, 1), "Z"),
        ("KeyX", (2, 2), "X"),
        ("KeyC", (2, 3), "C"),
        ("KeyV", (2, 4), "V"),
        ("KeyB", (2, 5), "B"),
        ("KeyY", (4, 5), "Y"),
        ("KeyU", (4, 4), "U"),
        ("KeyI", (4, 3), "I"),
        ("KeyO", (4, 2), "O"),
        ("KeyP", (4, 1), "P"),
        ("BracketLeft", (4, 0), "@"),
        ("KeyH", (5, 5), "H"),
        ("KeyJ", (5, 4), "J"),
        ("KeyK", (5, 3), "K"),
        ("KeyL", (5, 2), "L"),
        ("Semicolon", (5, 1), ";"),
        ("Quote", (5, 0), ":"),
        ("KeyN", (6, 5), "N"),
        ("KeyM", (6, 4), "M"),
        ("Comma", (6, 3), ","),
        ("Period", (6, 2), "."),
        ("Slash", (6, 1), "/"),
        ("IntlRo", (6, 0), "ろ"),
    ];

    const LOWER: [(&str, (usize, usize), &str); 11] = [
        ("ControlLeft", (3, 0), "Ctrl"),
        ("MetaLeft", (3, 1), "Win"),
        ("AltLeft", (3, 3), "Alt"),
        ("NonConvert", (3, 4), "無変換"),
        ("Space", (3, 5), "Space"),
        ("Enter", (7, 5), "Enter"),
        ("KanaMode", (7, 4), "カタカナひらがな／Backspace"),
        ("AltRight", (7, 3), "Alt"),
        ("MetaRight", (7, 2), "Win"),
        ("ContextMenu", (7, 1), "アプリケーション"),
        ("ControlRight", (7, 0), "Ctrl"),
    ];

    #[test]
    fn item11_every_row_maps_code_to_position_and_label() {
        for (code, position, label) in UPPER.iter().chain(LOWER.iter()) {
            assert_eq!(position_of(code), Some(*position), "{code}");
            assert_eq!(
                label_of(position.0, position.1).as_deref(),
                Some(*label),
                "{code}"
            );
        }
        // 2つ目の code（決め方6）は、1つ目と同じ位置
        assert_eq!(position_of("Backspace"), Some((7, 4)));
        assert_eq!(position_of("Escape"), Some((1, 0)));
        assert_eq!(ENTRIES.len(), 36 + 11 + 2);
    }

    #[test]
    fn item11_codes_not_in_the_table_have_no_position() {
        for code in [
            "F1",
            "Digit1",
            "",
            "keyq",
            "KeyQ ",
            " KeyQ",
            "Convert",
            "ShiftRight",
            "Backquote",
            "KEYQ",
        ] {
            assert_eq!(position_of(code), None, "{code:?}");
        }
    }

    #[test]
    fn item11_positions_without_a_host_key_have_no_label() {
        // 手元のキーが当たっていない位置
        assert_eq!(label_of(3, 2), None);
        // 丸い部品
        assert_eq!(label_of(2, 6), None);
        assert_eq!(label_of(5, 6), None);
        // キーの無い位置と、範囲の外
        assert_eq!(label_of(0, 6), None);
        assert_eq!(label_of(7, 6), None);
        assert_eq!(label_of(8, 0), None);
        // 当たっている位置は、48 のうち 47
        let labelled = (0..8)
            .flat_map(|row| (0..7).map(move |col| (row, col)))
            .filter(|(row, col)| label_of(*row, *col).is_some())
            .count();
        assert_eq!(labelled, 47);
    }

    #[test]
    fn item11_no_duplicates_except_the_second_codes() {
        for (index, entry) in ENTRIES.iter().enumerate() {
            let same_code = ENTRIES
                .iter()
                .filter(|other| other.code == entry.code)
                .count();
            assert_eq!(same_code, 1, "{} が2回出る", entry.code);
            // 丸い部品の位置と、キーの無い位置は、表に出ない
            let (row, col) = entry.position;
            assert!(
                row < 8 && col < 6,
                "{} の位置 {:?}",
                entry.code,
                entry.position
            );
            // 同じ位置が2回出るのは、2つ目の code の行だけ
            let earlier = ENTRIES[..index]
                .iter()
                .any(|other| other.position == entry.position);
            let second_code = matches!(entry.code, "Backspace" | "Escape");
            assert_eq!(
                earlier, second_code,
                "{} の位置 {:?}",
                entry.code, entry.position
            );
        }
        // 2つ目の code の行は、初版では2つ（Backspace→(7,4)、Escape→(1,0)）
        let positions: Vec<(usize, usize)> = ENTRIES.iter().map(|entry| entry.position).collect();
        for position in &positions {
            let times = positions.iter().filter(|other| *other == position).count();
            let expected = if matches!(position, (7, 4) | (1, 0)) {
                2
            } else {
                1
            };
            assert_eq!(times, expected, "{position:?}");
        }
    }
}
