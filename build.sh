rm -rv src/pkg
echo
wasm-pack build --target web --release
echo
mv -v pkg src
echo
find src/pkg -type f ! -name '*.js' ! -name '*.wasm' -delete -print
echo
find src/pkg -type f ! -name '*.wasm' -exec minify {} -o {} \; -print
echo
find src -name "*.html" -o -name "*.js" -o -name "*.wasm" | tar -cvjf passplexer_0.2.2.tar.bz2 -T -
