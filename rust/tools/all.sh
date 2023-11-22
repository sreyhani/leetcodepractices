#!/usr/bin/env bash
set -eu

CURRENT_FILE_DIR="$(dirname "$(readlink -f "${BASH_SOURCE[0]}")")"
cd $CURRENT_FILE_DIR

./format.sh
./lint.sh
./test.sh
