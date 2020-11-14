#!/usr/bin/env bash
set -eou pipefail
script_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")" >/dev/null 2>&1 && pwd)"

docker build \
  --file "${script_dir}/Dockerfile.sdk" \
  --tag "matthewjamesbriggs/exilesdk:latest" \
  "${script_dir}"
