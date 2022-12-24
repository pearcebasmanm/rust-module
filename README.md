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
- update the version number in module.json (both the "version" field, and in the url of the "download" field)
- push a tag starting with "v" (note to self, find out what that means)

## Current Limitations
- Interface with the Foundry API
- Use a Rust UI framework