//! `engine` の「押す」「離す」のテスト（⑥ レイヤーと修飾の状態、⑦ 欄への動きと直前のキー）。
//!
//! 期待値は、`docs/design.md` の 3.5節の例の表と、3.7節の例から、手で写した物。

use crate::chars::Layout;
use crate::engine::{Decided, Engine, FieldAction, PressOutcome};
use crate::keycode::{Keycode, Modifier};
use crate::sample::SAMPLE_VIL;
use crate::testkit::VilBuilder;
use crate::vil::parse;

type TestResult = Result<(), Box<dyn std::error::Error>>;

fn sample() -> Result<Engine, Box<dyn std::error::Error>> {
    Ok(Engine::new(parse(SAMPLE_VIL)?))
}

fn engine_of(builder: &VilBuilder) -> Result<Engine, Box<dyn std::error::Error>> {
    Ok(Engine::new(parse(&builder.build())?))
}

fn press_in(
    engine: &mut Engine,
    position: (usize, usize),
    layout: Layout,
) -> Result<PressOutcome, String> {
    engine
        .press(position.0, position.1, layout)
        .ok_or_else(|| format!("{position:?} が押せない"))
}

fn press(engine: &mut Engine, position: (usize, usize)) -> Result<PressOutcome, String> {
    press_in(engine, position, Layout::Jis)
}

/// 決まった中身を、見比べやすい文字にする（テストの側の書き方。実装の関数は使わない）。
fn describe(decided: &Decided) -> String {
    match decided.keycode() {
        None => "（割り当てなし）".to_owned(),
        Some(Keycode::No) => "KC_NO".to_owned(),
        Some(Keycode::Transparent) => "KC_TRNS".to_owned(),
        Some(Keycode::Char(key)) => format!("{key:?}"),
        Some(Keycode::Edit(key)) => format!("{key:?}"),
        Some(Keycode::Named(key)) => key.label().to_owned(),
        Some(Keycode::Modifier(modifier)) => format!("{modifier:?}"),
        Some(Keycode::Mo(layer)) => format!("MO({layer})"),
        Some(Keycode::Shifted { source, .. }) => source.clone(),
        Some(Keycode::Unsupported(text)) => text.clone(),
    }
}

fn resolved(engine: &Engine, position: (usize, usize)) -> String {
    describe(&engine.resolve(position.0, position.1))
}

const MO1: (usize, usize) = (3, 3);
const MO2: (usize, usize) = (7, 3);

#[test]
fn item6_design_table_layers_0_and_1() -> TestResult {
    let mut engine = sample()?;
    press(&mut engine, MO1)?;

    assert_eq!(engine.active_layers(), [0, 1]);
    assert_eq!(resolved(&engine, (0, 1)), "Digit('1')");
    assert_eq!(resolved(&engine, (1, 1)), "Letter('a')");
    assert_eq!(resolved(&engine, (2, 0)), "LShift");
    assert_eq!(resolved(&engine, (7, 0)), "KC_NO");
    assert_eq!(resolved(&engine, (3, 3)), "KC_NO");
    assert_eq!(resolved(&engine, (7, 3)), "MO(2)");
    assert_eq!(resolved(&engine, (4, 0)), "Minus");
    // 実際に押しても、同じキーコードに決まる
    assert_eq!(describe(&press(&mut engine, (0, 1))?.decided), "Digit('1')");
    assert_eq!(describe(&press(&mut engine, (7, 0))?.decided), "KC_NO");
    Ok(())
}

#[test]
fn item6_design_table_layers_0_and_2() -> TestResult {
    let mut engine = sample()?;
    press(&mut engine, MO2)?;

    assert_eq!(engine.active_layers(), [0, 2]);
    assert_eq!(resolved(&engine, (0, 0)), "Tab");
    assert_eq!(resolved(&engine, (7, 0)), "RShift");
    assert_eq!(resolved(&engine, (5, 0)), "Quote");
    assert_eq!(resolved(&engine, (4, 1)), "KC_NO");
    assert_eq!(resolved(&engine, (3, 3)), "MO(1)");
    assert_eq!(describe(&press(&mut engine, (0, 0))?.decided), "Tab");
    Ok(())
}

#[test]
fn item6_design_table_layers_0_1_and_2() -> TestResult {
    let mut engine = sample()?;
    press(&mut engine, MO1)?;
    press(&mut engine, MO2)?;

    assert_eq!(engine.active_layers(), [0, 1, 2]);
    assert_eq!(resolved(&engine, (0, 1)), "F1");
    assert_eq!(resolved(&engine, (0, 0)), "Grave");
    assert_eq!(resolved(&engine, (1, 0)), "Esc");
    assert_eq!(resolved(&engine, (4, 1)), "KC_NO");
    assert_eq!(resolved(&engine, (5, 0)), "RBracket");
    assert_eq!(resolved(&engine, (7, 0)), "KC_NO");
    assert_eq!(resolved(&engine, (6, 1)), "KC_UNDS");
    assert_eq!(resolved(&engine, (6, 0)), "LSFT(KC_RO)");
    assert_eq!(describe(&press(&mut engine, (0, 0))?.decided), "Grave");
    Ok(())
}

#[test]
fn item6_scene1_mo_is_released_by_the_remembered_keycode() -> TestResult {
    let mut engine = sample()?;
    assert_eq!(engine.active_layers(), [0]);

    let outcome = press(&mut engine, MO1)?;
    assert_eq!(describe(&outcome.decided), "MO(1)");
    assert_eq!(engine.active_layers(), [0, 1]);
    // (3,3) は、レイヤー1では KC_NO。離すときは、覚えた MO(1) で処理する
    engine.release(3, 3);
    assert_eq!(engine.active_layers(), [0]);
    Ok(())
}

#[test]
fn item6_scene2_trns_and_no_differ() -> TestResult {
    // (2,0) → (3,3) → (0,1)
    let mut engine = sample()?;
    press(&mut engine, (2, 0))?;
    press(&mut engine, MO1)?;
    let outcome = press(&mut engine, (0, 1))?;
    assert_eq!(describe(&outcome.decided), "Digit('1')");
    assert!(engine.shift());

    // (3,3) → (7,0) → (0,1)
    let mut engine = sample()?;
    press(&mut engine, MO1)?;
    assert_eq!(describe(&press(&mut engine, (7, 0))?.decided), "KC_NO");
    let outcome = press(&mut engine, (0, 1))?;
    assert_eq!(describe(&outcome.decided), "Digit('1')");
    assert!(!engine.shift());

    // (3,3) → (2,0) → (0,1)
    let mut engine = sample()?;
    press(&mut engine, MO1)?;
    assert_eq!(describe(&press(&mut engine, (2, 0))?.decided), "LShift");
    press(&mut engine, (0, 1))?;
    assert!(engine.shift());
    Ok(())
}

#[test]
fn item6_release_uses_the_remembered_keycode() -> TestResult {
    // (7,0) を押す（右 Shift）→ (3,3) を押す → (7,0) を離す: Shift が切れる
    let mut engine = sample()?;
    press(&mut engine, (7, 0))?;
    assert!(engine.modifier(Modifier::RShift));
    press(&mut engine, MO1)?;
    assert!(engine.shift());
    engine.release(7, 0);
    assert!(!engine.shift());

    // (3,3) → (7,0)（KC_NO）→ (3,3) を離す → (7,0) を離す: Shift は入っていない
    let mut engine = sample()?;
    press(&mut engine, MO1)?;
    press(&mut engine, (7, 0))?;
    engine.release(3, 3);
    assert!(!engine.shift());
    engine.release(7, 0);
    assert!(!engine.shift());
    assert_eq!(engine.active_layers(), [0]);
    Ok(())
}

#[test]
fn item6_two_mo_keys() -> TestResult {
    for (first, second) in [(MO1, MO2), (MO2, MO1)] {
        // 離す順は、押した順と同じ
        let mut engine = sample()?;
        press(&mut engine, first)?;
        press(&mut engine, second)?;
        assert_eq!(engine.active_layers(), [0, 1, 2], "{first:?} → {second:?}");
        engine.release(first.0, first.1);
        let rest = if first == MO1 { [0, 2] } else { [0, 1] };
        assert_eq!(engine.active_layers(), rest);
        engine.release(second.0, second.1);
        assert_eq!(engine.active_layers(), [0]);

        // 離す順は、押した順の逆
        let mut engine = sample()?;
        press(&mut engine, first)?;
        press(&mut engine, second)?;
        engine.release(second.0, second.1);
        let rest = if second == MO1 { [0, 2] } else { [0, 1] };
        assert_eq!(engine.active_layers(), rest);
        engine.release(first.0, first.1);
        assert_eq!(engine.active_layers(), [0]);
    }
    Ok(())
}

#[test]
fn item6_inactive_layers_are_skipped() -> TestResult {
    let mut engine = sample()?;
    press(&mut engine, MO2)?;

    // レイヤー1の KC_GRAVE にはならない
    assert_eq!(describe(&press(&mut engine, (0, 0))?.decided), "Tab");
    Ok(())
}

#[test]
fn item6_top_layer_of_ten() -> TestResult {
    let builder = VilBuilder::new(10)
        .key(0, 3, 3, "MO(9)")
        .key(0, 0, 1, "KC_Q")
        .key(9, 0, 1, "KC_Z");
    let mut engine = engine_of(&builder)?;

    press(&mut engine, (3, 3))?;
    assert_eq!(engine.active_layers(), [0, 9]);
    assert_eq!(
        describe(&press(&mut engine, (0, 1))?.decided),
        "Letter('z')"
    );
    engine.release(0, 1);
    engine.release(3, 3);
    assert_eq!(engine.active_layers(), [0]);
    assert_eq!(
        describe(&press(&mut engine, (0, 1))?.decided),
        "Letter('q')"
    );
    Ok(())
}

#[test]
fn item6_mo_and_the_number_of_layers() -> TestResult {
    let builder = VilBuilder::new(4)
        .key(0, 0, 0, "MO(3)")
        .key(0, 0, 1, "MO(4)")
        .key(0, 0, 2, "MO(31)")
        .key(0, 0, 3, "MO(0)");
    let mut engine = engine_of(&builder)?;

    // レイヤーの数ちょうどの手前（3）は、有効になる
    assert_eq!(press(&mut engine, (0, 0))?.last_key, "MO(3)");
    assert_eq!(engine.active_layers(), [0, 3]);
    engine.release(0, 0);
    assert_eq!(engine.active_layers(), [0]);

    // レイヤーの数以上は、押しても離しても、何も起きない
    for position in [(0, 1), (0, 2)] {
        press(&mut engine, position)?;
        assert_eq!(engine.active_layers(), [0], "{position:?}");
        engine.release(position.0, position.1);
        assert_eq!(engine.active_layers(), [0], "{position:?}");
    }

    // MO(0): レイヤー0は、いつも有効
    assert_eq!(press(&mut engine, (0, 3))?.last_key, "MO(0)");
    assert_eq!(engine.active_layers(), [0]);
    engine.release(0, 3);
    assert_eq!(engine.active_layers(), [0]);
    assert_eq!(describe(&engine.resolve(0, 0)), "MO(3)");
    Ok(())
}

#[test]
fn item6_two_mo_keys_for_the_same_layer_do_not_count() -> TestResult {
    let builder = VilBuilder::new(2)
        .key(0, 3, 3, "MO(1)")
        .key(0, 7, 3, "MO(1)");
    let mut engine = engine_of(&builder)?;

    press(&mut engine, (3, 3))?;
    press(&mut engine, (7, 3))?;
    assert_eq!(engine.active_layers(), [0, 1]);
    // 片方を離すと、もう片方を押したままでも、無効になる
    engine.release(3, 3);
    assert_eq!(engine.active_layers(), [0]);
    engine.release(7, 3);
    assert_eq!(engine.active_layers(), [0]);
    Ok(())
}

#[test]
fn item6_all_transparent_is_unassigned_and_minus_one_is_no() -> TestResult {
    let builder = VilBuilder::new(2)
        .key(0, 3, 3, "MO(1)")
        .raw(0, 0, 1, "-1")
        .raw(1, 0, 2, "-1")
        .key(0, 0, 2, "KC_A");
    let mut engine = engine_of(&builder)?;

    // (0,0) は、レイヤー0も KC_TRNS
    let outcome = press(&mut engine, (0, 0))?;
    assert_eq!(outcome.decided, Decided::Unassigned);
    assert_eq!(outcome.action, FieldAction::None);
    engine.release(0, 0);
    // キーのある位置の -1 は、KC_NO
    assert_eq!(describe(&press(&mut engine, (0, 1))?.decided), "KC_NO");
    // 上のレイヤーの -1 も KC_NO で、下の KC_A を隠す
    press(&mut engine, (3, 3))?;
    assert_eq!(describe(&press(&mut engine, (0, 2))?.decided), "KC_NO");
    assert_eq!(engine.active_layers(), [0, 1]);
    Ok(())
}

#[test]
fn item6_positions_without_keys_cannot_be_pressed() -> TestResult {
    let mut engine = sample()?;

    for position in [(0, 6), (7, 6), (2, 6), (5, 6), (8, 0), (0, 7)] {
        assert_eq!(
            engine.press(position.0, position.1, Layout::Jis),
            None,
            "{position:?}"
        );
        assert_eq!(engine.held(position.0, position.1), None);
    }
    Ok(())
}

/// 修飾キーと、いろいろな種類のキーを並べた .vil。
fn modifier_board() -> VilBuilder {
    VilBuilder::new(2)
        .key(0, 0, 0, "KC_LCTRL")
        .key(0, 0, 1, "KC_LSHIFT")
        .key(0, 0, 2, "KC_LALT")
        .key(0, 0, 3, "KC_LGUI")
        .key(0, 0, 4, "KC_RCTRL")
        .key(0, 0, 5, "KC_RSHIFT")
        .key(0, 1, 0, "KC_RALT")
        .key(0, 1, 1, "KC_RGUI")
        .key(0, 1, 2, "KC_LSHIFT")
        .key(0, 2, 1, "KC_C")
        .key(0, 2, 2, "KC_SPACE")
        .key(0, 2, 3, "KC_ENTER")
        .key(0, 2, 4, "KC_TAB")
        .key(0, 2, 5, "KC_BSPACE")
        .key(0, 3, 0, "KC_UNDS")
        .key(0, 3, 1, "USER00")
        .key(0, 3, 3, "MO(1)")
        .key(0, 3, 4, "KC_ESCAPE")
        .key(0, 3, 5, "KC_DELETE")
        .key(0, 4, 0, "KC_LEFT")
        .key(0, 4, 1, "KC_F1")
        .key(0, 4, 2, "KC_HENK")
        .key(0, 4, 3, "KC_NO")
        .key(0, 4, 5, "KC_1")
}

const MODIFIER_POSITIONS: [((usize, usize), Modifier); 8] = [
    ((0, 0), Modifier::LCtrl),
    ((0, 1), Modifier::LShift),
    ((0, 2), Modifier::LAlt),
    ((0, 3), Modifier::LGui),
    ((0, 4), Modifier::RCtrl),
    ((0, 5), Modifier::RShift),
    ((1, 0), Modifier::RAlt),
    ((1, 1), Modifier::RGui),
];

#[test]
fn item6_each_of_the_eight_modifiers() -> TestResult {
    for (position, modifier) in MODIFIER_POSITIONS {
        let mut engine = engine_of(&modifier_board())?;
        press(&mut engine, position)?;
        for other in Modifier::ALL {
            assert_eq!(engine.modifier(other), other == modifier, "{modifier:?}");
        }
        engine.release(position.0, position.1);
        for other in Modifier::ALL {
            assert!(!engine.modifier(other), "{modifier:?}");
        }
    }
    Ok(())
}

#[test]
fn item6_modifiers_do_not_count_and_left_and_right_are_separate() -> TestResult {
    // 同じ KC_LSHIFT を2つ押して、1つ離すと、切れる
    let mut engine = engine_of(&modifier_board())?;
    press(&mut engine, (0, 1))?;
    press(&mut engine, (1, 2))?;
    assert!(engine.shift());
    engine.release(0, 1);
    assert!(!engine.shift());
    engine.release(1, 2);
    assert!(!engine.shift());

    // 左と右を押して、左を離すと、入ったまま
    let mut engine = engine_of(&modifier_board())?;
    press(&mut engine, (0, 1))?;
    press(&mut engine, (0, 5))?;
    engine.release(0, 1);
    assert!(engine.shift());
    assert!(!engine.modifier(Modifier::LShift));
    assert!(engine.modifier(Modifier::RShift));
    engine.release(0, 5);
    assert!(!engine.shift());
    Ok(())
}

/// 3.7節の例の1行。押したままにする位置・押す位置・直前のキー・欄への動き。
type Example = (
    &'static [(usize, usize)],
    (usize, usize),
    &'static str,
    FieldAction,
);

fn check_examples(layout: Layout, examples: &[Example]) -> TestResult {
    for (holds, target, last_key, action) in examples {
        let mut engine = sample()?;
        for hold in *holds {
            press_in(&mut engine, *hold, layout)?;
        }
        let outcome = press_in(&mut engine, *target, layout)?;
        assert_eq!(outcome.last_key, *last_key, "{holds:?} → {target:?}");
        assert_eq!(outcome.action, *action, "{holds:?} → {target:?}");
    }
    Ok(())
}

#[test]
fn item7_design_examples_in_jis() -> TestResult {
    let examples: [Example; 17] = [
        (&[], (0, 1), "Q → q", FieldAction::Insert('q')),
        (&[(2, 0)], (0, 1), "Shift+Q → Q", FieldAction::Insert('Q')),
        (&[(3, 3)], (0, 1), "1 → 1", FieldAction::Insert('1')),
        (
            &[(2, 0), (3, 3)],
            (0, 1),
            "Shift+1 → !",
            FieldAction::Insert('!'),
        ),
        (&[(2, 0), (3, 3)], (4, 1), "Shift+0", FieldAction::None),
        (&[(3, 3)], (0, 0), "半角/全角", FieldAction::None),
        (&[(3, 0)], (2, 3), "Ctrl+C", FieldAction::None),
        (&[(3, 0), (2, 0)], (2, 3), "Ctrl+Shift+C", FieldAction::None),
        (&[(3, 0)], (7, 5), "Ctrl+Enter", FieldAction::None),
        (&[], (1, 0), "Esc", FieldAction::None),
        (&[(2, 0)], (0, 0), "Shift+Tab", FieldAction::Insert('\t')),
        (&[], (2, 0), "Shift", FieldAction::None),
        (&[(3, 0)], (2, 0), "Ctrl+Shift", FieldAction::None),
        (&[], (3, 3), "MO(1)", FieldAction::None),
        (&[(3, 3)], (7, 0), "（割り当てなし）", FieldAction::None),
        (&[(7, 3)], (6, 1), "Shift+- → =", FieldAction::Insert('=')),
        (&[(7, 3)], (6, 0), "Shift+\\ → _", FieldAction::Insert('_')),
    ];
    check_examples(Layout::Jis, &examples)
}

#[test]
fn item7_design_examples_in_us() -> TestResult {
    let examples: [Example; 3] = [
        (&[(7, 3)], (6, 1), "Shift+- → _", FieldAction::Insert('_')),
        (&[(7, 3)], (6, 0), "Shift+Ro", FieldAction::None),
        (&[(3, 3)], (0, 0), "` → `", FieldAction::Insert('`')),
    ];
    check_examples(Layout::Us, &examples)
}

#[test]
fn item7_scene2_types_exclamation_one_exclamation() -> TestResult {
    let mut engine = sample()?;
    press(&mut engine, (2, 0))?;
    press(&mut engine, MO1)?;
    assert_eq!(press(&mut engine, (0, 1))?.action, FieldAction::Insert('!'));

    let mut engine = sample()?;
    press(&mut engine, MO1)?;
    press(&mut engine, (7, 0))?;
    assert_eq!(press(&mut engine, (0, 1))?.action, FieldAction::Insert('1'));

    let mut engine = sample()?;
    press(&mut engine, MO1)?;
    press(&mut engine, (2, 0))?;
    assert_eq!(press(&mut engine, (0, 1))?.action, FieldAction::Insert('!'));
    Ok(())
}

#[test]
fn item7_either_shift_gives_the_shifted_character() -> TestResult {
    for shift in [(0, 1), (0, 5)] {
        let mut engine = engine_of(&modifier_board())?;
        press(&mut engine, shift)?;
        let outcome = press(&mut engine, (2, 1))?;
        assert_eq!(outcome.action, FieldAction::Insert('C'), "{shift:?}");
        assert_eq!(outcome.last_key, "Shift+C → C");
        let outcome = press(&mut engine, (4, 5))?;
        assert_eq!(outcome.action, FieldAction::Insert('!'));
    }
    Ok(())
}

#[test]
fn item7_ctrl_alt_and_gui_block_the_field_but_shift_does_not() -> TestResult {
    let keys = [(2, 1), (2, 2), (2, 3), (2, 4), (2, 5)];
    // Ctrl・Alt・GUI（左と右）: どのキーも、欄を変えない
    for modifier in [(0, 0), (0, 2), (0, 3), (0, 4), (1, 0), (1, 1)] {
        let mut engine = engine_of(&modifier_board())?;
        press(&mut engine, modifier)?;
        for key in keys {
            let outcome = press(&mut engine, key)?;
            assert_eq!(outcome.action, FieldAction::None, "{modifier:?} + {key:?}");
        }
    }
    // Shift だけなら、変える
    let mut engine = engine_of(&modifier_board())?;
    press(&mut engine, (0, 1))?;
    let actions: Vec<FieldAction> = keys
        .iter()
        .map(|key| press(&mut engine, *key).map(|outcome| outcome.action))
        .collect::<Result<_, _>>()?;
    assert_eq!(
        actions,
        [
            FieldAction::Insert('C'),
            FieldAction::Insert(' '),
            FieldAction::Insert('\n'),
            FieldAction::Insert('\t'),
            FieldAction::DeleteLast,
        ]
    );
    // 修飾なしでも、変える
    let mut engine = engine_of(&modifier_board())?;
    assert_eq!(press(&mut engine, (2, 2))?.action, FieldAction::Insert(' '));
    assert_eq!(press(&mut engine, (2, 5))?.action, FieldAction::DeleteLast);
    Ok(())
}

#[test]
fn item7_modifier_order_is_ctrl_shift_alt_gui() -> TestResult {
    let mut engine = engine_of(&modifier_board())?;
    // GUI・Alt・Shift・Ctrl の順に押しても、並びは決まった順
    assert_eq!(press(&mut engine, (0, 3))?.last_key, "GUI");
    assert_eq!(press(&mut engine, (0, 2))?.last_key, "Alt+GUI");
    assert_eq!(press(&mut engine, (0, 1))?.last_key, "Shift+Alt+GUI");
    assert_eq!(press(&mut engine, (0, 0))?.last_key, "Ctrl+Shift+Alt+GUI");
    let outcome = press(&mut engine, (2, 1))?;
    assert_eq!(outcome.last_key, "Ctrl+Shift+Alt+GUI+C");
    assert_eq!(outcome.action, FieldAction::None);
    Ok(())
}

#[test]
fn item7_shifted_keycodes_do_not_change_the_modifiers() -> TestResult {
    let mut engine = engine_of(&modifier_board())?;
    let outcome = press(&mut engine, (3, 0))?;
    assert_eq!(outcome.last_key, "Shift+- → =");
    assert_eq!(outcome.action, FieldAction::Insert('='));
    assert!(!engine.shift());
    for modifier in Modifier::ALL {
        assert!(!engine.modifier(modifier));
    }
    engine.release(3, 0);
    assert!(!engine.shift());

    // Ctrl を押したまま KC_UNDS
    press(&mut engine, (0, 0))?;
    let outcome = press(&mut engine, (3, 0))?;
    assert_eq!(outcome.last_key, "Ctrl+Shift+-");
    assert_eq!(outcome.action, FieldAction::None);
    assert!(!engine.shift());
    Ok(())
}

#[test]
fn item7_unsupported_key_changes_nothing() -> TestResult {
    let mut engine = engine_of(&modifier_board())?;
    let outcome = press(&mut engine, (3, 1))?;

    assert_eq!(outcome.last_key, "USER00（未対応）");
    assert_eq!(outcome.action, FieldAction::None);
    assert_eq!(engine.active_layers(), [0]);
    for modifier in Modifier::ALL {
        assert!(!engine.modifier(modifier));
    }
    engine.release(3, 1);
    assert_eq!(engine.active_layers(), [0]);
    Ok(())
}

#[test]
fn item7_keys_that_do_not_change_the_field() -> TestResult {
    let table = [
        ((3, 4), "Esc"),
        ((3, 5), "Del"),
        ((4, 0), "←"),
        ((4, 1), "F1"),
        ((4, 2), "変換"),
        ((0, 0), "Ctrl"),
        ((3, 3), "MO(1)"),
        ((4, 3), "（割り当てなし）"),
        // レイヤー0まで KC_TRNS（割り当てなし）
        ((4, 4), "（割り当てなし）"),
    ];
    for (position, last_key) in table {
        let mut engine = engine_of(&modifier_board())?;
        let outcome = press(&mut engine, position)?;
        assert_eq!(outcome.action, FieldAction::None, "{position:?}");
        assert_eq!(outcome.last_key, last_key, "{position:?}");
    }
    Ok(())
}
