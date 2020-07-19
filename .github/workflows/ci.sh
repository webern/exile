#!/usr/bin/env bash
set -eo pipefail

# replicates the ci experience so you can check all the things before ci.
# you will need to make sure you have the latest rust toolchain:
# rustup update stable
DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" >/dev/null 2>&1 && pwd)"
REPO=$(cd "${DIR}/../.." && pwd)
echo $REPO

cd "${REPO}" && cargo clippy -- --version && cargo fmt -- --version &&
  cargo fmt -- --check &&
  cargo clippy --locked -- -D warnings &&
  cargo clippy --tests --locked -- -D warnings &&
  cargo test --all-features &&
  cargo build --package exile --lib
