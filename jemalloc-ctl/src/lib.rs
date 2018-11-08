//! `jemalloc` control and introspection.
//!
//! `jemalloc` offers a powerful introspection and control interface through the `mallctl` function.
//! It can be used to tune the allocator, take heap dumps, and retrieve statistics. This crate
//! provides a typed API over that interface.
//!
//! While `mallctl` takes a string to specify an operation (e.g. `stats.allocated` or
//! `stats.arenas.15.muzzy_decay_ms`), the overhead of repeatedly parsing those strings is not
//! ideal. Fortunately, `jemalloc` offers the ability to translate the string ahead of time into a
//! "Management Information Base" (MIB) to speed up future lookups.
//!
//! This crate provides both a function and a type for each `mallctl` operation. While the
//! function is more convenient, the type will be more efficient if the operation will be repeatedly
//! performed. Its constructor performs the MIB lookup, so the struct should be saved if the same
//! operation is going to be repeatedly performed.
//!
//! # Examples
//!
//! Repeatedly printing allocation statistics:
//!
//! ```no_run
//! extern crate jemallocator;
//! extern crate jemalloc_ctl;
//!
//! use std::thread;
//! use std::time::Duration;
//!
//! #[global_allocator]
//! static ALLOC: jemallocator::Jemalloc = jemallocator::Jemalloc;
//!
//! fn main() {
//!     loop {
//!         // many statistics are cached and only updated when the epoch is advanced.
//!         jemalloc_ctl::epoch().unwrap();
//!
//!         let allocated = jemalloc_ctl::stats::allocated().unwrap();
//!         let resident = jemalloc_ctl::stats::resident().unwrap();
//!         println!("{} bytes allocated/{} bytes resident", allocated, resident);
//!         thread::sleep(Duration::from_secs(10));
//!     }
//! }
//! ```
//!
//! Doing the same with the MIB-based API:
//!
//! ```no_run
//! extern crate jemallocator;
//! extern crate jemalloc_ctl;
//!
//! use std::thread;
//! use std::time::Duration;
//!
//! #[global_allocator]
//! static ALLOC: jemallocator::Jemalloc = jemallocator::Jemalloc;
//!
//! fn main() {
//!     let epoch = jemalloc_ctl::Epoch::new().unwrap();
//!     let allocated = jemalloc_ctl::stats::Allocated::new().unwrap();
//!     let resident = jemalloc_ctl::stats::Resident::new().unwrap();
//!     loop {
//!         // many statistics are cached and only updated when the epoch is advanced.
//!         epoch.advance().unwrap();
//!
//!         let allocated = allocated.get().unwrap();
//!         let resident = resident.get().unwrap();
//!         println!("{} bytes allocated/{} bytes resident", allocated, resident);
//!         thread::sleep(Duration::from_secs(10));
//!     }
//! }
//! ```
#![deny(missing_docs)]
#![cfg_attr(not(feature = "use_std"), no_std)]
#![cfg_attr(feature = "cargo-clippy", allow(clippy::stutter))]
#![feature(coerce_unsized)]

extern crate jemalloc_sys;
extern crate libc;

#[cfg(test)]
extern crate jemallocator;

#[cfg(test)]
#[global_allocator]
static ALLOC: jemallocator::Jemalloc = jemallocator::Jemalloc;

#[cfg(not(feature = "use_std"))]
use core as std;
use std::{fmt, mem, num, ops, ptr, result, slice, str};

pub mod arenas;
mod background_threads;
pub mod config;
mod epoch;
mod error;
mod keys;
pub mod opt;
pub mod raw;
pub mod stats;
#[cfg(feature = "use_std")]
pub mod stats_print;
pub mod thread;
mod version;

pub use background_threads::*;
pub use epoch::*;
pub use error::{Error, Result};
pub use keys::{Access, AsName, Mib, MibStr, Name};
pub use version::*;
