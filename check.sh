#!/usr/bin/env bash
# runs the same set of build commands that will run in ci
set -e
cargo clippy -- --version && cargo fmt -- --version
cargo fmt -- --check
cargo clippy --locked -- -D warnings
cargo clippy --tests --locked -- -D warnings
cargo test --all-features
cargo build --package exile --lib