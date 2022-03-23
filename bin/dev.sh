#!/usr/bin/env bash

SCRIPT_BASEDIR=$(dirname "$0")
export RUST_BACKTRACE=1
export RUSTFLAGS=-Awarnings
export APP_BUILD_AT=$(date)
export SYMF_DB_USER=user1
export SYMF_DB_PASS=password1
export SYMF_DB_PASS_PRODUCTION=password1
export SYMF_DB_PASS_PRODUCTION_INSTANCE1=password1
export SYMF_DB_PASS_PRODUCTION_INSTANCE2=password2

which cargo &> /dev/null || { echo 'ERROR: cargo not found in PATH'; exit 1; }

cd "${SCRIPT_BASEDIR}/.."

set -x
cargo run --bin parameters -- $*
