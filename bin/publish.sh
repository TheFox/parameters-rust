#!/usr/bin/env bash

SCRIPT_BASEDIR=$(dirname "$0")

which cargo &> /dev/null || { echo 'ERROR: cargo not found in PATH'; exit 1; }

cd "${SCRIPT_BASEDIR}/.."
pwd

git status
git diff

pushd lib &> /dev/null
cargo publish
popd &> /dev/null

cargo publish
