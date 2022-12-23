cd rust
wasm-pack build --target web --no-typescript --out-dir ../wasm
rm -r target

cd ../wasm

rm .gitignore package.json