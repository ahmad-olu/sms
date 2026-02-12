# Run API
api:
    cargo run -p api

# Run Leptos dev server
# cd crates/web && cargo leptos watch
web:
    cargo leptos watch -p web

# Run all tests
test:
    cargo test --workspace

# Check everything
check:
    cargo check --workspace

# Format
fmt:
    cargo fmt --all

# Clippy
lint:
    cargo clippy --workspace --all-targets --all-features -- -D warnings

# Community build
build:
    cargo build --workspace

# Enterprise build
build-enterprise:
    cargo build --workspace --features enterprise

# Run enterprise
enterprise:
    cargo run -p api --features enterprise

set shell := ["bash", "-cu"]

run pkg:
    cargo run -p {{pkg}}

# Format + lint
ci:
    cargo fmt --all
    cargo clippy --workspace --all-targets --all-features -- -D warnings
    cargo test --workspace
