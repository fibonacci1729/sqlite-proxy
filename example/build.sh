cargo component build --release
wasm-tools compose target/wasm32-wasi/release/example.wasm -d ../sqlite-proxy/target/wasm32-wasi/release/sqlite-proxy.wasm -o service.wasm