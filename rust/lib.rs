use wasm_bindgen::prelude::*;

#[wasm_bindgen(js_name=rustFunction)]
pub fn rust_function() -> JsValue {
    "Rust String".into()
}