# Rust Module
A proof-of-concept module to tinker with the idea of using Rust in a Foundry VTT module.

## Testing Workflow
Rust changes
- make edits to src/lib.rs
- run wasm-pack build
- run npm install
- run npx webpack
Javascript changes
- make edits to src/index.js
- run npx webpack

## Errors
This module is not yet functional, and these are the errors it produces
`GET http://localhost:30000/b80ce72080fe0972b3a8.module.wasm 404 (Not Found)`
`Uncaught (in promise) TypeError: Failed to execute 'compile' on 'WebAssembly': HTTP status code is not ok`