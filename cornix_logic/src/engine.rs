//! キーを押す・離すの処理。レイヤーと修飾キーの状態を持ち、欄への動きと「直前のキー」を返す。
//!
//! 決まりは `docs/design.md` の B-25〜B-35（レイヤーと修飾）・B-37〜B-39（文字と欄）・
//! B-41 と 3.7節（直前のキー）。Cornix のファームウェア（RMK）の動きに合わせてある。

use crate::chars::{Layout, char_of, name_of};
use crate::keycode::{BaseKey, CharKey, EditKey, Keycode, Modifier};
use crate::vil::{Keymap, has_key, is_knob};

/// キーを押したときに決まった中身（B-25）。
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Decided {
    /// 割り当てなし（レイヤー0まで `KC_TRNS` だった）。
    Unassigned,
    /// キーコードに決まった（`KC_NO` も、ここ）。
    Key {
        /// 決まったキーコード。
        keycode: Keycode,
        /// そのキーコードがあったレイヤー。
        layer: usize,
    },
}

impl Decided {
    /// 決まったキーコード。割り当てなしなら `None`。
    #[must_use]
    pub const fn keycode(&self) -> Option<&Keycode> {
        match self {
            Self::Unassigned => None,
            Self::Key { keycode, .. } => Some(keycode),
        }
    }
}

/// 欄への動き（B-38）。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FieldAction {
    /// 末尾に、文字を足す（空白・改行・タブ文字を含む）。
    Insert(char),
    /// 末尾の1文字を消す。
    DeleteLast,
    /// 欄は変えない。
    None,
}

/// 「押す」の結果。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PressOutcome {
    /// 決まった中身。
    pub decided: Decided,
    /// 欄への動き。
    pub action: FieldAction,
    /// 「直前のキー」の文（3.7節）。
    pub last_key: String,
}

/// キーの状態（有効なレイヤー・修飾キー・押されている位置）を持つ。
#[derive(Debug, Clone)]
pub struct Engine {
    keymap: Keymap,
    /// レイヤーごとの有効・無効（数えない。B-30）。レイヤー0は、いつも有効。
    layers: Vec<bool>,
    /// 修飾の入・切（[`Modifier::ALL`] の順。数えない。B-33）。
    modifiers: [bool; 8],
    /// 押されている位置と、押したときに決まった中身（B-27）。
    held: Vec<((usize, usize), Decided)>,
}

impl Engine {
    /// キーマップから、何も押していない状態を作る。
    #[must_use]
    pub fn new(keymap: Keymap) -> Self {
        let mut layers = vec![false; keymap.layer_count()];
        if let Some(base) = layers.first_mut() {
            *base = true;
        }
        Self {
            keymap,
            layers,
            modifiers: [false; 8],
            held: Vec::new(),
        }
    }

    /// 読み込んであるキーマップ。
    #[must_use]
    pub const fn keymap(&self) -> &Keymap {
        &self.keymap
    }

    /// 有効なレイヤーの番号（小さい順）。
    #[must_use]
    pub fn active_layers(&self) -> Vec<usize> {
        (0..self.layers.len())
            .filter(|layer| self.layers[*layer])
            .collect()
    }

    /// その修飾が入っているか（左右は別）。
    #[must_use]
    pub const fn modifier(&self, modifier: Modifier) -> bool {
        self.modifiers[index_of(modifier)]
    }

    /// Shift（左右のどちらか）が入っているか。
    #[must_use]
    pub const fn shift(&self) -> bool {
        self.modifier(Modifier::LShift) || self.modifier(Modifier::RShift)
    }

    /// Ctrl（左右のどちらか）が入っているか。
    #[must_use]
    pub const fn ctrl(&self) -> bool {
        self.modifier(Modifier::LCtrl) || self.modifier(Modifier::RCtrl)
    }

    /// Alt（左右のどちらか）が入っているか。
    #[must_use]
    pub const fn alt(&self) -> bool {
        self.modifier(Modifier::LAlt) || self.modifier(Modifier::RAlt)
    }

    /// GUI（左右のどちらか）が入っているか。
    #[must_use]
    pub const fn gui(&self) -> bool {
        self.modifier(Modifier::LGui) || self.modifier(Modifier::RGui)
    }

    /// 押されている位置なら、押したときに決まった中身。
    #[must_use]
    pub fn held(&self, row: usize, col: usize) -> Option<&Decided> {
        self.held
            .iter()
            .find(|(position, _)| *position == (row, col))
            .map(|(_, decided)| decided)
    }

    /// いま押したら決まる中身（B-25）。
    ///
    /// 有効なレイヤーを、番号の大きい方から見て、`KC_TRNS` でない最初のキーコードに決まる。
    #[must_use]
    pub fn resolve(&self, row: usize, col: usize) -> Decided {
        (0..self.layers.len())
            .rev()
            .filter(|layer| self.layers[*layer])
            .find_map(|layer| match self.keymap.key(layer, row, col) {
                None | Some(Keycode::Transparent) => None,
                Some(keycode) => Some(Decided::Key {
                    keycode: keycode.clone(),
                    layer,
                }),
            })
            .unwrap_or(Decided::Unassigned)
    }

    /// 位置のキーを押す。キーの無い位置と、丸い部品の位置は、押せない（`None`）。
    pub fn press(&mut self, row: usize, col: usize, layout: Layout) -> Option<PressOutcome> {
        if !has_key(row, col) || is_knob(row, col) {
            return None;
        }
        // 押されている位置を、もう一度押した扱いにはしない（B-22）
        if self.held(row, col).is_some() {
            return None;
        }
        let decided = self.resolve(row, col);
        let (action, last_key) = self.apply(decided.keycode(), layout);
        self.held.push(((row, col), decided.clone()));
        Some(PressOutcome {
            decided,
            action,
            last_key,
        })
    }

    /// 位置のキーを離す。押したときに覚えたキーコードで処理する（B-27）。
    pub fn release(&mut self, row: usize, col: usize) {
        let Some(index) = self
            .held
            .iter()
            .position(|(position, _)| *position == (row, col))
        else {
            return;
        };
        let (_, decided) = self.held.remove(index);
        match decided.keycode() {
            Some(Keycode::Mo(layer)) => self.set_layer(*layer, false),
            Some(Keycode::Modifier(modifier)) => self.modifiers[index_of(*modifier)] = false,
            _ => {}
        }
    }

    /// 押し続け（repeat の付いた keydown）。欄への動きだけを返す（B-23）。
    ///
    /// レイヤーと修飾キーの状態は変えない。その位置が、押したときに「欄を変える」キーコード
    /// （文字のキー・Shift 付き・欄を変えるキー）に決まっていたら、いまの修飾キーの状態で、
    /// 欄への動きをもう一度する。押されていない位置は、無視する。
    #[must_use]
    pub fn repeat(&self, row: usize, col: usize, layout: Layout) -> FieldAction {
        match self.held(row, col).and_then(Decided::keycode) {
            Some(Keycode::Char(key)) => self.type_base(BaseKey::Char(*key), false, layout).0,
            Some(Keycode::Edit(key)) => self.type_base(BaseKey::Edit(*key), false, layout).0,
            Some(Keycode::Shifted { base, .. }) => self.type_base(*base, true, layout).0,
            _ => FieldAction::None,
        }
    }

    /// 押されているキーを、全部離した扱いにする（B-24）。
    ///
    /// 有効なレイヤーはレイヤー0だけに、修飾キーは全部切れた状態に戻る。
    pub fn release_all(&mut self) {
        self.held.clear();
        self.modifiers = [false; 8];
        for (layer, state) in self.layers.iter_mut().enumerate() {
            *state = layer == 0;
        }
    }

    /// 押されている位置（押した順）。
    #[must_use]
    pub fn held_positions(&self) -> Vec<(usize, usize)> {
        self.held.iter().map(|(position, _)| *position).collect()
    }

    /// 決まったキーコードを、状態に当てる。欄への動きと「直前のキー」を返す。
    fn apply(&mut self, keycode: Option<&Keycode>, layout: Layout) -> (FieldAction, String) {
        match keycode {
            None | Some(Keycode::No | Keycode::Transparent) => {
                (FieldAction::None, "（割り当てなし）".to_owned())
            }
            Some(Keycode::Mo(layer)) => {
                self.set_layer(*layer, true);
                (FieldAction::None, format!("MO({layer})"))
            }
            Some(Keycode::Modifier(modifier)) => {
                self.modifiers[index_of(*modifier)] = true;
                (FieldAction::None, self.modifier_names(false).join("+"))
            }
            Some(Keycode::Char(key)) => self.type_base(BaseKey::Char(*key), false, layout),
            Some(Keycode::Edit(key)) => self.type_base(BaseKey::Edit(*key), false, layout),
            Some(Keycode::Named(key)) => self.type_base(BaseKey::Named(*key), false, layout),
            Some(Keycode::Shifted { base, .. }) => self.type_base(*base, true, layout),
            Some(Keycode::Unsupported(text)) => (FieldAction::None, format!("{text}（未対応）")),
        }
    }

    /// 文字のキー・欄を変えるキー・名前だけのキーを、いまの修飾で処理する。
    ///
    /// `forced_shift` は、Shift 付きのキーコードのとき（その処理の間だけ、Shift が入っている扱い。B-34）。
    /// 状態は変えない。
    pub(crate) fn type_base(
        &self,
        base: BaseKey,
        forced_shift: bool,
        layout: Layout,
    ) -> (FieldAction, String) {
        let shift = forced_shift || self.shift();
        // Ctrl・Alt・GUI のどれかが入っている間は、欄を変えない（B-35）
        let blocked = self.ctrl() || self.alt() || self.gui();
        let (name, action) = match base {
            BaseKey::Char(key) => {
                let action = char_of(key, shift, layout)
                    .filter(|_| !blocked)
                    .map_or(FieldAction::None, FieldAction::Insert);
                (char_key_name(key, layout), action)
            }
            BaseKey::Edit(key) => {
                let action = match key {
                    _ if blocked => FieldAction::None,
                    EditKey::Enter => FieldAction::Insert('\n'),
                    EditKey::Tab => FieldAction::Insert('\t'),
                    EditKey::Space => FieldAction::Insert(' '),
                    EditKey::Bspace => FieldAction::DeleteLast,
                };
                (key.label().to_owned(), action)
            }
            BaseKey::Named(key) => (key.label().to_owned(), FieldAction::None),
        };
        let mut parts = self.modifier_names(forced_shift);
        parts.push(&name);
        let combination = parts.join("+");
        let last_key = match (base, action) {
            (BaseKey::Char(_), FieldAction::Insert(typed)) => format!("{combination} → {typed}"),
            _ => combination,
        };
        (action, last_key)
    }

    /// 入っている修飾の名前（Ctrl・Shift・Alt・GUI の順。左右は区別しない）。
    fn modifier_names(&self, forced_shift: bool) -> Vec<&str> {
        [
            (self.ctrl(), "Ctrl"),
            (forced_shift || self.shift(), "Shift"),
            (self.alt(), "Alt"),
            (self.gui(), "GUI"),
        ]
        .into_iter()
        .filter_map(|(on, name)| on.then_some(name))
        .collect()
    }

    /// レイヤーの有効・無効を変える。レイヤー0と、レイヤーの数以上の番号は、変えない（B-28・B-31）。
    fn set_layer(&mut self, layer: u8, on: bool) {
        let layer = usize::from(layer);
        if layer >= 1
            && let Some(state) = self.layers.get_mut(layer)
        {
            *state = on;
        }
    }
}

/// 修飾の、状態の並びの中での番号（[`Modifier::ALL`] の順）。
const fn index_of(modifier: Modifier) -> usize {
    match modifier {
        Modifier::LCtrl => 0,
        Modifier::LShift => 1,
        Modifier::LAlt => 2,
        Modifier::LGui => 3,
        Modifier::RCtrl => 4,
        Modifier::RShift => 5,
        Modifier::RAlt => 6,
        Modifier::RGui => 7,
    }
}

/// 文字のキーの名前（3.7節）: 英字は大文字1つ。ほかは素の文字。素が「なし」なら、3.2節の名前。
fn char_key_name(key: CharKey, layout: Layout) -> String {
    match key {
        CharKey::Letter(letter) => letter.to_ascii_uppercase().to_string(),
        _ => char_of(key, false, layout).map_or_else(
            || name_of(key, layout).unwrap_or_default().to_owned(),
            String::from,
        ),
    }
}
