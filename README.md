# sqlite-proxy

## Prerequisites
- Install [cargo component v0.4.0](https://github.com/bytecodealliance/cargo-component):

```bash
cargo install --git https://github.com/bytecodealliance/cargo-component --tag v0.4.0 cargo-component --locked
```

- Install [wasm-tools](https://github.com/bytecodealliance/wasm-tools): 

```bash
cargo install --git https://github.com/bytecodealliance/wasm-tools wasm-tools --locked
```

> NOTE: I had to install `wasm-tools` from `fe363f0` because of some undiagnosed bug introduced in a later commit around import name validation.

## Build, Compose, Up, Profit

Build the sqlite-proxy component:
```
cargo component build --release
```

Build, compose, and up the example:
```
spin up --build -f examples/todo --sqlite "@examples/todo/migration.sql"
```

Add and delete a todo and in the terminal you should see the `sqlite` requests being "proxied" through the wrapper component:
```
Serving http://127.0.0.1:3000
Available Routes:
  todo: http://127.0.0.1:3000/api (wildcard)
  fs: http://127.0.0.1:3000 (wildcard)
IN WRAPPER
IN EXECUTE
IN WRAPPER
IN EXECUTE
IN WRAPPER
IN EXECUTE
IN WRAPPER
IN EXECUTE
```