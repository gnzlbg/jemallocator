#!/usr/bin/env sh

set -ex

if rustup component add rustfmt-preview ; then
    command -v rustfmt
    rustfmt -V
    cargo fmt --all -- --check
fi

if rustup component add clippy-preview ; then
    cargo clippy -V
    cargo clippy -p jemalloc-sys -- -D clippy::pedantic
    cargo clippy -p jemallocator -- -D clippy::pedantic
    cargo clippy -p jemallocator-global -- -D clippy::pedantic
    cargo clippy -p jemalloc-ctl -- -D clippy::pedantic
fi

if shellcheck --version ; then
    shellcheck ci/*.sh
else
    echo "shellcheck not found"
    exit 1
fi
