
Building & Running:
```
cargo build --release --target=wasm32-wasip1
midenc compile --entrypoint is_prime::entrypoint --emit masm=is_prime.masm,masp  target/wasm32-wasip1/release/is_prime.wasm
midenc run is_prime.masp --inputs inputs.toml
```# is-prime-v0.1.0
