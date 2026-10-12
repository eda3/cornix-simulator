//! `vil` の、読めない形（④）と、知らせの数（⑤）のテスト。

use crate::testkit::VilBuilder;
use crate::vil::{Notices, VilError, parse};

const GOOD_ROW: &str = r#"["KC_TRNS","KC_TRNS","KC_TRNS","KC_TRNS","KC_TRNS","KC_TRNS",-1]"#;

/// 行を並べて、1レイヤーの JSON にする。
fn layer_of(rows: &[&str]) -> String {
    format!("[{}]", rows.join(","))
}

fn good_layer() -> String {
    layer_of(&[GOOD_ROW; 8])
}

/// レイヤーを並べて、.vil にする。
fn vil_of(layers: &[String]) -> String {
    format!(r#"{{"version":1,"layout":[{}]}}"#, layers.join(","))
}

/// 列の数を変えた行。
fn row_with_cols(cols: usize) -> String {
    format!("[{}]", vec!["\"KC_TRNS\""; cols].join(","))
}

fn error_of(text: &str) -> Result<VilError, String> {
    match parse(text) {
        Ok(_) => Err(format!("読めてしまった: {text}")),
        Err(error) => Ok(error),
    }
}

#[test]
fn item4_not_json() -> Result<(), String> {
    for text in ["", "abc", "{", "   ", "\u{feff}", "{\"layout\":[}"] {
        assert_eq!(error_of(text)?, VilError::NotJson, "{text:?}");
    }
    Ok(())
}

#[test]
fn item4_not_object() -> Result<(), String> {
    for text in ["[]", "5", "null", "\"abc\"", "true"] {
        assert_eq!(error_of(text)?, VilError::NotObject, "{text:?}");
    }
    Ok(())
}

#[test]
fn item4_layout_missing_or_not_array_or_empty() -> Result<(), String> {
    assert_eq!(error_of("{}")?, VilError::NoLayout);
    assert_eq!(error_of(r#"{"uid":0,"Layout":[]}"#)?, VilError::NoLayout);
    for layout in ["5", "\"abc\"", "{}", "null"] {
        let text = format!(r#"{{"layout":{layout}}}"#);
        assert_eq!(error_of(&text)?, VilError::LayoutNotArray, "{text}");
    }
    assert_eq!(error_of(r#"{"layout":[]}"#)?, VilError::NoLayers);
    Ok(())
}

#[test]
fn item4_layer_not_array() -> Result<(), String> {
    for bad in ["5", "\"abc\"", "{}", "null"] {
        let text = vil_of(&[bad.to_owned()]);
        assert_eq!(
            error_of(&text)?,
            VilError::LayerNotArray { layer: 0 },
            "{text}"
        );
    }
    Ok(())
}

#[test]
fn item4_bad_row_count() -> Result<(), String> {
    for rows in [7, 9, 0] {
        let text = vil_of(&[layer_of(&vec![GOOD_ROW; rows])]);
        assert_eq!(
            error_of(&text)?,
            VilError::BadRowCount {
                layer: 0,
                found: rows
            }
        );
    }
    Ok(())
}

#[test]
fn item4_row_not_array() -> Result<(), String> {
    let mut rows = [GOOD_ROW; 8];
    rows[3] = "5";
    let text = vil_of(&[layer_of(&rows)]);
    assert_eq!(error_of(&text)?, VilError::RowNotArray { layer: 0, row: 3 });
    Ok(())
}

#[test]
fn item4_row_errors_report_the_layer() -> Result<(), String> {
    // レイヤー0は正しく、レイヤー2の行だけが壊れている
    let mut rows = [GOOD_ROW; 8];
    rows[7] = "null";
    let text = vil_of(&[good_layer(), good_layer(), layer_of(&rows)]);
    assert_eq!(error_of(&text)?, VilError::RowNotArray { layer: 2, row: 7 });

    let bad = row_with_cols(6);
    let mut rows = [GOOD_ROW; 8];
    rows[0] = &bad;
    let text = vil_of(&[good_layer(), layer_of(&rows)]);
    assert_eq!(
        error_of(&text)?,
        VilError::BadColCount {
            layer: 1,
            row: 0,
            found: 6
        }
    );
    Ok(())
}

#[test]
fn item4_bad_col_count() -> Result<(), String> {
    for cols in [6, 8, 0] {
        let bad = row_with_cols(cols);
        let mut rows = [GOOD_ROW; 8];
        rows[5] = &bad;
        let text = vil_of(&[layer_of(&rows)]);
        assert_eq!(
            error_of(&text)?,
            VilError::BadColCount {
                layer: 0,
                row: 5,
                found: cols
            }
        );
    }
    Ok(())
}

#[test]
fn item4_bad_value() -> Result<(), String> {
    for bad in ["null", "true", "1.5", "[]", "{}", "1.0", "false"] {
        let text = VilBuilder::new(1).raw(0, 2, 4, bad).build();
        assert_eq!(
            error_of(&text)?,
            VilError::BadValue {
                layer: 0,
                row: 2,
                col: 4
            },
            "{bad}"
        );
    }
    Ok(())
}

#[test]
fn item4_place_is_reported_for_layer_1_and_for_the_last_cell() -> Result<(), String> {
    // 2番目のレイヤーだけが壊れている
    let text = vil_of(&[good_layer(), "5".to_owned(), good_layer()]);
    assert_eq!(error_of(&text)?, VilError::LayerNotArray { layer: 1 });
    let text = vil_of(&[good_layer(), layer_of(&[GOOD_ROW; 7])]);
    assert_eq!(
        error_of(&text)?,
        VilError::BadRowCount { layer: 1, found: 7 }
    );
    // 最後の行の最後の列（キーの無い位置でも、値の種類は確かめる）
    let text = VilBuilder::new(3).raw(2, 7, 6, "null").build();
    assert_eq!(
        error_of(&text)?,
        VilError::BadValue {
            layer: 2,
            row: 7,
            col: 6
        }
    );
    let text = VilBuilder::new(1).raw(0, 0, 6, "1.5").build();
    assert_eq!(
        error_of(&text)?,
        VilError::BadValue {
            layer: 0,
            row: 0,
            col: 6
        }
    );
    Ok(())
}

#[test]
fn item4_only_the_first_error_is_returned() -> Result<(), String> {
    // 同じ行の中: 列の小さい方
    let text = VilBuilder::new(1)
        .raw(0, 1, 5, "null")
        .raw(0, 1, 2, "true")
        .build();
    assert_eq!(
        error_of(&text)?,
        VilError::BadValue {
            layer: 0,
            row: 1,
            col: 2
        }
    );
    // 同じレイヤーの中: 行の小さい方
    let text = VilBuilder::new(1)
        .raw(0, 6, 0, "null")
        .raw(0, 2, 5, "null")
        .build();
    assert_eq!(
        error_of(&text)?,
        VilError::BadValue {
            layer: 0,
            row: 2,
            col: 5
        }
    );
    // レイヤーの小さい方
    let text = VilBuilder::new(3)
        .raw(2, 0, 0, "null")
        .raw(1, 7, 5, "null")
        .build();
    assert_eq!(
        error_of(&text)?,
        VilError::BadValue {
            layer: 1,
            row: 7,
            col: 5
        }
    );
    // 外側（行の数）が、内側（値）より先
    let mut rows = vec![GOOD_ROW; 9];
    rows[0] = "[null]";
    let text = vil_of(&[layer_of(&rows)]);
    assert_eq!(
        error_of(&text)?,
        VilError::BadRowCount { layer: 0, found: 9 }
    );
    // 外側（列の数）が、内側（値）より先
    let mut rows = [GOOD_ROW; 8];
    rows[0] = "[null]";
    let text = vil_of(&[layer_of(&rows)]);
    assert_eq!(
        error_of(&text)?,
        VilError::BadColCount {
            layer: 0,
            row: 0,
            found: 1
        }
    );
    Ok(())
}

#[test]
fn item4_messages_are_the_ones_in_design() {
    let table = [
        (VilError::NotJson, "JSON として読めません"),
        (
            VilError::NotObject,
            "いちばん外側がオブジェクトではありません",
        ),
        (VilError::NoLayout, "layout がありません"),
        (VilError::LayoutNotArray, "layout が配列ではありません"),
        (VilError::NoLayers, "レイヤーが1つもありません"),
        (
            VilError::LayerNotArray { layer: 3 },
            "レイヤー 3 が配列ではありません",
        ),
        (
            VilError::BadRowCount { layer: 1, found: 9 },
            "レイヤー 1 の行の数が 8 ではありません（9 行）",
        ),
        (
            VilError::RowNotArray { layer: 2, row: 7 },
            "レイヤー 2 の行 7 が配列ではありません",
        ),
        (
            VilError::BadColCount {
                layer: 0,
                row: 4,
                found: 6,
            },
            "レイヤー 0 の行 4 の列の数が 7 ではありません（6 列）",
        ),
        (
            VilError::BadValue {
                layer: 9,
                row: 7,
                col: 6,
            },
            "レイヤー 9 の (7,6) の値が、文字列でも整数でもありません",
        ),
    ];
    for (error, message) in table {
        assert_eq!(error.to_string(), message);
    }
}

fn notices_of(builder: &VilBuilder) -> Result<Notices, VilError> {
    Ok(parse(&builder.build())?.notices())
}

#[test]
fn item5_unsupported_keys_are_counted_on_the_48_keys_only() -> Result<(), VilError> {
    let builder = VilBuilder::new(3)
        .key(0, 0, 0, "USER00")
        .key(0, 7, 5, "USER00")
        .key(2, 4, 0, "LT1(KC_SPACE)")
        // 丸い部品の2か所は、数えない
        .key(0, 2, 6, "KC_MUTE")
        .key(1, 5, 6, "KC_BTN3")
        // キーの無い位置の文字列も、数えない
        .key(0, 0, 6, "USER00")
        .key(1, 7, 6, "USER01");
    assert_eq!(
        notices_of(&builder)?,
        Notices {
            unsupported_keys: 3,
            combos: 0,
            tap_dances: 0,
            macros: 0
        }
    );
    // -1 以外の整数も未対応。-1 と KC_NO は、未対応ではない
    let builder = VilBuilder::new(1)
        .raw(0, 0, 0, "5")
        .raw(0, 0, 1, "-1")
        .key(0, 0, 2, "KC_NO");
    assert_eq!(notices_of(&builder)?.unsupported_keys, 1);
    assert_eq!(notices_of(&VilBuilder::new(1))?.unsupported_keys, 0);
    Ok(())
}

#[test]
fn item5_combos() -> Result<(), VilError> {
    let empty = r#"["KC_NO","KC_NO","KC_NO","KC_NO","KC_NO"]"#;
    let one = r#"["KC_NO","KC_NO","KC_A","KC_NO","KC_NO"]"#;
    let full = r#"["KC_A","KC_B","KC_NO","KC_NO","KC_C"]"#;
    let cases = [
        (format!("[{empty},{empty}]"), 0),
        ("[]".to_owned(), 0),
        (format!("[{empty},{one},{empty}]"), 1),
        (format!("[{one},{empty},{full}]"), 2),
    ];
    for (combo, expected) in cases {
        let notices = notices_of(&VilBuilder::new(1).top("combo", &combo))?;
        assert_eq!(notices.combos, expected, "{combo}");
        assert_eq!(notices.tap_dances, 0);
        assert_eq!(notices.macros, 0);
    }
    Ok(())
}

#[test]
fn item5_tap_dances_ignore_numbers() -> Result<(), VilError> {
    let empty = r#"["KC_NO","KC_NO","KC_NO","KC_NO",250]"#;
    let used = r#"["KC_A","KC_NO","KC_NO","KC_NO",250]"#;
    let builder = VilBuilder::new(1).top("tap_dance", &format!("[{empty}]"));
    assert_eq!(notices_of(&builder)?.tap_dances, 0);
    let builder = VilBuilder::new(1).top("tap_dance", &format!("[{empty},{used},{empty}]"));
    let notices = notices_of(&builder)?;
    assert_eq!(notices.tap_dances, 1);
    assert_eq!(notices.combos, 0);
    Ok(())
}

#[test]
fn item5_macros() -> Result<(), VilError> {
    let builder = VilBuilder::new(1).top("macro", "[[],[],[]]");
    assert_eq!(notices_of(&builder)?.macros, 0);
    let builder = VilBuilder::new(1).top("macro", r#"[[],[["text","abc"]],[],[["tap","KC_A"]]]"#);
    let notices = notices_of(&builder)?;
    assert_eq!(notices.macros, 2);
    assert_eq!(notices.combos, 0);
    assert_eq!(notices.tap_dances, 0);
    Ok(())
}

#[test]
fn item5_missing_or_non_array_fields_count_zero() -> Result<(), VilError> {
    assert_eq!(notices_of(&VilBuilder::new(1))?, Notices::default());
    for bad in ["5", "\"abc\"", "{}", "null"] {
        let builder = VilBuilder::new(1)
            .top("combo", bad)
            .top("tap_dance", bad)
            .top("macro", bad);
        assert_eq!(notices_of(&builder)?, Notices::default(), "{bad}");
    }
    // 配列の中に、配列でない物が混ざっていても、数えないだけ
    let builder = VilBuilder::new(1)
        .top("combo", r#"[5,"KC_A",null]"#)
        .top("macro", r#"[5,"x",null]"#);
    assert_eq!(notices_of(&builder)?, Notices::default());
    Ok(())
}

/// 知らせの文（B-07）。期待値は、design の B-07 の形から手で書いた物。
#[test]
fn b07_notice_message_omits_zero_counts() {
    let notices = |unsupported_keys, combos, tap_dances, macros| Notices {
        unsupported_keys,
        combos,
        tap_dances,
        macros,
    };
    // 全部 0 なら、知らせは出さない
    assert_eq!(notices(0, 0, 0, 0).message(), None);
    assert_eq!(
        notices(6, 0, 0, 0).message().as_deref(),
        Some("動かない物: 未対応のキー 6 個")
    );
    assert_eq!(
        notices(1, 2, 3, 4).message().as_deref(),
        Some("動かない物: 未対応のキー 1 個、コンボ 2 個、タップダンス 3 個、マクロ 4 個")
    );
    // 0 個の物は、間にあっても、先頭にあっても省く
    assert_eq!(
        notices(0, 2, 0, 4).message().as_deref(),
        Some("動かない物: コンボ 2 個、マクロ 4 個")
    );
    assert_eq!(
        notices(0, 0, 1, 0).message().as_deref(),
        Some("動かない物: タップダンス 1 個")
    );
}
