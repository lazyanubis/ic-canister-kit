#!/usr/bin/env bash

set -euo pipefail

features=(
    common times identity canister number token http ecdsa bitcoin
    functions call-once schedule
    stable canister-did full
)
examples=(template service storage assets)

cargo fmt --all -- --check
cargo test --all-features --locked
cargo clippy --all-targets --all-features --locked -- -D warnings
RUSTDOCFLAGS="-D warnings" cargo doc --no-deps --all-features --locked
cargo check --target wasm32-unknown-unknown --all-features --locked

for feature in "${features[@]}"; do
    cargo check --no-default-features --features "$feature" --locked
done

for example in "${examples[@]}"; do
    cargo check \
        --manifest-path "examples/${example}/Cargo.toml" \
        --target wasm32-unknown-unknown \
        --locked
done

if ! command -v cargo-deny >/dev/null; then
    echo "cargo-deny is required (expected version: 0.19.4)" >&2
    exit 1
fi
if ! command -v cargo-audit >/dev/null; then
    echo "cargo-audit is required (expected version: 0.22.2)" >&2
    exit 1
fi

cargo deny --all-features check --show-stats
cargo audit
