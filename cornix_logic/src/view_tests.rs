//! `view` のテスト（⑬。B-12〜B-19）。期待値は、`docs/test-items.md` の⑬と、見本（design 3.5節）から手で写した物。

use crate::chars::Layout;
use crate::engine::Engine;
use crate::sample::SAMPLE_VIL;
use crate::testkit::VilBuilder;
use crate::view::{Cell, Shown, Tone, View, render};
use crate::vil::parse;

type TestResult = Result<(), Box<dyn std::error::Error>>;

fn sample() -> Result<Engine, Box<dyn std::error::Error>> {
    Ok(Engine::new(parse(SAMPLE_VIL)?))
}

fn press(engine: &mut Engine, position: (usize, usize)) -> Result<(), String> {
    engine
        .press(position.0, position.1, Layout::Jis)
        .map(|_| ())
        .ok_or_else(|| format!("{position:?} が押せない"))
}

fn cell(view: &View, position: (usize, usize)) -> Result<&Cell, String> {
    view.cell(position.0, position.1)
        .ok_or_else(|| format!("{position:?} が図に無い"))
}

/// 下の文字・上の文字・濃さ。
fn legend(view: &View, position: (usize, usize)) -> Result<(&str, &str, Tone), String> {
    let cell = cell(view, position)?;
    Ok((cell.lower.as_str(), cell.upper.as_str(), cell.tone))
}

#[test]
fn item13_auto_with_nothing_pressed() -> TestResult {
    let engine = sample()?;
    let jis = render(&engine, Layout::Jis, Shown::Auto, true);
    let us = render(&engine, Layout::Us, Shown::Auto, true);

    assert_eq!(jis.cells.len(), 50);
    assert_eq!(legend(&jis, (0, 1))?, ("Q", "", Tone::Normal));
    assert_eq!(legend(&jis, (4, 0))?, ("@", "`", Tone::Normal));
    assert_eq!(legend(&us, (4, 0))?, ("[", "{", Tone::Normal));
    assert_eq!(legend(&jis, (7, 3))?, ("MO(2)", "", Tone::Normal));
    assert_eq!(legend(&jis, (1, 0))?, ("Esc", "", Tone::Normal));
    assert_eq!(legend(&jis, (7, 2))?, ("変換", "", Tone::Normal));
    assert_eq!(legend(&jis, (2, 0))?, ("LShift", "", Tone::Normal));
    assert_eq!(legend(&jis, (3, 5))?, ("Space", "", Tone::Normal));
    // JIS の ろ のキー: 下が \、上が _。US では文字が無いので、名前
    assert_eq!(legend(&jis, (6, 0))?, ("\\", "_", Tone::Normal));
    assert_eq!(legend(&us, (6, 0))?, ("Ro", "", Tone::Normal));
    assert_eq!(jis.active_layers, [0]);
    assert!(jis.cells.iter().all(|cell| !cell.pressed));
    // 何も押していないレイヤー0には、薄い物が無い
    assert!(jis.cells.iter().all(|cell| cell.tone != Tone::Dim));
    Ok(())
}

#[test]
fn item13_auto_while_mo1_is_held() -> TestResult {
    let mut engine = sample()?;
    press(&mut engine, (3, 3))?;
    let view = render(&engine, Layout::Jis, Shown::Auto, true);

    assert_eq!(legend(&view, (0, 1))?, ("1", "!", Tone::Normal));
    assert_eq!(legend(&view, (1, 1))?, ("A", "", Tone::Dim));
    assert_eq!(legend(&view, (7, 0))?, ("", "", Tone::Empty));
    // 押されているキーは、押したときのキーコード（レイヤー0の MO(1)）を、ふつうの濃さで
    assert_eq!(legend(&view, (3, 3))?, ("MO(1)", "", Tone::Normal));
    assert!(cell(&view, (3, 3))?.pressed);
    assert!(!cell(&view, (0, 1))?.pressed);
    // JIS の KC_0 は、Shift 側が「なし」なので、下だけ
    assert_eq!(legend(&view, (4, 1))?, ("0", "", Tone::Normal));
    assert_eq!(view.active_layers, [0, 1]);

    // 下のレイヤーから来たキーを押している間は、ふつうの濃さで出る
    press(&mut engine, (1, 1))?;
    let view = render(&engine, Layout::Jis, Shown::Auto, true);
    assert_eq!(legend(&view, (1, 1))?, ("A", "", Tone::Normal));
    assert!(cell(&view, (1, 1))?.pressed);
    Ok(())
}

#[test]
fn item13_auto_while_mo1_and_mo2_are_held() -> TestResult {
    let mut engine = sample()?;
    press(&mut engine, (3, 3))?;
    press(&mut engine, (7, 3))?;
    let view = render(&engine, Layout::Jis, Shown::Auto, true);

    assert_eq!(legend(&view, (0, 1))?, ("F1", "", Tone::Normal));
    assert_eq!(legend(&view, (0, 0))?, ("半角/全角", "", Tone::Dim));
    assert_eq!(legend(&view, (1, 0))?, ("Esc", "", Tone::Dim));
    assert_eq!(legend(&view, (4, 1))?, ("", "", Tone::Empty));
    assert_eq!(view.active_layers, [0, 1, 2]);
    // US では、レイヤー1の KC_GRAVE は文字のキー
    let us = render(&engine, Layout::Us, Shown::Auto, true);
    assert_eq!(legend(&us, (0, 0))?, ("`", "~", Tone::Dim));
    Ok(())
}

#[test]
fn item13_a_chosen_layer_is_shown_as_it_is() -> TestResult {
    let mut engine = sample()?;
    press(&mut engine, (0, 1))?;
    let view = render(&engine, Layout::Jis, Shown::Layer(1), true);

    assert_eq!(legend(&view, (1, 1))?, ("▽", "", Tone::Normal));
    assert_eq!(legend(&view, (3, 3))?, ("", "", Tone::Empty));
    assert_eq!(legend(&view, (7, 0))?, ("", "", Tone::Empty));
    assert_eq!(legend(&view, (0, 1))?, ("1", "!", Tone::Normal));
    assert!(view.cells.iter().all(|cell| cell.tone != Tone::Dim));
    // 押されている印は、選んでいるレイヤーと関係なく付く
    assert!(cell(&view, (0, 1))?.pressed);
    assert!(!cell(&view, (1, 1))?.pressed);
    // 打った結果（有効なレイヤー）は、変わらない
    assert_eq!(view.active_layers, [0]);

    // いちばん上のレイヤー（全部 KC_TRNS）と、いちばん下のレイヤー
    let top = render(&engine, Layout::Jis, Shown::Layer(9), true);
    assert_eq!(legend(&top, (0, 0))?, ("▽", "", Tone::Normal));
    let bottom = render(&engine, Layout::Jis, Shown::Layer(0), true);
    assert_eq!(legend(&bottom, (0, 1))?, ("Q", "", Tone::Normal));
    assert_eq!(legend(&bottom, (3, 3))?, ("MO(1)", "", Tone::Normal));
    Ok(())
}

#[test]
fn item13_shifted_keycodes_on_layer_2() -> TestResult {
    let engine = sample()?;
    let jis = render(&engine, Layout::Jis, Shown::Layer(2), true);
    let us = render(&engine, Layout::Us, Shown::Layer(2), true);

    assert_eq!(legend(&jis, (6, 1))?, ("=", "", Tone::Normal));
    assert_eq!(legend(&us, (6, 1))?, ("_", "", Tone::Normal));
    assert_eq!(legend(&jis, (6, 0))?, ("_", "", Tone::Normal));
    assert_eq!(legend(&us, (6, 0))?, ("LSFT(KC_RO)", "", Tone::Normal));
    Ok(())
}

#[test]
fn item13_unsupported_keycodes() -> TestResult {
    let builder = VilBuilder::new(2)
        .key(0, 0, 0, "USER00")
        .raw(0, 0, 1, "5")
        .key(0, 3, 3, "MO(1)")
        .key(1, 0, 2, "LT1(KC_SPACE)");
    let mut engine = Engine::new(parse(&builder.build())?);
    for shown in [Shown::Auto, Shown::Layer(0)] {
        let view = render(&engine, Layout::Jis, shown, true);
        assert_eq!(legend(&view, (0, 0))?, ("USER00", "", Tone::Unsupported));
        assert_eq!(legend(&view, (0, 1))?, ("5", "", Tone::Unsupported));
    }
    // 下のレイヤーから来た未対応も、「未対応」のまま
    press(&mut engine, (3, 3))?;
    let view = render(&engine, Layout::Jis, Shown::Auto, true);
    assert_eq!(legend(&view, (0, 0))?, ("USER00", "", Tone::Unsupported));
    assert_eq!(
        legend(&view, (0, 2))?,
        ("LT1(KC_SPACE)", "", Tone::Unsupported)
    );
    Ok(())
}

#[test]
fn item13_knobs_always_say_unsupported() -> TestResult {
    let builder = VilBuilder::new(2)
        .key(0, 2, 6, "KC_A")
        .key(0, 5, 6, "KC_MUTE")
        .key(1, 2, 6, "KC_NO");
    let engine = Engine::new(parse(&builder.build())?);
    for shown in [Shown::Auto, Shown::Layer(0), Shown::Layer(1)] {
        for host_labels in [true, false] {
            let view = render(&engine, Layout::Jis, shown, host_labels);
            for knob in [(2, 6), (5, 6)] {
                let cell = cell(&view, knob)?;
                assert_eq!(cell.lower, "未対応", "{knob:?}");
                assert_eq!(cell.upper, "");
                assert_eq!(cell.tone, Tone::Unsupported);
                assert!(!cell.pressed);
                assert_eq!(cell.host_label, None);
            }
        }
    }
    // キーの無い位置は、図に無い
    let view = render(&engine, Layout::Jis, Shown::Auto, true);
    assert!(view.cell(0, 6).is_none());
    assert!(view.cell(7, 6).is_none());
    Ok(())
}

#[test]
fn item13_host_labels() -> TestResult {
    let engine = sample()?;
    let shown = render(&engine, Layout::Jis, Shown::Auto, true);

    assert_eq!(cell(&shown, (0, 1))?.host_label.as_deref(), Some("Q"));
    assert_eq!(
        cell(&shown, (3, 2))?.host_label.as_deref(),
        Some("手元のキー無し")
    );
    // 2つの code が当たる位置（design 3.4節の決め方6）
    assert_eq!(
        cell(&shown, (1, 0))?.host_label.as_deref(),
        Some("英数／Esc")
    );
    assert_eq!(
        cell(&shown, (7, 4))?.host_label.as_deref(),
        Some("カタカナひらがな／Backspace")
    );

    let hidden = render(&engine, Layout::Jis, Shown::Auto, false);
    assert_eq!(cell(&hidden, (0, 1))?.host_label, None);
    assert_eq!(cell(&hidden, (3, 2))?.host_label, None);
    assert!(hidden.cells.iter().all(|cell| cell.host_label.is_none()));
    // 刻印は、出す・出さないで変わらない
    assert_eq!(legend(&hidden, (0, 1))?, ("Q", "", Tone::Normal));
    Ok(())
}

#[test]
fn item13_switching_the_layout_changes_only_the_legends() -> TestResult {
    let mut engine = sample()?;
    press(&mut engine, (3, 3))?;
    press(&mut engine, (4, 0))?;
    let jis = render(&engine, Layout::Jis, Shown::Auto, true);
    let us = render(&engine, Layout::Us, Shown::Auto, true);

    // (4,0) は、MO(1) の間は KC_MINUS。押されている
    assert_eq!(legend(&jis, (4, 0))?, ("-", "=", Tone::Normal));
    assert_eq!(legend(&us, (4, 0))?, ("-", "_", Tone::Normal));
    assert_eq!(jis.active_layers, us.active_layers);
    for (a, b) in jis.cells.iter().zip(us.cells.iter()) {
        assert_eq!(a.position, b.position);
        assert_eq!(a.pressed, b.pressed);
        assert_eq!(a.tone, b.tone);
        assert_eq!(a.host_label, b.host_label);
    }
    engine.release(4, 0);
    engine.release(3, 3);
    let jis = render(&engine, Layout::Jis, Shown::Auto, true);
    let us = render(&engine, Layout::Us, Shown::Auto, true);
    assert_eq!(legend(&jis, (4, 0))?, ("@", "`", Tone::Normal));
    assert_eq!(legend(&us, (4, 0))?, ("[", "{", Tone::Normal));
    Ok(())
}

/// 有効なレイヤーの文（B-19）。期待値は、design の B-19 の形から手で書いた物。
#[test]
fn b19_active_layers_text() -> TestResult {
    let mut engine = sample()?;
    let text =
        |engine: &Engine| render(engine, Layout::Jis, Shown::Auto, true).active_layers_text();
    assert_eq!(text(&engine), "有効なレイヤー: 0");
    press(&mut engine, (7, 3))?;
    assert_eq!(text(&engine), "有効なレイヤー: 0・2");
    press(&mut engine, (3, 3))?;
    assert_eq!(text(&engine), "有効なレイヤー: 0・1・2");
    Ok(())
}
