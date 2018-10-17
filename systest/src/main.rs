#![allow(bad_style, improper_ctypes, dead_code, unused_imports)]

extern crate jemalloc_sys;
extern crate libc;

use std::alloc::System;

#[global_allocator]
static A: System = System;

use libc::{c_int, c_void, c_char};
use jemalloc_sys::*;

include!(concat!(env!("OUT_DIR"), "/all.rs"));
