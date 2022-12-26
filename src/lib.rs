use wasm_bindgen::prelude::*;
mod api;
use api::*;

#[wasm_bindgen]
pub fn run() {
    hello();
}

fn hello() {
    let a: Closure<dyn Fn()> = Closure::new(|| log("Hello From Rust!"));
    on("ready", &a);
    a.forget();
}
