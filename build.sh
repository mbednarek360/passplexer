rm -r src/pkg
wasm-pack build --target web --release
mv pkg src
tar -cjvf passplexer_0.2.2.tar.bz2 --exclude='*.rs' src
