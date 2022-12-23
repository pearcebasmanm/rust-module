# Rust Module
A proof-of-concept module to tinker with the idea of using Rust in a Foundry VTT module.

## Testing Workflow
Rust changes
- make edits to rust/lib.rs
- run build_wasm.sh
- refresh foundry (F5)

Javascript changes
- make edits to js/index.js
- refresh foundry (F5)

## Errors
This module is not yet functional, and this is the error it produces
`Uncaught TypeError: Cannot read properties of undefined (reading 'rustFunction')`