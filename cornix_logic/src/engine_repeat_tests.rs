//! `engine` の押し続け・二重の押す離す・全部離すのテスト（⑧。B-22〜B-24）。

use crate::chars::Layout;
use crate::engine::{Engine, FieldAction, PressOutcome};
use crate::keycode::Modifier;
use crate::sample::SAMPLE_VIL;
use crate::testkit::VilBuilder;
use crate::vil::parse;

type TestResult = Result<(), Box<dyn std::error::Error>>;

fn sample() -> Result<Engine, Box<dyn std::error::Error>> {
    Ok(Engine::new(parse(SAMPLE_VIL)?))
}

fn press(engine: &mut Engine, position: (usize, usize)) -> Result<PressOutcome, String> {
    engine
        .press(position.0, position.1, Layout::Jis)
        .ok_or_else(|| format!("{position:?} が押せない"))
}

fn repeat(engine: &Engine, position: (usize, usize)) -> FieldAction {
    engine.repeat(position.0, position.1, Layout::Jis)
}

/// 欄の代わり（テストの側の書き方。`text` モジュールは使わない）。
fn apply(field: &mut String, action: FieldAction) {
    match action {
        FieldAction::Insert(typed) => field.push(typed),
        FieldAction::DeleteLast => {
            field.pop();
        }
        FieldAction::None => {}
    }
}

#[test]
fn item8_repeat_types_the_same_character_again() -> TestResult {
    let mut engine = sample()?;
    let mut field = String::new();

    let outcome = press(&mut engine, (0, 1))?;
    apply(&mut field, outcome.action);
    apply(&mut field, repeat(&engine, (0, 1)));
    apply(&mut field, repeat(&engine, (0, 1)));

    assert_eq!(field, "qqq");
    // 押し続けは、欄への動きだけを返す（「直前のキー」は、押したときの `Q → q` のまま）
    assert_eq!(outcome.last_key, "Q → q");
    assert_eq!(engine.active_layers(), [0]);
    assert_eq!(engine.held_positions(), [(0, 1)]);
    Ok(())
}

#[test]
fn item8_repeat_uses_the_keycode_decided_at_press() -> TestResult {
    let mut engine = sample()?;
    press(&mut engine, (0, 1))?;
    press(&mut engine, (3, 3))?;

    // レイヤー1が有効になっても、押したときの KC_Q のまま（`1` ではない）
    assert_eq!(repeat(&engine, (0, 1)), FieldAction::Insert('q'));
    Ok(())
}

#[test]
fn item8_repeat_uses_the_current_modifiers() -> TestResult {
    let mut engine = sample()?;
    press(&mut engine, (2, 0))?;
    assert_eq!(press(&mut engine, (0, 1))?.action, FieldAction::Insert('Q'));
    assert_eq!(repeat(&engine, (0, 1)), FieldAction::Insert('Q'));
    engine.release(2, 0);

    assert_eq!(repeat(&engine, (0, 1)), FieldAction::Insert('q'));
    Ok(())
}

#[test]
fn item8_repeat_uses_the_given_layout() -> TestResult {
    // (4,0) は KC_LBRACKET: JIS では @、US では [（design 3.2節）
    let mut engine = sample()?;
    engine.press(4, 0, Layout::Us).ok_or("(4,0) が押せない")?;
    assert_eq!(engine.repeat(4, 0, Layout::Us), FieldAction::Insert('['));
    assert_eq!(engine.repeat(4, 0, Layout::Jis), FieldAction::Insert('@'));

    // (7,3) を押したままの (6,1) は KC_UNDS: JIS では =、US では _
    let mut engine = sample()?;
    engine.press(7, 3, Layout::Us).ok_or("(7,3) が押せない")?;
    engine.press(6, 1, Layout::Us).ok_or("(6,1) が押せない")?;
    assert_eq!(engine.repeat(6, 1, Layout::Us), FieldAction::Insert('_'));
    assert_eq!(engine.repeat(6, 1, Layout::Jis), FieldAction::Insert('='));

    // (3,3) を押したままの (0,0) は KC_GRAVE: JIS では文字なし、US では `
    let mut engine = sample()?;
    engine.press(3, 3, Layout::Us).ok_or("(3,3) が押せない")?;
    engine.press(0, 0, Layout::Us).ok_or("(0,0) が押せない")?;
    assert_eq!(engine.repeat(0, 0, Layout::Us), FieldAction::Insert('`'));
    assert_eq!(engine.repeat(0, 0, Layout::Jis), FieldAction::None);
    Ok(())
}

#[test]
fn item8_repeat_of_bksp_and_with_ctrl() -> TestResult {
    let mut engine = sample()?;
    let mut field = "abc".to_owned();
    // (7,4) は KC_BSPACE
    apply(&mut field, press(&mut engine, (7, 4))?.action);
    apply(&mut field, repeat(&engine, (7, 4)));
    assert_eq!(field, "a");

    // Ctrl（(3,0)）が入っている間の押し続けは、欄を変えない
    press(&mut engine, (3, 0))?;
    assert_eq!(repeat(&engine, (7, 4)), FieldAction::None);
    press(&mut engine, (0, 1))?;
    assert_eq!(repeat(&engine, (0, 1)), FieldAction::None);
    engine.release(3, 0);
    assert_eq!(repeat(&engine, (0, 1)), FieldAction::Insert('q'));
    assert_eq!(repeat(&engine, (7, 4)), FieldAction::DeleteLast);
    Ok(())
}

#[test]
fn item8_repeat_of_shifted_and_edit_keys() -> TestResult {
    let builder = VilBuilder::new(1)
        .key(0, 0, 0, "KC_UNDS")
        .key(0, 0, 1, "KC_SPACE")
        .key(0, 0, 2, "KC_ENTER")
        .key(0, 0, 3, "KC_TAB")
        .key(0, 0, 4, "LSFT(KC_F1)");
    let mut engine = Engine::new(parse(&builder.build())?);
    for position in [(0, 0), (0, 1), (0, 2), (0, 3), (0, 4)] {
        press(&mut engine, position)?;
    }

    assert_eq!(repeat(&engine, (0, 0)), FieldAction::Insert('='));
    assert_eq!(repeat(&engine, (0, 1)), FieldAction::Insert(' '));
    assert_eq!(repeat(&engine, (0, 2)), FieldAction::Insert('\n'));
    assert_eq!(repeat(&engine, (0, 3)), FieldAction::Insert('\t'));
    // Shift 付きでも、中が名前だけのキーなら、欄は変えない
    assert_eq!(repeat(&engine, (0, 4)), FieldAction::None);
    Ok(())
}

#[test]
fn item8_repeat_changes_nothing_for_other_keys() -> TestResult {
    let mut engine = sample()?;
    // MO(1)・Shift・Esc・（MO(1) のあとの）KC_NO の位置
    for position in [(3, 3), (2, 0), (1, 0), (7, 0)] {
        press(&mut engine, position)?;
    }
    let layers = engine.active_layers();
    let held = engine.held_positions();

    for position in [(3, 3), (2, 0), (1, 0), (7, 0)] {
        assert_eq!(repeat(&engine, position), FieldAction::None, "{position:?}");
    }
    // 押されていない位置（文字のキーでも）と、キーの無い位置の押し続けは、無視する
    assert_eq!(repeat(&engine, (0, 1)), FieldAction::None);
    assert_eq!(repeat(&engine, (0, 6)), FieldAction::None);
    assert_eq!(repeat(&engine, (9, 9)), FieldAction::None);
    assert_eq!(engine.active_layers(), layers);
    assert_eq!(engine.active_layers(), [0, 1]);
    assert_eq!(engine.held_positions(), held);
    assert!(engine.shift());
    Ok(())
}

#[test]
fn item8_double_press_and_stray_release_are_ignored() -> TestResult {
    let mut engine = sample()?;
    let mut field = String::new();

    apply(&mut field, press(&mut engine, (0, 1))?.action);
    // 2回目は無視（結果が返らないので、欄も「直前のキー」も変わらない）
    assert_eq!(engine.press(0, 1, Layout::Jis), None);
    assert_eq!(field, "q");
    assert_eq!(engine.held_positions(), [(0, 1)]);
    // 1回離せば、離れる
    engine.release(0, 1);
    assert_eq!(engine.held_positions(), []);

    // 押していない位置を離しても、何も変わらない
    press(&mut engine, (3, 3))?;
    press(&mut engine, (2, 0))?;
    engine.release(0, 1);
    engine.release(7, 3);
    engine.release(0, 6);
    engine.release(9, 9);
    assert_eq!(engine.active_layers(), [0, 1]);
    assert!(engine.shift());
    assert_eq!(engine.held_positions(), [(3, 3), (2, 0)]);

    // MO を2回押しても、1回離せば無効になる
    assert_eq!(engine.press(3, 3, Layout::Jis), None);
    engine.release(3, 3);
    assert_eq!(engine.active_layers(), [0]);
    Ok(())
}

#[test]
fn item8_release_all() -> TestResult {
    let mut engine = sample()?;
    // MO(1)・MO(2)・左 Shift・Ctrl・文字のキー
    for position in [(3, 3), (7, 3), (2, 0), (3, 0)] {
        press(&mut engine, position)?;
    }
    let outcome = press(&mut engine, (1, 1))?;
    assert_eq!(engine.active_layers(), [0, 1, 2]);

    engine.release_all();

    assert_eq!(engine.active_layers(), [0]);
    for modifier in Modifier::ALL {
        assert!(!engine.modifier(modifier), "{modifier:?}");
    }
    assert_eq!(engine.held_positions(), []);
    assert_eq!(engine.held(3, 3), None);
    // 欄と「直前のキー」は、engine の外で持つ（返してあった値は、そのまま）
    // (1,1) は、レイヤー2では KC_F6
    assert_eq!(outcome.last_key, "Ctrl+Shift+F6");

    // そのあと、同じ位置をまた押せる
    assert_eq!(press(&mut engine, (1, 1))?.action, FieldAction::Insert('a'));
    assert_eq!(press(&mut engine, (3, 3))?.last_key, "MO(1)");
    assert_eq!(engine.active_layers(), [0, 1]);
    // 離し残しがあっても、あとの release は何も壊さない
    engine.release_all();
    engine.release(3, 3);
    assert_eq!(engine.active_layers(), [0]);
    Ok(())
}
