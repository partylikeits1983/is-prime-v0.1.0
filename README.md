# 1) 
Install the `midenc` compiler and `cargo-miden` extension:

```bash
cargo install cargo-miden
cargo install midenc --locked
```

# 2) 

*Following the tutorial here: https://0xmiden.github.io/compiler/guides/rust_to_wasm.html*

Building & Running:
```
cargo build --release --target=wasm32-wasip1
midenc compile --entrypoint is_prime::entrypoint --emit masm=is_prime.masm,masp  target/wasm32-wasip1/release/is_prime.wasm
midenc run is_prime.masp --inputs inputs.toml
```
