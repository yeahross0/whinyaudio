## Build instructions

1. Make sure you have `wasm32-unknown-unknown` target installed in rustup (if not, do: `rustup target add wasm32-unknown-unknown`)
2. To build the example, do: `cargo build --target wasm32-unknown-unknown`
3. Copy the wasm file to this directory: `cp ../target/wasm32-unknown-unknown/debug/wasm-examples.wasm .`
4. Make sure you have `basic-http-server` installed (if not, do: `cargo install basic-http-server`).
5. Execute `basic-http-server` in `wasm-examples` directory.

If everything has succeeded, open a web browser at http://localhost:4000/, click the screen, and you should hear noise.
