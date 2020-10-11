#!/usr/bin/env bash
set -eou pipefail
DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" >/dev/null 2>&1 && pwd)"
DATA="${DIR}/../data"
cd "${DATA}"
DATA=$(pwd)
cd "${DIR}"

docker build -f "${DIR}/Dockerfile" --tag testgenimage "${DIR}"
docker run -it \
    -v "${DIR}/xmltestgen:/xmltestgen" \
    -v "${DATA}:/data" \
    testgenimage \
    /bin/bash
