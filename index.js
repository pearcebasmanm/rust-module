import init, * as wasm from "./pkg/rust_module.js";
init().then(wasm.run)
