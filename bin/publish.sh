#!/usr/bin/env bash

SCRIPT_BASEDIR=$(dirname "$0")

which cargo &> /dev/null || { echo 'ERROR: cargo not found in PATH'; exit 1; }

cd "${SCRIPT_BASEDIR}/.."
pwd

git status
git diff

set -x
cargo publish --allow-dirty --package parameters_lib
cargo publish --allow-dirty --package parameters
