//! DOM との受け渡しで、どのページでも使う手伝い。wasm32 のときだけ。

use wasm_bindgen::convert::FromWasmAbi;
use wasm_bindgen::prelude::*;
use web_sys::EventTarget;

/// イベントを受ける関数を登録する。関数は、ページが閉じるまで生かしておく。
pub fn listen<E>(
    target: &EventTarget,
    kind: &str,
    handler: impl FnMut(E) -> Result<(), JsValue> + 'static,
) -> Result<(), JsValue>
where
    E: FromWasmAbi + 'static,
{
    let closure = Closure::<dyn FnMut(E) -> Result<(), JsValue>>::new(handler);
    target.add_event_listener_with_callback(kind, closure.as_ref().unchecked_ref())?;
    closure.forget();
    Ok(())
}
