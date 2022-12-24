# Rust Module
A proof-of-concept for using Rust in a Foundry VTT module.

## Testing Workflow
Rust changes
- make edits in src/lib.rs
- run `wasm-pack build --target web`
- refresh foundry (F5)

Javascript changes
- make edits in index.js
- refresh foundry (F5)

## Releasing Workflow
`zip rust_module module.json LICENSE index.js pkg/rust_module.js pkg/rust_module_bg.wasm`

## Current Limitations
- Interface with the Foundry API
- Use a Rust UI framework