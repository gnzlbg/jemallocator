#!/usr/bin/env bash

set -ex

: ${TARGET?"The TARGET environment variable must be set."}
: ${CARGO_CMD?"The CARGO_CMD  environment variable must be set."}

examples=(no_std vec)

# Build examples:
for ex in ${examples[*]}
do
    ${CARGO_CMD} build --release --manifest-path examples/${ex}/Cargo.toml --target ${TARGET}
done

# Run examples:
for ex in ${examples[*]}
do
    ${CARGO_CMD} run --release --manifest-path examples/${ex}/Cargo.toml --target ${TARGET}
done

# Run examples with valgrind in tier 1 targets:
#
# FIXME: i686-unknown-linux-gnu fails due to a missing symbol (strcpy)
# that valgrind tries to override
# FIXME: "i686-apple-darwin" fails with "cannot execute binary file"
# FIXME: "x86_64-apple-darwin" fails with a valgrind crash
if [[ ${TARGET} == "x86_64-unknown-linux-gnu" ]] \
       || [[ ${TARGET} == "x86_64-pc-windows-gnu" ]] \
       || [[ ${TARGET} == "i686-pc-windows-gnu" ]]; then
   for ex in ${examples[*]}
   do
       valgrind -v target/${TARGET}/release/${ex}
   done
fi

# Run examples with sanitizers on x86_64-unknown-linux-gnu
#
# Sanitizers provide their own global memory allocator, but
# one can still use jemalloc as long as its not set as the
# global allocator
if [[ ${TARGET} == "x86_64-unknown-linux-gnu" ]]; then
    san_examples=(no_std)
    for ex in ${san_examples[*]}
    do
        # AddressSanitizer
        # FIXME: this should also work with the global allocator
        RUSTFLAGS="-Z sanitizer=address" \
                 cargo build --release --manifest-path examples/${ex}/Cargo.toml \
                 --target ${TARGET} --features=no_global
        target/${TARGET}/release/${ex}

        # ThreadSanitizer
        RUSTFLAGS="-Z sanitizer=thread" \
                 cargo build --release --manifest-path examples/${ex}/Cargo.toml \
                 --target ${TARGET} --features=no_global
        # FIXME: times out on Travis
        #target/${TARGET}/release/${ex}

        # MemorySanitizer
        RUSTFLAGS="-Z sanitizer=memory" \
                 cargo build --release --manifest-path examples/${ex}/Cargo.toml \
                 --target ${TARGET} --features=no_global
        # FIXME: times out on Travis
        #target/${TARGET}/release/${ex}
    done
fi
