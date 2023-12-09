cargo component build --release
wasm-tools compose target/wasm32-wasi/release/todo.wasm -d ../../target/wasm32-wasi/release/sqlite_proxy.wasm -o todo.wasm