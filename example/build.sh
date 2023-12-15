cargo component build --release
cargo component build --manifest-path ../sqlite-proxy/Cargo.toml --release
wasm-tools compose target/wasm32-wasi/release/example.wasm -d ../sqlite-proxy/target/wasm32-wasi/release/sqlite_proxy.wasm -o service.wasm