//! DOM との受け渡しで、どのページでも使う手伝い。wasm32 のときだけ。

use js_sys::{Function, Reflect};
use wasm_bindgen::convert::FromWasmAbi;
use wasm_bindgen::prelude::*;
use web_sys::{EventTarget, Window};

/// `navigator.keyboard`。Keyboard Lock の無いブラウザ（値が undefined か null）では `None`。
///
/// web-sys に Keyboard Lock の型が無いので、`js_sys::Reflect` で読む。
pub fn keyboard(window: &Window) -> Result<Option<JsValue>, JsValue> {
    let keyboard = Reflect::get(&window.navigator(), &JsValue::from_str("keyboard"))?;
    Ok((!keyboard.is_undefined() && !keyboard.is_null()).then_some(keyboard))
}

/// JavaScript のオブジェクトのメソッドを、引数なしで呼ぶ。
///
/// web-sys に型の無い物（`navigator.keyboard` の `lock`・`unlock`）を呼ぶのに使う。
pub fn call(target: &JsValue, method: &str) -> Result<JsValue, JsValue> {
    let function: Function = Reflect::get(target, &JsValue::from_str(method))?.dyn_into()?;
    function.call0(target)
}

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
