import init, * as wasm from "../wasm/rust_module.js";

init().then(() => {
    console.log(wasm.rustFunction())
})
