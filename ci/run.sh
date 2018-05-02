#!/usr/bin/env bash

set -ex

: ${TARGET?"The TARGET environment variable must be set."}

echo "Running tests for target: ${TARGET}"
export RUST_BACKTRACE=1
export RUST_TEST_THREADS=1
export RUST_TEST_NOCAPTURE=1
export CARGO_CMD=cross

export JEMALLOC_SYS_RUN_TESTS=1

# FIXME when jemalloc 5.1 is released
# This fetches the jemalloc 5.1rc1 commit manually
cd jemalloc-sys
rm -rf jemalloc
git clone https://github.com/jemalloc/jemalloc.git
cd jemalloc
git checkout -b rc1 b8f4c730eff28edee4b583ff5b6ee1fac0f26c27
./autogen.sh
make distclean
cd ../..

# Use cargo on native CI platforms:
if [[ ${TARGET} = *"windows"* ]] || [[ ${TARGET} = *"x86_64-unknown-linux-gnu"* ]] || [[ ${TARGET} = *"apple"* ]]; then
    export CARGO_CMD=cargo
else
    cargo install cross
fi

# Make sure TARGET is installed when using cargo:
if [[ ${CARGO_CMD} == "cargo" ]]; then
    rustup target add ${TARGET} || true
fi

${CARGO_CMD} build --target $TARGET
${CARGO_CMD} build --target $TARGET --features profiling
${CARGO_CMD} build --target $TARGET --features debug
${CARGO_CMD} build --target $TARGET --features 'debug profiling'
${CARGO_CMD} test --target $TARGET
${CARGO_CMD} test --target $TARGET --release
