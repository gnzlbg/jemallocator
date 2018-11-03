# jemallocator

[![Travis-CI Status]][travis] [![Appveyor Status]][appveyor] [![Latest Version]][crates.io] [![docs]][docs.rs]

> Links against `jemalloc` and provides a `Jemalloc` unit type that implements
> the allocator APIs and can be set as the `#[global_allocator]`

## Documentation

* [Latest release (docs.rs)][docs.rs]
* [Master branch][master_docs]

To use `jemallocator` add it as a dependency:

```toml
# Cargo.toml
[dependencies]
jemallocator = "0.1.8"
```

To set `jemallocator::Jemalloc` as the global allocator add this to your project:

```rust
extern crate jemallocator;

#[global_allocator]
static ALLOC: jemallocator::Jemalloc = jemallocator::Jemalloc;
```

And that's it! Once you've defined this `static` then jemalloc will be used for
all allocations requested by Rust code in the same program.

## Platform support

The following table describes the supported platforms: 

* `build`: does the library compile for the target?
* `run`: do the tests pass on the target?
* `valgrind`: do the tests pass under valgrind?

Tier 1 targets are tested on all Rust channels (stable, beta, and nightly). All
other targets are only tested on Rust nightly.

| Linux targets:                      | build     | run     | valgrind     |
|-------------------------------------|-----------|---------|--------------|
| `aarch64-unknown-linux-gnu`         | ✓         | ✓       | ✗            |
| `arm-unknown-linux-gnueabi`         | ✓         | ✓       | ✗            |
| `armv7-unknown-linux-gnueabi`       | ✓         | ✓       | ✗            |
| `i586-unknown-linux-gnu`            | ✓         | ✓       | ✗            |
| `i686-unknown-linux-gnu` (tier 1)   | ✓         | ✓       | ✗            |
| `mips-unknown-linux-gnu`            | ✓         | ✓       | ✗            |
| `mipsel-unknown-linux-musl`         | ✓         | ✓       | ✗            |
| `mips64-unknown-linux-gnuabi64`     | ✓         | ✓       | ✗            |
| `mips64el-unknown-linux-gnuabi64`   | ✓         | ✓       | ✗            |
| `powerpc-unknown-linux-gnu`         | ✓         | ✓       | ✗            |
| `powerpc64-unknown-linux-gnu`       | ✓         | ✓       | ✗            |
| `powerpc64le-unknown-linux-gnu`     | ✓         | ✓       | ✗            |
| `x86_64-unknown-linux-gnu` (tier 1) | ✓         | ✓       | ✓            |
| **MacOSX targets:**                 | **build** | **run** | **valgrind** |
| `x86_64-apple-darwin` (tier 1)      | ✓         | ✓       | ✗            |
| `i686-apple-darwin` (tier 1)        | ✓         | ✓       | ✗            |
| **Windows targets:**                | **build** | **run** | **valgrind** |
| `x86_64-pc-windows-msvc` (tier 1)   | ✗         | ✗       | ✗            |
| `i686-pc-windows-msvc` (tier 1)     | ✗         | ✗       | ✗            |
| `x86_64-pc-windows-gnu` (tier 1)    | ✓         | ✓       | ✗            |
| `i686-pc-windows-gnu` (tier 1)      | ✓         | ✓       | ✗            |
| **Android targets:**                | **build** | **run** | **valgrind** |
| `aarch64-linux-android`             | ✓         | ✓       | ✗            |
| `x86_64-linux-android`              | ✓         | ✓       | ✗            |

## Features

The `jemallocator` crate re-exports the [features of the `jemalloc-sys`
dependency](https://github.com/alexcrichton/jemallocator/blob/master/jemalloc-sys/readme.md).

## License

This project is licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or
   http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or
   http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in `jemallocator` by you, as defined in the Apache-2.0 license,
shall be dual licensed as above, without any additional terms or conditions.

[travis]: https://travis-ci.org/alexcrichton/jemallocator
[Travis-CI Status]: https://travis-ci.org/alexcrichton/jemallocator.svg?branch=master
[appveyor]: https://ci.appveyor.com/project/alexcrichton/jemallocator/branch/master
[Appveyor Status]: https://ci.appveyor.com/api/projects/status/github/alexcrichton/jemallocator?branch=master&svg=true
[Latest Version]: https://img.shields.io/crates/v/jemallocator.svg
[crates.io]: https://crates.io/crates/jemallocator
[docs]: https://docs.rs/jemallocator/badge.svg
[docs.rs]: https://docs.rs/jemallocator/
[master_docs]: https://alexcrichton.github.io/jemallocator/jemallocator
