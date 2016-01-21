#![allow(bad_style, improper_ctypes, dead_code, unused_imports)]
#![feature(alloc_system)]

extern crate alloc_system;
extern crate jemalloc_sys;
extern crate libc;

use libc::*;
use jemalloc_sys::*;

include!(concat!(env!("OUT_DIR"), "/all.rs"));
