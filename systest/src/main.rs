#![allow(bad_style, improper_ctypes, dead_code, unused_imports)]

extern crate jemalloc_sys;
extern crate libc;

use std::alloc::System;

#[global_allocator]
static A: System = System;

use jemalloc_sys::*;
use libc::{c_char, c_int, c_void};

include!(concat!(env!("OUT_DIR"), "/all.rs"));
