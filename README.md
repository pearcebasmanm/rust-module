# Rust Module
A proof-of-concept for using Rust in a Foundry VTT module.

## Testing Workflow
Rust changes
- make edits to rust.rs
- run `wasm-pack build --target web`
- refresh foundry (F5)

Javascript changes
- make edits to js/index.js
- refresh foundry (F5)

## What it can't do (at least not yet)
- Interface with the Foundry API
- Use a Rust UI framework