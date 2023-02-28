## Development

```shell
 cargo init --lib 
 cargo build --target=wasm32-unknown-unknown --release
# text format
 ../wabt/build/wasm2wat ./target/wasm32-unknown-unknown/release/rust_example.wasm
# debugging binary size
cargo install twiggy
twiggy top target/wasm32-unknown-unknown/release/rust_example.wasm
```