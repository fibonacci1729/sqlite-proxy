# sqlite-proxy

This repo is an example of how to compose a proxy component to intercept sqlite executions in the Spin World.

## Repo structure

The `sqlite-proxy/` directory contains a proxy to intercept executions against a sqlite database.

The `example/` directory contains a Spin application which consists of one http handler which returns data from a sqlite database in the body. In the `spin.toml` file, the component build instructions point to a `build.sh` script which builds the example component and composes it with the sqlite-middleware component.

## Demo instructions

### Pre-requisites

- Install [cargo component](https://github.com/bytecodealliance/cargo-component):

```bash
cargo install --git https://github.com/bytecodealliance/cargo-component cargo-component
```

- Install latest [Spin](https://github.com/fermyon/spin)

### Build the components and run the demo

```bash

# Build the Spin application and sqlite-proxy. Spin build runs the `example/build.sh` script.
spin build

# Build and run the example
spin up --sqlite @db.sql

# Curl http://127.0.0.1:3000/login in a browser
curl -i http://localhost:3000
```

Try changing the Spin app to run without the sqlite-proxy, by changing the wasm file used in the `spin.toml` file:

1. Uncomment `source = "target/wasm32-wasi/release/example.wasm"`
2. Comment out `#source = "service.wasm"`

```toml
[component.example]
#source = "service.wasm"
source = "target/wasm32-wasi/release/example.wasm"
```

Now deploy your application.

```sh
$ spin deploy
Uploading example version 0.1.0 to Fermyon Cloud...
Deploying...
Waiting for application to become ready............. ready
Available Routes:
  example: https://example-12345.fermyon.app (wildcard)
```

In the example deploy output above, the app now exists at endpoint `https://example-12345.fermyon.app`.
