rm -r src/pkg
wasm-pack build --target web --release
mv pkg src
