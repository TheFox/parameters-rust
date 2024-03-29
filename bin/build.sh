#!/usr/bin/env bash

SCRIPT_BASEDIR=$(dirname "$0")

which cargo &> /dev/null || { echo 'ERROR: cargo not found in PATH'; exit 1; }
which strip &> /dev/null || { echo 'ERROR: strip not found in PATH'; exit 1; }

cd "${SCRIPT_BASEDIR}/.."

set -e
cargo build -v --release
ls -la target/release/parameters

strip target/release/parameters
ls -la target/release/parameters
