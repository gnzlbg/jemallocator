#!/usr/bin/env sh

set -ex

: "${TARGET?The TARGET environment variable must be set.}"

echo "Running tests for target: ${TARGET}, Rust version=${TRAVIS_RUST_VERSION}"
export RUST_BACKTRACE=1
export RUST_TEST_THREADS=1
export RUST_TEST_NOCAPTURE=1
export CARGO_CMD=cross

# Runs jemalloc tests using "make check":
#export JEMALLOC_SYS_RUN_TESTS=1

# Use cargo on native CI platforms:
case "${TARGET}" in
    "x86_64-unknown-linux-gnu") export CARGO_CMD=cargo ;;
    *"windows"*) export CARGO_CMD=cargo ;;
    *"apple"*) export CARGO_CMD=cargo ;;
esac

if [ "${CARGO_CMD}" = "cross" ]
then
    cargo install cross || echo "cross is already installed"
fi

${CARGO_CMD} test -vv --target "${TARGET}"
${CARGO_CMD} test -vv --target "${TARGET}" --features profiling
${CARGO_CMD} test -vv --target "${TARGET}" --features debug
${CARGO_CMD} test -vv --target "${TARGET}" --features stats
${CARGO_CMD} test -vv --target "${TARGET}" --features 'debug profiling'
${CARGO_CMD} test -vv --target "${TARGET}" --features unprefixed_malloc_on_supported_platforms
${CARGO_CMD} test -vv --target "${TARGET}" --release
${CARGO_CMD} test -vv --target "${TARGET}" -p jemalloc-sys
${CARGO_CMD} test -vv --target "${TARGET}" -p jemalloc-sys --features unprefixed_malloc_on_supported_platforms
${CARGO_CMD} test -vv --target "${TARGET}" -p systest

if [ "${TRAVIS_RUST_VERSION}" = "nightly"  ]
then
    # The Alloc trait is unstable:
    ${CARGO_CMD} test -vv --target "${TARGET}" --features alloc_trait
fi
