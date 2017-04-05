// Copyright 2015 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![no_std]

extern crate libc;

use libc::{c_int, c_void, size_t};

extern "C" {
    #[cfg_attr(any(target_os = "macos", target_os = "android", target_os = "ios"),
               link_name = "je_mallocx")]
    pub fn mallocx(size: size_t, flags: c_int) -> *mut c_void;
    #[cfg_attr(any(target_os = "macos", target_os = "android", target_os = "ios"),
               link_name = "je_rallocx")]
    pub fn rallocx(ptr: *mut c_void, size: size_t, flags: c_int) -> *mut c_void;
    #[cfg_attr(any(target_os = "macos", target_os = "android", target_os = "ios"),
               link_name = "je_xallocx")]
    pub fn xallocx(ptr: *mut c_void, size: size_t, extra: size_t, flags: c_int) -> size_t;
    #[cfg_attr(any(target_os = "macos", target_os = "android", target_os = "ios"),
               link_name = "je_sdallocx")]
    pub fn sdallocx(ptr: *mut c_void, size: size_t, flags: c_int);
    #[cfg_attr(any(target_os = "macos", target_os = "android", target_os = "ios"),
               link_name = "je_nallocx")]
    pub fn nallocx(size: size_t, flags: c_int) -> size_t;
}
