use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    pub fn log(s: &str);
    #[wasm_bindgen(js_namespace = Hooks)]
    pub fn on(event: &str, closure: &Closure<dyn Fn()>);
}
