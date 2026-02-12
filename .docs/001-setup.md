##1. Create the Workspace

```bash
mkdir sms
cd sms

cargo install --locked cargo-leptos
# for an Axum template
cargo leptos new --git https://github.com/leptos-rs/start-axum
rustup target add wasm32-unknown-unknown


cargo new sms-front
cargo new shared
```

sms/Cargo.toml
```bash
[workspace]
members = [
    "sms-backend",
    "sms-front",
    "shared"
]

resolver = "2"
```

NOTE: to run leptos `cargo leptos watch`
NOTE: Run the API (Axum server) `cargo run -p api`
NOTE: Run the WEB (leptos) `cargo leptos watch -p web`
