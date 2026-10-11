//! Cornix LP シミュレータの画面（WASM）。
//!
//! 計測ページ（`static/keytest.html`）の記録を作る判断は、web-sys に依存しない [`keylog`] に置く
//! （`cargo test` で確かめるため）。DOM とイベントの配線は、wasm32 のときだけの `keytest` に任せる。

pub mod keylog;
#[cfg(target_arch = "wasm32")]
pub mod keytest;
