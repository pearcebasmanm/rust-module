# Rust Module
A proof-of-concept module to tinker with the idea of using Rust in a Foundry VTT module.

## Testing Workflow
Rust changes
- make edits to rust.rs
- run `wasm-pack build --target web`
- refresh foundry (F5)

Javascript changes
- make edits to js/index.js
- refresh foundry (F5)

## Planned Capabilities
- Interfaced with the Foundry API
- Use a Rust UI framework