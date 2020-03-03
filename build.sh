rm -rv src/pkg
echo
wasm-pack build --target web --release
echo
mv -v pkg src
echo
find src -name "*.js" -o -name ".wasm" | tar -cvjf passplexer_0.2.2.tar.bz2 -T -
