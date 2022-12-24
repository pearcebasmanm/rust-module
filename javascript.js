import init, * as wasm from "./pkg/rust_module.js";
init();

Hooks.on("ready", () => {
    console.log(wasm.hello_world())
})
