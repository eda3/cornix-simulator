//! テスト用の .vil を組み立てる手伝い。入力を作るのに使う（期待値を作るのには使わない）。

/// 位置とキーコードを渡すと、.vil の文字列を返す。
///
/// 何も置かないマスは、キーのある位置が "`KC_TRNS`"、キーの無い位置が -1。
/// いちばん外側には、`version` と `uid` を入れておく（[`VilBuilder::without`] で外せる）。
pub struct VilBuilder {
    layers: usize,
    cells: Vec<((usize, usize, usize), String)>,
    top: Vec<(String, String)>,
}

impl VilBuilder {
    pub fn new(layers: usize) -> Self {
        Self {
            layers,
            cells: Vec::new(),
            top: vec![
                ("version".to_owned(), "1".to_owned()),
                ("uid".to_owned(), "0".to_owned()),
            ],
        }
    }

    /// マスに、キーコードの文字列を置く。
    pub fn key(self, layer: usize, row: usize, col: usize, keycode: &str) -> Self {
        self.raw(layer, row, col, &format!("\"{keycode}\""))
    }

    /// マスに、JSON の値をそのまま置く（整数・null・配列など）。
    pub fn raw(mut self, layer: usize, row: usize, col: usize, json: &str) -> Self {
        self.cells
            .retain(|(position, _)| *position != (layer, row, col));
        self.cells.push(((layer, row, col), json.to_owned()));
        self
    }

    /// いちばん外側の項目を置く（値は、JSON のまま）。
    pub fn top(mut self, name: &str, json: &str) -> Self {
        self.top.retain(|(existing, _)| existing != name);
        self.top.push((name.to_owned(), json.to_owned()));
        self
    }

    /// いちばん外側の項目を外す。
    pub fn without(mut self, name: &str) -> Self {
        self.top.retain(|(existing, _)| existing != name);
        self
    }

    /// `layout` の中身（レイヤーの配列）の JSON。
    pub fn layout(&self) -> String {
        let layers: Vec<String> = (0..self.layers)
            .map(|layer| {
                let rows: Vec<String> = (0..8)
                    .map(|row| {
                        let cols: Vec<String> =
                            (0..7).map(|col| self.cell(layer, row, col)).collect();
                        format!("[{}]", cols.join(","))
                    })
                    .collect();
                format!("[{}]", rows.join(","))
            })
            .collect();
        format!("[{}]", layers.join(","))
    }

    pub fn build(&self) -> String {
        let mut fields = vec![format!("\"layout\":{}", self.layout())];
        fields.extend(
            self.top
                .iter()
                .map(|(name, json)| format!("\"{name}\":{json}")),
        );
        format!("{{{}}}", fields.join(","))
    }

    fn cell(&self, layer: usize, row: usize, col: usize) -> String {
        self.cells
            .iter()
            .find(|(position, _)| *position == (layer, row, col))
            .map_or_else(
                || {
                    let has_key = col < 6 || row == 2 || row == 5;
                    if has_key { "\"KC_TRNS\"" } else { "-1" }.to_owned()
                },
                |(_, json)| json.clone(),
            )
    }
}
