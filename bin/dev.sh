#!/usr/bin/env bash

SCRIPT_BASEDIR=$(dirname "$0")

which cargo &> /dev/null || { echo 'ERROR: cargo not found in PATH'; exit 1; }

cd "${SCRIPT_BASEDIR}/.."

mkdir -p tmp

export RUST_BACKTRACE=1
export RUSTFLAGS=-Awarnings

set -x

cargo run -- $*
