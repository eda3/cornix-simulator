//! 注意の文（`docs/design.md` の B-43）。全画面かどうかと、Keyboard Lock が掛かっているかで変わる。

/// ふつうの窓のときの文。
const WINDOWED: &str = "ふつうの窓では、Ctrl+W でタブが閉じ、Win キーでスタートメニューが開きます。Ctrl・Win・Alt も試すときは「全画面で試す」を使ってください。";
/// 全画面で、Keyboard Lock が掛かっているときの文。
const LOCKED: &str = "全画面です。マウスを画面の上へ動かすと出るボタンで戻れます。";
/// 全画面で、Keyboard Lock が掛かっていないときの文。
const NOT_LOCKED: &str = "全画面ですが、このブラウザでは Win キーなどを受けられません。";

/// 注意の文。ふつうの窓では、Keyboard Lock の様子は見ない。
///
/// ```rust
/// use cornix_logic::caution::text;
///
/// assert_eq!(
///     text(true, true),
///     "全画面です。マウスを画面の上へ動かすと出るボタンで戻れます。"
/// );
/// ```
#[must_use]
pub const fn text(fullscreen: bool, keyboard_locked: bool) -> &'static str {
    match (fullscreen, keyboard_locked) {
        (false, _) => WINDOWED,
        (true, true) => LOCKED,
        (true, false) => NOT_LOCKED,
    }
}

/// 全画面のボタンの文（B-42）。
#[must_use]
pub const fn button_label(fullscreen: bool) -> &'static str {
    if fullscreen {
        "全画面をやめる"
    } else {
        "全画面で試す"
    }
}

#[cfg(test)]
mod tests {
    use super::{button_label, text};

    /// 期待値は、design の B-42 から手で写した物。
    #[test]
    fn b42_button_label() {
        assert_eq!(button_label(false), "全画面で試す");
        assert_eq!(button_label(true), "全画面をやめる");
    }

    /// 期待値は、design の B-43 から手で写した物。
    #[test]
    fn b43_three_texts_by_state() {
        let windowed = "ふつうの窓では、Ctrl+W でタブが閉じ、Win キーでスタートメニューが開きます。Ctrl・Win・Alt も試すときは「全画面で試す」を使ってください。";
        assert_eq!(text(false, false), windowed);
        // ふつうの窓では、Keyboard Lock が掛かっていても（抜けた直後など）、ふつうの窓の文
        assert_eq!(text(false, true), windowed);
        assert_eq!(
            text(true, true),
            "全画面です。マウスを画面の上へ動かすと出るボタンで戻れます。"
        );
        assert_eq!(
            text(true, false),
            "全画面ですが、このブラウザでは Win キーなどを受けられません。"
        );
    }
}
