# example-rust-axum

A minimal [axum](https://github.com/tokio-rs/axum) server deployed on UQBITZ with the CLI.

- Detected as `firecracker` → `rust-runtime`, built with `cargo build --release`.
- The platform sets `PORT` (3000 for Rust); the app binds `0.0.0.0:$PORT`.
- The binary is found automatically in `target/release/` — it does not need to be named `app`.

```sh
cargo run --release   # http://localhost:3000
```

License: MIT
