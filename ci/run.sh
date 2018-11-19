#!/usr/bin/env sh

set -ex

: "${TARGET?The TARGET environment variable must be set.}"

echo "Running tests for target: ${TARGET}, Rust version=${TRAVIS_RUST_VERSION}"
export RUST_BACKTRACE=1
export RUST_TEST_THREADS=1
export RUST_TEST_NOCAPTURE=1
export CARGO_CMD=cross

# FIXME: workaround cargo breaking Travis-CI again:
# https://github.com/rust-lang/cargo/issues/5721
if [ "$TRAVIS" = "true" ]
then
    export TERM=dumb
fi

# Runs jemalloc tests when building jemalloc-sys (runs "make check"):
if [ "${NO_JEMALLOC_TESTS}" = "1" ]
then
    echo "jemalloc's tests are not run"
else
    export JEMALLOC_SYS_RUN_JEMALLOC_TESTS=1
fi

# Use cargo on native CI platforms:
case "${TARGET}" in
    "x86_64-unknown-linux-gnu") export CARGO_CMD=cargo ;;
    *"windows"*)
        export CARGO_CMD=cargo
        export NOBGT=1
        ;;
    *"apple"*) export CARGO_CMD=cargo ;;
esac

if [ "${CARGO_CMD}" = "cross" ]
then
    cargo install cross || echo "cross is already installed"
fi

if [ "${VALGRIND}" = "1" ]
then
    case "${TARGET}" in
        "x86_64-unknown-linux-gnu")
            export CARGO_TARGET_X86_64_UNKNOWN_LINUX_GNU_RUNNER=valgrind
            ;;
        "x86_64-apple-darwin")
            export CARGO_TARGET_X86_64_APPLE_DARWIN_RUNNER=valgrind
            ;;
        *)
            echo "Specify how to run valgrind for TARGET=${TARGET}"
            exit 1
            ;;
    esac
fi

${CARGO_CMD} test -vv --target "${TARGET}" 2>&1 | tee build_no_std.txt

if [ "${TARGET}" = "x86_64-unknown-linux-gnu" ] \
       || [ "${TARGET}" = "x86_64-apple-darwin" ]
then
    # Check that the no-std builds are not linked against a libc with default
    # features or the `use_std` feature enabled:
    ! grep -q "default" build_no_std.txt
    ! grep -q "use_std" build_no_std.txt

    RUST_SYS_ROOT=$(rustc --target="${TARGET}" --print sysroot)
    RUST_LLVM_NM="${RUST_SYS_ROOT}/lib/rustlib/${TARGET}/bin/llvm-nm"

    find target/ -iname '*jemalloc*.rlib' | while read -r rlib; do
        echo "${RUST_LLVM_NM} ${rlib}"
        ! $RUST_LLVM_NM "${rlib}" | grep "std"
    done
fi

${CARGO_CMD} test -vv --target "${TARGET}" \
             --no-default-features \
             --features debug,stats,background_threads_runtime_support,\
             unprefixed_malloc_on_supported_platforms
${CARGO_CMD} test -vv --target "${TARGET}" --no-default-features \
             --features background_threads_runtime_support

# jemalloc's tests fail on some targets when the profiling feature is enabled:
# https://github.com/jemalloc/jemalloc/issues/1320
# https://github.com/alexcrichton/jemallocator/issues/85
case "${TARGET}" in
    *"windows"*)
        unset JEMALLOC_SYS_RUN_JEMALLOC_TESTS
        
        ${CARGO_CMD} test -vv \
                     --target "${TARGET}" --features profiling

        if [ "${NO_JEMALLOC_TESTS}" = "1" ]
        then
            :
        else
            export JEMALLOC_SYS_RUN_JEMALLOC_TESTS=1
        fi

        ;;
    *)
        ${CARGO_CMD} test -vv --target "${TARGET}" --features profiling
        ;;
esac

if [ "${NOBGT}" = "1" ]
then
    echo "enabling background threads by default at run-time is not tested"
else
    ${CARGO_CMD} test -vv --target "${TARGET}" --features background_threads
fi

${CARGO_CMD} test -vv --target "${TARGET}" --release
${CARGO_CMD} test -vv --target "${TARGET}" --manifest-path jemalloc-sys/Cargo.toml
${CARGO_CMD} test -vv --target "${TARGET}" \
             --manifest-path jemalloc-sys/Cargo.toml \
             --features unprefixed_malloc_on_supported_platforms
${CARGO_CMD} test -vv --target "${TARGET}" -p systest
${CARGO_CMD} test -vv --target "${TARGET}" \
             --manifest-path jemallocator-global/Cargo.toml
${CARGO_CMD} test -vv --target "${TARGET}" \
             --manifest-path jemallocator-global/Cargo.toml \
             --features force_global_jemalloc

if [ "${TRAVIS_RUST_VERSION}" = "nightly"  ]
then
    # The Alloc trait is unstable:
    ${CARGO_CMD} test -vv --target "${TARGET}" --features alloc_trait
fi
