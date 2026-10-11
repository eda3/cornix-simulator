//! 計測ページ（keytest）の記録。届いたキーイベントから、まとめの表の行と、直近のイベントを作る。
//!
//! web-sys には依存しない（`cargo test` で確かめるため）。振る舞いは `docs/design.md` の
//! B-47・B-48・B-51・B-52。

use std::collections::{VecDeque, vec_deque};

/// 直近のイベントとして覚えておく件数（B-48）。
pub const RECENT_LIMIT: usize = 20;

/// Markdown の表の、見出しの2行（B-52）。`docs/measurements.md` の①の枠と同じ文字列。
const MARKDOWN_HEAD: &str = concat!(
    "| 手元の刻印 | code | key | location | keydown | keyup | repeat | メモ |\n",
    "|---|---|---|---|---|---|---|---|\n",
);

/// キーイベントの種類。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EventKind {
    /// 押した（keydown）。
    KeyDown,
    /// 離した（keyup）。
    KeyUp,
}

impl EventKind {
    /// ブラウザのイベントの名前（`keydown`・`keyup`）。
    #[must_use]
    pub const fn name(self) -> &'static str {
        match self {
            Self::KeyDown => "keydown",
            Self::KeyUp => "keyup",
        }
    }
}

/// ブラウザから届いたキーイベント1件。値は、届いたまま持つ。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KeyEvent {
    /// keydown か keyup か。
    pub kind: EventKind,
    /// `KeyboardEvent.code`。
    pub code: String,
    /// `KeyboardEvent.key`。
    pub key: String,
    /// `KeyboardEvent.location`。
    pub location: u32,
    /// `KeyboardEvent.repeat`。
    pub repeat: bool,
}

impl KeyEvent {
    /// 直近のイベントの表に出す文字（種類・code・key・repeat の順。B-48）。
    #[must_use]
    pub fn cells(&self) -> [String; 4] {
        [
            self.kind.name().to_owned(),
            shown(&self.code).to_owned(),
            shown(&self.key).to_owned(),
            self.repeat.to_string(),
        ]
    }
}

/// まとめの表の1行（B-47）。code ごとに1行。code が空文字のときは、key の値ごとに1行。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SummaryRow {
    /// `KeyboardEvent.code`（空文字のこともある）。
    pub code: String,
    /// 届いた key の値。届いた順で、重複なし。
    pub keys: Vec<String>,
    /// 届いた location の値。届いた順で、重複なし。
    pub locations: Vec<u32>,
    /// keydown の回数（repeat を除く）。
    pub keydown: u32,
    /// keyup の回数。
    pub keyup: u32,
    /// repeat の付いた keydown の回数。
    pub repeat: u32,
}

impl SummaryRow {
    /// 表に出す文字（code・key・location・keydown・keyup・repeat の順）。
    ///
    /// 空文字の code は「（空）」と出す（B-47）。key と location は、半角の空白で区切って並べる。
    #[must_use]
    pub fn cells(&self) -> [String; 6] {
        let keys: Vec<&str> = self.keys.iter().map(|key| shown(key)).collect();
        let locations: Vec<String> = self.locations.iter().map(u32::to_string).collect();
        [
            shown(&self.code).to_owned(),
            keys.join(" "),
            locations.join(" "),
            self.keydown.to_string(),
            self.keyup.to_string(),
            self.repeat.to_string(),
        ]
    }

    /// このイベントを数える行か。
    fn holds(&self, event: &KeyEvent) -> bool {
        self.code == event.code && (!event.code.is_empty() || self.keys.contains(&event.key))
    }

    /// イベント1件を、この行に数える。
    fn count(&mut self, event: &KeyEvent) {
        if !self.keys.contains(&event.key) {
            self.keys.push(event.key.clone());
        }
        if !self.locations.contains(&event.location) {
            self.locations.push(event.location);
        }
        match (event.kind, event.repeat) {
            (EventKind::KeyDown, false) => self.keydown += 1,
            (EventKind::KeyDown, true) => self.repeat += 1,
            (EventKind::KeyUp, _) => self.keyup += 1,
        }
    }
}

/// 計測ページの記録。まとめの表の行（B-47）と、直近のイベント（B-48）を持つ。
///
/// ```rust
/// use cornix_web::keylog::{EventKind, KeyEvent, KeyLog};
///
/// let mut log = KeyLog::default();
/// log.record(KeyEvent {
///     kind: EventKind::KeyDown,
///     code: "KeyQ".to_owned(),
///     key: "q".to_owned(),
///     location: 0,
///     repeat: false,
/// });
/// assert_eq!(log.rows().len(), 1);
/// assert_eq!(log.recent().count(), 1);
/// ```
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct KeyLog {
    rows: Vec<SummaryRow>,
    recent: VecDeque<KeyEvent>,
}

impl KeyLog {
    /// イベント1件を記録する。
    ///
    /// まとめの表では、その code の行（無ければ、末尾に足した行）に数える。
    /// 直近のイベントは [`RECENT_LIMIT`] 件までで、あふれた分は古い物から消える。
    pub fn record(&mut self, event: KeyEvent) {
        if let Some(row) = self.rows.iter_mut().find(|row| row.holds(&event)) {
            row.count(&event);
        } else {
            let mut row = SummaryRow {
                code: event.code.clone(),
                keys: Vec::new(),
                locations: Vec::new(),
                keydown: 0,
                keyup: 0,
                repeat: 0,
            };
            row.count(&event);
            self.rows.push(row);
        }
        self.recent.push_front(event);
        self.recent.truncate(RECENT_LIMIT);
    }

    /// まとめの表と、直近のイベントを空にする（B-51）。
    pub fn clear(&mut self) {
        self.rows.clear();
        self.recent.clear();
    }

    /// まとめの表の行。最初に届いた順。
    #[must_use]
    pub fn rows(&self) -> &[SummaryRow] {
        &self.rows
    }

    /// 直近のイベント。新しい順。
    #[must_use]
    pub fn recent(&self) -> vec_deque::Iter<'_, KeyEvent> {
        self.recent.iter()
    }

    /// まとめの表の中身を、Markdown の表にした文字列（B-52）。
    ///
    /// 列は、`docs/measurements.md` の①の枠と同じ8つ。前の「手元の刻印」と、後ろの「メモ」は、
    /// 空で出す（貼ったあとに、手で書く列）。間の6列は、[`SummaryRow::cells`] と同じ。
    /// 値の中の `|` は、表の区切りと重ならないように `\|` と書く。
    #[must_use]
    pub fn to_markdown(&self) -> String {
        let mut markdown = MARKDOWN_HEAD.to_owned();
        for row in &self.rows {
            let cells = row.cells().map(|cell| cell.replace('|', "\\|"));
            markdown.push_str("| | ");
            markdown.push_str(&cells.join(" | "));
            markdown.push_str(" | |\n");
        }
        markdown
    }
}

/// code と key を、表に出す文字にする。空文字は「（空）」、空白1つ（Space の key）は「（空白）」。
fn shown(value: &str) -> &str {
    match value {
        "" => "（空）",
        " " => "（空白）",
        other => other,
    }
}

#[cfg(test)]
mod tests {
    use super::{EventKind, KeyEvent, KeyLog, SummaryRow};

    /// 見出しだけ（行が無い）の Markdown。`docs/measurements.md` の①の枠の2行から、手で写した物。
    const EMPTY_MARKDOWN: &str = concat!(
        "| 手元の刻印 | code | key | location | keydown | keyup | repeat | メモ |\n",
        "|---|---|---|---|---|---|---|---|\n",
    );

    fn event(kind: EventKind, code: &str, key: &str, location: u32, repeat: bool) -> KeyEvent {
        KeyEvent {
            kind,
            code: code.to_owned(),
            key: key.to_owned(),
            location,
            repeat,
        }
    }

    fn down(code: &str, key: &str) -> KeyEvent {
        event(EventKind::KeyDown, code, key, 0, false)
    }

    fn held(code: &str, key: &str) -> KeyEvent {
        event(EventKind::KeyDown, code, key, 0, true)
    }

    fn up(code: &str, key: &str) -> KeyEvent {
        event(EventKind::KeyUp, code, key, 0, false)
    }

    fn log_of(events: Vec<KeyEvent>) -> KeyLog {
        let mut log = KeyLog::default();
        for event in events {
            log.record(event);
        }
        log
    }

    fn row(log: &KeyLog, index: usize) -> Result<&SummaryRow, String> {
        log.rows()
            .get(index)
            .ok_or_else(|| format!("行 {index} がありません"))
    }

    fn recent_codes(log: &KeyLog) -> Vec<&str> {
        log.recent().map(|event| event.code.as_str()).collect()
    }

    #[test]
    fn item1_counts_keydown_keyup_and_repeat_per_code() -> Result<(), String> {
        let log = log_of(vec![
            down("KeyQ", "q"),
            held("KeyQ", "q"),
            held("KeyQ", "q"),
            held("KeyQ", "q"),
            up("KeyQ", "q"),
            down("KeyA", "a"),
            up("KeyA", "a"),
            down("KeyQ", "q"),
            up("KeyQ", "q"),
            down("ShiftLeft", "Shift"),
        ]);

        assert_eq!(log.rows().len(), 3);
        let q = row(&log, 0)?;
        assert_eq!(q.code, "KeyQ");
        assert_eq!((q.keydown, q.keyup, q.repeat), (2, 2, 3));
        let a = row(&log, 1)?;
        assert_eq!(a.code, "KeyA");
        assert_eq!((a.keydown, a.keyup, a.repeat), (1, 1, 0));
        // keyup がまだ来ていないキーは、keydown と keyup の数が合わないまま出る
        let shift = row(&log, 2)?;
        assert_eq!(shift.code, "ShiftLeft");
        assert_eq!((shift.keydown, shift.keyup, shift.repeat), (1, 0, 0));
        Ok(())
    }

    #[test]
    fn item1_repeat_without_keydown_counts_only_repeat() -> Result<(), String> {
        // 押している途中でページに来たとき: repeat の付いた keydown と keyup だけが届く
        let log = log_of(vec![held("KeyQ", "q"), up("KeyQ", "q")]);

        assert_eq!(log.rows().len(), 1);
        let q = row(&log, 0)?;
        assert_eq!((q.keydown, q.keyup, q.repeat), (0, 1, 1));
        Ok(())
    }

    #[test]
    fn item1_keys_are_listed_in_arrival_order_without_duplicates() -> Result<(), String> {
        let log = log_of(vec![
            down("Backquote", "Zenkaku"),
            up("Backquote", "Zenkaku"),
            down("Backquote", "Hankaku"),
            up("Backquote", "Hankaku"),
            down("Backquote", "Zenkaku"),
            down("Digit2", "2"),
            down("Digit2", "\""),
            down("Digit2", "2"),
        ]);

        assert_eq!(log.rows().len(), 2);
        let backquote = row(&log, 0)?;
        assert_eq!(backquote.code, "Backquote");
        assert_eq!(backquote.keys, ["Zenkaku", "Hankaku"]);
        assert_eq!((backquote.keydown, backquote.keyup), (3, 2));
        assert_eq!(backquote.cells()[1], "Zenkaku Hankaku");
        let digit2 = row(&log, 1)?;
        assert_eq!(digit2.keys, ["2", "\""]);
        assert_eq!(digit2.cells()[1], "2 \"");
        Ok(())
    }

    #[test]
    fn item1_keys_keep_arrival_order_not_sorted() -> Result<(), String> {
        let log = log_of(vec![
            down("Backquote", "Zenkaku"),
            down("Backquote", "Hankaku"),
            down("KeyZ", "z"),
            down("KeyZ", "Z"),
        ]);

        // 並べ替えると "Hankaku" が先に、"Z" が先に来る
        assert_eq!(row(&log, 0)?.keys, ["Zenkaku", "Hankaku"]);
        assert_eq!(row(&log, 1)?.keys, ["z", "Z"]);
        Ok(())
    }

    #[test]
    fn item1_rows_are_in_first_arrival_order() {
        let log = log_of(vec![
            down("KeyQ", "q"),
            down("KeyA", "a"),
            up("KeyQ", "q"),
            down("Digit1", "1"),
            up("KeyA", "a"),
            down("KeyQ", "q"),
        ]);

        // 最初に届いた順（並べ替えない。あとからまた届いても、行は動かない）
        let codes: Vec<&str> = log.rows().iter().map(|row| row.code.as_str()).collect();
        assert_eq!(codes, ["KeyQ", "KeyA", "Digit1"]);
    }

    #[test]
    fn item1_empty_code_makes_a_row_per_key() -> Result<(), String> {
        let log = log_of(vec![
            event(EventKind::KeyDown, "", "Shift", 2, false),
            event(EventKind::KeyDown, "", "Unidentified", 0, false),
            event(EventKind::KeyUp, "", "Shift", 2, false),
            event(EventKind::KeyDown, "ShiftLeft", "Shift", 1, false),
            event(EventKind::KeyDown, "", "Unidentified", 0, true),
        ]);

        assert_eq!(log.rows().len(), 3);
        let shift = row(&log, 0)?;
        assert_eq!(shift.code, "");
        assert_eq!(shift.keys, ["Shift"]);
        assert_eq!((shift.keydown, shift.keyup, shift.repeat), (1, 1, 0));
        assert_eq!(shift.cells(), ["（空）", "Shift", "2", "1", "1", "0"]);
        let unidentified = row(&log, 1)?;
        assert_eq!(unidentified.code, "");
        assert_eq!(unidentified.keys, ["Unidentified"]);
        assert_eq!(
            (
                unidentified.keydown,
                unidentified.keyup,
                unidentified.repeat
            ),
            (1, 0, 1)
        );
        assert_eq!(
            unidentified.cells(),
            ["（空）", "Unidentified", "0", "1", "0", "1"]
        );
        // code が空でない行は、同じ key（Shift）でも別の行
        let shift_left = row(&log, 2)?;
        assert_eq!(shift_left.code, "ShiftLeft");
        assert_eq!(
            shift_left.cells(),
            ["ShiftLeft", "Shift", "1", "1", "0", "0"]
        );
        Ok(())
    }

    #[test]
    fn item1_same_key_with_different_codes_makes_separate_rows() {
        // code が空でなければ、key が同じでも code ごとの行（左右の Shift）
        let log = log_of(vec![
            event(EventKind::KeyDown, "ShiftLeft", "Shift", 1, false),
            event(EventKind::KeyDown, "ShiftRight", "Shift", 2, false),
        ]);

        let codes: Vec<&str> = log.rows().iter().map(|row| row.code.as_str()).collect();
        assert_eq!(codes, ["ShiftLeft", "ShiftRight"]);
    }

    #[test]
    fn item1_locations_are_listed_in_arrival_order_without_duplicates() -> Result<(), String> {
        let log = log_of(vec![
            event(EventKind::KeyDown, "ShiftRight", "Shift", 2, false),
            event(EventKind::KeyUp, "ShiftRight", "Shift", 2, false),
            event(EventKind::KeyDown, "Enter", "Enter", 3, false),
            event(EventKind::KeyDown, "Enter", "Enter", 0, false),
            event(EventKind::KeyDown, "Enter", "Enter", 3, false),
        ]);

        assert_eq!(row(&log, 0)?.locations, [2]);
        assert_eq!(row(&log, 0)?.cells()[2], "2");
        assert_eq!(row(&log, 1)?.locations, [3, 0]);
        assert_eq!(row(&log, 1)?.cells()[2], "3 0");
        Ok(())
    }

    #[test]
    fn item1_space_and_empty_key_are_shown_with_names() -> Result<(), String> {
        let log = log_of(vec![down("Space", " "), down("KeyX", "")]);

        assert_eq!(row(&log, 0)?.keys, [" "]);
        assert_eq!(
            row(&log, 0)?.cells(),
            ["Space", "（空白）", "0", "1", "0", "0"]
        );
        assert_eq!(
            row(&log, 1)?.cells(),
            ["KeyX", "（空）", "0", "1", "0", "0"]
        );
        Ok(())
    }

    #[test]
    fn item1_recent_is_newest_first() {
        let log = log_of(vec![down("KeyQ", "q"), up("KeyQ", "q"), held("KeyA", "a")]);

        let recent: Vec<[String; 4]> = log.recent().map(KeyEvent::cells).collect();
        assert_eq!(
            recent,
            [
                ["keydown", "KeyA", "a", "true"],
                ["keyup", "KeyQ", "q", "false"],
                ["keydown", "KeyQ", "q", "false"],
            ]
        );
    }

    #[test]
    fn item1_recent_shows_empty_code_and_space_key_with_names() {
        let log = log_of(vec![
            event(EventKind::KeyDown, "", "Shift", 2, false),
            up("Space", " "),
        ]);

        let recent: Vec<[String; 4]> = log.recent().map(KeyEvent::cells).collect();
        assert_eq!(
            recent,
            [
                ["keyup", "Space", "（空白）", "false"],
                ["keydown", "（空）", "Shift", "false"],
            ]
        );
    }

    #[test]
    fn item1_recent_keeps_20_and_drops_the_oldest_at_21() {
        let codes = [
            "K1", "K2", "K3", "K4", "K5", "K6", "K7", "K8", "K9", "K10", "K11", "K12", "K13",
            "K14", "K15", "K16", "K17", "K18", "K19", "K20", "K21", "K22",
        ];
        let mut log = KeyLog::default();

        // 19件: 全部残る
        for code in &codes[..19] {
            log.record(down(code, "x"));
        }
        assert_eq!(
            recent_codes(&log),
            [
                "K19", "K18", "K17", "K16", "K15", "K14", "K13", "K12", "K11", "K10", "K9", "K8",
                "K7", "K6", "K5", "K4", "K3", "K2", "K1",
            ]
        );

        // 20件: ちょうど上限。全部残る
        log.record(down(codes[19], "x"));
        assert_eq!(
            recent_codes(&log),
            [
                "K20", "K19", "K18", "K17", "K16", "K15", "K14", "K13", "K12", "K11", "K10", "K9",
                "K8", "K7", "K6", "K5", "K4", "K3", "K2", "K1",
            ]
        );

        // 21件目: いちばん古い K1 が消える
        log.record(down(codes[20], "x"));
        assert_eq!(
            recent_codes(&log),
            [
                "K21", "K20", "K19", "K18", "K17", "K16", "K15", "K14", "K13", "K12", "K11", "K10",
                "K9", "K8", "K7", "K6", "K5", "K4", "K3", "K2",
            ]
        );

        // 22件目: 次に古い K2 が消える
        log.record(down(codes[21], "x"));
        assert_eq!(
            recent_codes(&log),
            [
                "K22", "K21", "K20", "K19", "K18", "K17", "K16", "K15", "K14", "K13", "K12", "K11",
                "K10", "K9", "K8", "K7", "K6", "K5", "K4", "K3",
            ]
        );

        // まとめの表は、直近の20件と関係なく、全部の行を持つ
        assert_eq!(log.rows().len(), 22);
    }

    #[test]
    fn item1_recent_counts_repeats_as_events() {
        // 同じキーの押し続けも、1件ずつ数える（21件で、最初の keydown が消える）
        let mut log = KeyLog::default();
        log.record(down("KeyQ", "q"));
        for _ in 0..20 {
            log.record(held("KeyQ", "q"));
        }

        assert_eq!(log.recent().count(), 20);
        assert!(log.recent().all(|event| event.repeat));
        assert_eq!(log.rows().len(), 1);
    }

    #[test]
    fn item1_clear_empties_rows_and_recent() -> Result<(), String> {
        let mut log = log_of(vec![
            down("KeyQ", "q"),
            held("KeyQ", "q"),
            up("KeyQ", "q"),
            down("", "Shift"),
        ]);
        assert_eq!(log.rows().len(), 2);
        assert_eq!(log.recent().count(), 4);

        log.clear();

        assert!(log.rows().is_empty());
        assert_eq!(log.recent().count(), 0);
        assert_eq!(log.to_markdown(), EMPTY_MARKDOWN);

        // 消したあとは、数え直しになる（前の回数を引き継がない）
        log.record(down("KeyQ", "q"));
        assert_eq!(log.rows().len(), 1);
        let q = row(&log, 0)?;
        assert_eq!((q.keydown, q.keyup, q.repeat), (1, 0, 0));
        assert_eq!(recent_codes(&log), ["KeyQ"]);
        Ok(())
    }

    #[test]
    fn item1_clear_on_empty_log_stays_empty() {
        let mut log = KeyLog::default();

        log.clear();

        assert!(log.rows().is_empty());
        assert_eq!(log.recent().count(), 0);
    }

    #[test]
    fn item1_no_events_gives_empty_table() {
        let log = KeyLog::default();

        assert!(log.rows().is_empty());
        assert_eq!(log.recent().count(), 0);
        assert_eq!(log.to_markdown(), EMPTY_MARKDOWN);
    }

    #[test]
    fn item1_markdown_has_the_same_rows_as_the_summary() {
        let log = log_of(vec![
            down("KeyQ", "q"),
            held("KeyQ", "q"),
            up("KeyQ", "q"),
            down("Backquote", "Zenkaku"),
            up("Backquote", "Hankaku"),
            event(EventKind::KeyDown, "", "Shift", 2, false),
            down("Space", " "),
            up("Space", " "),
            down("IntlYen", "\\"),
            down("IntlYen", "|"),
            event(EventKind::KeyDown, "ShiftLeft", "Shift", 1, false),
        ]);

        assert_eq!(
            log.to_markdown(),
            "| 手元の刻印 | code | key | location | keydown | keyup | repeat | メモ |\n\
             |---|---|---|---|---|---|---|---|\n\
             | | KeyQ | q | 0 | 1 | 1 | 1 | |\n\
             | | Backquote | Zenkaku Hankaku | 0 | 1 | 1 | 0 | |\n\
             | | （空） | Shift | 2 | 1 | 0 | 0 | |\n\
             | | Space | （空白） | 0 | 1 | 1 | 0 | |\n\
             | | IntlYen | \\ \\| | 0 | 2 | 0 | 0 | |\n\
             | | ShiftLeft | Shift | 1 | 1 | 0 | 0 | |\n"
        );
    }
}
