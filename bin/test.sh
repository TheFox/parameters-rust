#!/usr/bin/env bash

SCRIPT_BASEDIR=$(dirname "$0")
TEST_OPTS=${TEST_OPTS:-}

which cargo &> /dev/null || { echo 'ERROR: cargo not found in PATH'; exit 1; }

cd "${SCRIPT_BASEDIR}/.."

export RUST_BACKTRACE=full

echo "TEST_OPTS: '${TEST_OPTS}'"

set -x

cargo test ${TEST_OPTS} $*
