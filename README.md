# jemallocator

[![Build Status](https://travis-ci.org/alexcrichton/jemallocator.svg?branch=master)](https://travis-ci.org/alexcrichton/jemallocator)

[Documentation](http://alexcrichton.com/jemallocator)

A nightly-only Rust allocator crate which links to jemalloc and forces all Rust
allocations to use jemalloc as well.

Usage:

```toml
# Cargo.toml
[dependencies]
jemallocator = "0.1"
```

Rust:

```rust
extern crate jemallocator;
```

And that's it! Once you've linked to this crate then jemalloc will be used for
all allocations which happen in the crate itself.

# License

`jemallocator` is primarily distributed under the terms of both the MIT license
and the Apache License (Version 2.0), with portions covered by various BSD-like
licenses.

See LICENSE-APACHE, and LICENSE-MIT for details.
