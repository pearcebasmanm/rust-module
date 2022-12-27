use leptos::Element;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    pub type Hooks;
    #[wasm_bindgen(js_namespace = Hooks, js_name = on)]
    fn onto(hook: String, closure: &Closure<dyn Fn(Element)>) -> i32;
}

impl Hooks {
    pub fn on(hook: Hook, closure: Closure<dyn Fn(Element)>) {
        Hooks::onto(hook.into(), &closure);
        closure.forget();
    }
}


pub use Hook::*;
pub enum Hook {
    Ready,
    RenderApplication,
    // ...
}

impl From<Hook> for String {
    fn from(val: Hook) -> Self {
        match val {
            Ready => "ready",
            RenderApplication => "renderApplication",
        }.to_owned()
    }
}