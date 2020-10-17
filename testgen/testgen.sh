#!/usr/bin/env bash
set -eou pipefail
script_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")" >/dev/null 2>&1 && pwd)"
data_dir="${script_dir}/../data"
cd "${data_dir}"
data_dir=$(pwd)
workspace="${data_dir}/.."
cd "${workspace}"
workspace=$(pwd)

echo "script_dir  = ${script_dir}"
echo "data_dir    = ${data_dir}"
echo "workspace   = ${workspace}"

docker build -f "${script_dir}/Dockerfile" --tag testgenimage "${script_dir}"
docker run \
    -v "${script_dir}/xmltestgen:/xmltestgen" \
    -v "${data_dir}:/data" \
    -v "${workspace}:/workspace" \
    testgenimage
