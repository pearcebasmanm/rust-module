use wasm_bindgen::prelude::*;

// run Rust from JavaScript
#[wasm_bindgen]
pub fn rustfn() {
    // println!("Hello World (Rust)");
}

// // run JavaScript from Rust
// #[wasm_bindgen(module = "www/index.js")]
// extern "C" {
//     fn jstest();
// }