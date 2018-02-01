# jemallocator

[![Build Status](https://travis-ci.org/alexcrichton/jemallocator.svg?branch=master)](https://travis-ci.org/alexcrichton/jemallocator)

[Documentation](https://docs.rs/jemallocator)

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
#![feature(global_allocator)]
extern crate jemallocator;

#[global_allocator]
static ALLOC: jemallocator::Jemalloc = jemallocator::Jemalloc;
```

And that's it! Once you've linked to this crate then jemalloc will be used for
all allocations which happen in the crate itself.


# License

This project is licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or
   http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or
   http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in jemallocator by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
