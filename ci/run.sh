#!/usr/bin/env bash

set -e

: ${TARGET?"The TARGET environment variable must be set."}

echo "Running tests for target: ${TARGET}, Rust version=${TRAVIS_RUST_VERSION}"
export RUST_BACKTRACE=1
export RUST_TEST_THREADS=1
export RUST_TEST_NOCAPTURE=1
export CARGO_CMD=cross

# Native CI platforms able to natively run binaries compiled for ${TARGET}.
if [[ ${TARGET} = *"windows"* ]] || \
       [[ ${TARGET} = *"x86_64-unknown-linux-gnu"* ]] \
       || [[ ${TARGET} = *"i686-unknown-linux-gnu"* ]] \
       || [[ ${TARGET} = *"i586-unknown-linux-gnu"* ]] \
       || [[ ${TARGET} = *"apple"* ]]; then
    # Runs jemalloc tests using "make check":
    export JEMALLOC_SYS_RUN_TESTS=1

    # Use cargo on native CI platforms:
    export CARGO_CMD=cargo
else
    cargo install cross || echo "cross is already installed"
fi

if [[ ${TARGET} = *"x86_64-unknown-linux-gnu"* ]]; then
    export JEMALLOC_SYS_VERIFY_CONFIGURE=1
fi

# Make sure TARGET is installed when using cargo:
if [[ ${CARGO_CMD} == "cargo" ]]; then
    rustup target add ${TARGET} || true
fi

# Check that no-std builds are not linked against a libc with default features that
# links std:
if [[ ${TARGET} = *"x86_64-unknown-linux-gnu"* ]] \
       || [[ ${TARGET} = *"apple"* ]]; then
    ${CARGO_CMD} build -vv --target $TARGET --verbose --no-default-features 2>&1 | tee build.txt
    cat build.txt | grep -q "default"
    cat build.txt | grep -q "use_std"
    # Make sure that the resulting build contains no std symbols
    ! find target/ -name *.rlib -exec nm {} \; | grep "std"
fi

${CARGO_CMD} test -vv --target $TARGET
${CARGO_CMD} test -vv --target $TARGET --features profiling
${CARGO_CMD} test -vv --target $TARGET --features debug
${CARGO_CMD} test -vv --target $TARGET --features stats
${CARGO_CMD} test -vv --target $TARGET --features 'debug profiling'
${CARGO_CMD} test -vv --target $TARGET --features unprefixed_malloc_on_supported_platforms
${CARGO_CMD} test -vv --target $TARGET --release
# FIXME: something broke in the toolchain leading to these
# two commands producing a stack overflow inside of cargo:
# https://github.com/alexcrichton/jemallocator/issues/61
#${CARGO_CMD} test -vv --target $TARGET -p jemalloc-sys
#${CARGO_CMD} test -vv --target $TARGET -p jemalloc-sys --features unprefixed_malloc_on_supported_platforms

if [[ ${TRAVIS_RUST_VERSION} == "nightly"  ]]; then
    # feature(global_allocator) is unstable:
    ${CARGO_CMD} test -vv --target $TARGET -p systest
    # The Alloc trait is unstable:
    ${CARGO_CMD} test -vv --target $TARGET --features alloc_trait

    # Run the examples
    . ci/examples.sh
fi
