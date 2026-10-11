//! 出た文字の欄。末尾に足すだけの表示で、カーソルは無い（`docs/design.md` の B-38・B-40）。

use crate::engine::FieldAction;

/// 出た文字の欄。
///
/// ```rust
/// use cornix_logic::engine::FieldAction;
/// use cornix_logic::text::TextField;
///
/// let mut field = TextField::default();
/// field.apply(FieldAction::Insert('a'));
/// field.apply(FieldAction::Insert('b'));
/// field.apply(FieldAction::DeleteLast);
/// assert_eq!(field.text(), "a");
/// ```
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct TextField {
    text: String,
}

impl TextField {
    /// 欄の文字列。
    #[must_use]
    pub fn text(&self) -> &str {
        &self.text
    }

    /// 欄への動きを当てる。1文字消すのは、欄が空なら、何もしない。
    pub fn apply(&mut self, action: FieldAction) {
        match action {
            FieldAction::Insert(typed) => self.text.push(typed),
            FieldAction::DeleteLast => {
                self.text.pop();
            }
            FieldAction::None => {}
        }
    }

    /// 全部消す（B-40）。
    pub fn clear(&mut self) {
        self.text.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::TextField;
    use crate::engine::FieldAction;

    fn field_of(actions: &[FieldAction]) -> TextField {
        let mut field = TextField::default();
        for action in actions {
            field.apply(*action);
        }
        field
    }

    #[test]
    fn item10_characters_are_appended_in_order() {
        let field = field_of(&[
            FieldAction::Insert('a'),
            FieldAction::Insert(' '),
            FieldAction::Insert('B'),
            FieldAction::Insert('\n'),
            FieldAction::Insert('\t'),
            FieldAction::None,
            FieldAction::Insert('!'),
        ]);
        assert_eq!(field.text(), "a B\n\t!");
        assert_eq!(TextField::default().text(), "");
    }

    #[test]
    fn item10_delete_last() {
        // 空のときは、何も起きない
        let mut field = TextField::default();
        field.apply(FieldAction::DeleteLast);
        assert_eq!(field.text(), "");
        // 1文字のときは、空になる
        let mut field = field_of(&[FieldAction::Insert('a')]);
        field.apply(FieldAction::DeleteLast);
        assert_eq!(field.text(), "");
        field.apply(FieldAction::DeleteLast);
        assert_eq!(field.text(), "");
        // 末尾の1文字だけが消える
        let mut field = field_of(&[FieldAction::Insert('a'), FieldAction::Insert('b')]);
        field.apply(FieldAction::DeleteLast);
        assert_eq!(field.text(), "a");
        // 改行やタブ文字も、1文字として消える
        let mut field = field_of(&[
            FieldAction::Insert('a'),
            FieldAction::Insert('\n'),
            FieldAction::Insert('\t'),
        ]);
        field.apply(FieldAction::DeleteLast);
        assert_eq!(field.text(), "a\n");
        field.apply(FieldAction::DeleteLast);
        assert_eq!(field.text(), "a");
        // 何もしない動きは、欄を変えない
        field.apply(FieldAction::None);
        assert_eq!(field.text(), "a");
    }

    #[test]
    fn item10_clear() {
        let mut field = field_of(&[FieldAction::Insert('a'), FieldAction::Insert('\n')]);
        field.clear();
        assert_eq!(field.text(), "");
        // 空のときに全部消しても、空のまま
        field.clear();
        assert_eq!(field.text(), "");
        // 消したあとも、足せる
        field.apply(FieldAction::Insert('z'));
        assert_eq!(field.text(), "z");
    }
}
