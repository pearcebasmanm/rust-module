use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn hello_world() -> JsValue {
    "Hello From Rust!".into()
}
