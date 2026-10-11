//! キーの座標と回転（B-11）。表は `docs/design.md` の 3.3節から手で写した物。
//!
//! 単位は「キー1つ分の間隔」。x は右向き、y は下向きで、(x, y) はキーの左上の角。
//! 回転は、時計回りが正（度）。

/// 図に描く1つの部品（キーか、丸い部品）。
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct KeyShape {
    /// 位置（行, 列）。
    pub position: (usize, usize),
    /// 左上の角の x。
    pub x: f64,
    /// 左上の角の y。
    pub y: f64,
    /// 回転（度。時計回りが正）。0 なら、回転なし。
    pub rotation: f64,
    /// 回転の中心。回転が 0 のキーには無い。
    pub pivot: Option<(f64, f64)>,
    /// 丸い部品か。
    pub knob: bool,
}

const fn key(row: usize, col: usize, x: f64, y: f64) -> KeyShape {
    KeyShape {
        position: (row, col),
        x,
        y,
        rotation: 0.0,
        pivot: None,
        knob: false,
    }
}

const fn knob(row: usize, col: usize, x: f64, y: f64) -> KeyShape {
    KeyShape {
        position: (row, col),
        x,
        y,
        rotation: 0.0,
        pivot: None,
        knob: true,
    }
}

const fn rotated(
    row: usize,
    col: usize,
    x: f64,
    y: f64,
    rotation: f64,
    pivot: (f64, f64),
) -> KeyShape {
    KeyShape {
        position: (row, col),
        x,
        y,
        rotation,
        pivot: Some(pivot),
        knob: false,
    }
}

/// 50の位置の座標（3.3節の表の順）。
pub const SHAPES: [KeyShape; 50] = [
    key(0, 0, 0.0, 0.5),
    key(0, 1, 1.0, 0.5),
    key(0, 2, 2.0, 0.25),
    key(0, 3, 3.0, 0.0),
    key(0, 4, 4.0, 0.25),
    key(0, 5, 5.0, 0.5),
    key(1, 0, 0.0, 1.5),
    key(1, 1, 1.0, 1.5),
    key(1, 2, 2.0, 1.25),
    key(1, 3, 3.0, 1.0),
    key(1, 4, 4.0, 1.25),
    key(1, 5, 5.0, 1.5),
    key(2, 0, 0.0, 2.55),
    key(2, 1, 1.0, 2.55),
    key(2, 2, 2.0, 2.25),
    key(2, 3, 3.0, 2.0),
    key(2, 4, 4.0, 2.25),
    key(2, 5, 5.0, 2.55),
    knob(2, 6, 6.0, 2.0),
    key(3, 0, 0.0, 3.55),
    key(3, 1, 1.0, 3.55),
    key(3, 2, 2.0, 3.3),
    key(3, 3, 3.5, 3.63),
    rotated(3, 4, 4.5, 3.7, 11.93, (5.0, 4.75)),
    rotated(3, 5, 5.5, 3.65, 23.0, (5.0, 4.75)),
    key(4, 0, 13.5, 0.5),
    key(4, 1, 12.5, 0.5),
    key(4, 2, 11.5, 0.33),
    key(4, 3, 10.5, 0.0),
    key(4, 4, 9.5, 0.3),
    key(4, 5, 8.5, 0.5),
    key(5, 0, 13.5, 1.5),
    key(5, 1, 12.5, 1.5),
    key(5, 2, 11.5, 1.33),
    key(5, 3, 10.5, 1.0),
    key(5, 4, 9.5, 1.3),
    key(5, 5, 8.5, 1.5),
    knob(5, 6, 7.5, 2.0),
    key(6, 0, 13.5, 2.55),
    key(6, 1, 12.5, 2.55),
    key(6, 2, 11.5, 2.33),
    key(6, 3, 10.5, 2.0),
    key(6, 4, 9.5, 2.3),
    key(6, 5, 8.5, 2.55),
    key(7, 0, 13.5, 3.55),
    key(7, 1, 12.5, 3.55),
    key(7, 2, 11.5, 3.38),
    key(7, 3, 10.0, 3.63),
    rotated(7, 4, 9.0, 3.75, -11.93, (9.25, 4.75)),
    rotated(7, 5, 7.64, 5.38, -23.0, (5.0, 4.75)),
];

/// 位置の座標。キーの無い位置は `None`。
#[must_use]
pub fn shape_of(row: usize, col: usize) -> Option<&'static KeyShape> {
    SHAPES.iter().find(|shape| shape.position == (row, col))
}

#[cfg(test)]
mod tests {
    use super::{SHAPES, shape_of};
    use crate::sample::SAMPLE_VIL;

    type TestResult = Result<(), Box<dyn std::error::Error>>;

    /// x・y・回転・回転の中心。
    type Row = ((usize, usize), f64, f64, f64, Option<(f64, f64)>);

    #[test]
    fn item12_fifty_positions_the_same_as_the_sample() -> TestResult {
        assert_eq!(SHAPES.len(), 50);
        // 見本の .vil の、-1 でない位置（レイヤー0）
        let json: serde_json::Value = serde_json::from_str(SAMPLE_VIL)?;
        let rows = json["layout"][0].as_array().ok_or("layout が無い")?;
        let mut expected = Vec::new();
        for (row, cols) in rows.iter().enumerate() {
            for (col, value) in cols.as_array().ok_or("行が配列でない")?.iter().enumerate() {
                if value.as_i64() != Some(-1) {
                    expected.push((row, col));
                }
            }
        }
        assert_eq!(expected.len(), 50);
        let mut actual: Vec<(usize, usize)> = SHAPES.iter().map(|shape| shape.position).collect();
        actual.sort_unstable();
        expected.sort_unstable();
        assert_eq!(actual, expected);
        // 同じ位置が2回出ない
        actual.dedup();
        assert_eq!(actual.len(), 50);
        Ok(())
    }

    #[test]
    fn item12_knobs_are_only_the_two() {
        let knobs: Vec<(usize, usize)> = SHAPES
            .iter()
            .filter(|shape| shape.knob)
            .map(|shape| shape.position)
            .collect();
        assert_eq!(knobs, [(2, 6), (5, 6)]);
    }

    #[test]
    fn item12_values_copied_from_the_design_table() -> TestResult {
        let table: [Row; 15] = [
            ((0, 0), 0.0, 0.5, 0.0, None),
            ((0, 3), 3.0, 0.0, 0.0, None),
            ((2, 0), 0.0, 2.55, 0.0, None),
            ((2, 6), 6.0, 2.0, 0.0, None),
            ((3, 2), 2.0, 3.3, 0.0, None),
            ((3, 3), 3.5, 3.63, 0.0, None),
            ((3, 4), 4.5, 3.7, 11.93, Some((5.0, 4.75))),
            ((3, 5), 5.5, 3.65, 23.0, Some((5.0, 4.75))),
            ((4, 0), 13.5, 0.5, 0.0, None),
            ((4, 2), 11.5, 0.33, 0.0, None),
            ((5, 6), 7.5, 2.0, 0.0, None),
            ((7, 2), 11.5, 3.38, 0.0, None),
            ((7, 3), 10.0, 3.63, 0.0, None),
            ((7, 4), 9.0, 3.75, -11.93, Some((9.25, 4.75))),
            ((7, 5), 7.64, 5.38, -23.0, Some((5.0, 4.75))),
        ];
        for (position, x, y, rotation, pivot) in table {
            let shape = shape_of(position.0, position.1).ok_or(format!("{position:?} が無い"))?;
            assert_eq!(
                (shape.x, shape.y, shape.rotation, shape.pivot),
                (x, y, rotation, pivot),
                "{position:?}"
            );
        }
        // キーの無い位置と、範囲の外
        assert_eq!(shape_of(0, 6), None);
        assert_eq!(shape_of(7, 6), None);
        assert_eq!(shape_of(8, 0), None);
        Ok(())
    }

    #[test]
    fn item12_only_four_keys_are_rotated() {
        let rotated: Vec<(usize, usize)> = SHAPES
            .iter()
            .filter(|shape| shape.rotation != 0.0)
            .map(|shape| shape.position)
            .collect();
        assert_eq!(rotated, [(3, 4), (3, 5), (7, 4), (7, 5)]);
        // 回転の中心があるのは、回転のあるキーだけ
        for shape in &SHAPES {
            assert_eq!(
                shape.pivot.is_some(),
                shape.rotation != 0.0,
                "{:?}",
                shape.position
            );
        }
    }
}
