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
Whenever you push a changed to module.json it will create a new release automatically
- Remember to change both the "version" field, and the url of the "download" field
