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

use libc::{c_int, c_void, size_t, c_char};

pub const MALLOCX_ZERO: c_int = 0x40;

extern "C" {
    #[cfg_attr(any(target_os = "macos", target_os = "android", target_os = "ios"),
               link_name = "je_mallocx")]
    pub fn mallocx(size: size_t, flags: c_int) -> *mut c_void;
    #[cfg_attr(any(target_os = "macos", target_os = "android", target_os = "ios"),
               link_name = "je_calloc")]
    pub fn calloc(size: size_t, flags: size_t) -> *mut c_void;
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

    #[cfg_attr(any(target_os = "macos", target_os = "android", target_os = "ios"),
               link_name = "je_malloc_usable_size")]
    pub fn malloc_usable_size(ptr: *const c_void) -> size_t;

    // mallctl
    #[cfg_attr(any(target_os = "macos", target_os = "android", target_os = "ios"),
               link_name = "je_mallctl")]
    pub fn mallctl(name: *const c_char,
                   oldp: *mut c_void,
                   oldpenp: *mut size_t,
                   newp: *mut c_void,
                   newlen: size_t)
                   -> c_int;
    #[cfg_attr(any(target_os = "macos", target_os = "android", target_os = "ios"),
               link_name = "je_mallctlnametomib")]
    pub fn mallctlnametomib(name: *const c_char, mibp: *mut size_t, miblenp: *mut size_t) -> c_int;
    #[cfg_attr(any(target_os = "macos", target_os = "android", target_os = "ios"),
               link_name = "je_mallctlbymib")]
    pub fn mallctlbymib(mib: *const size_t,
                        miblen: size_t,
                        oldp: *mut c_void,
                        oldpenp: *mut size_t,
                        newp: *mut c_void,
                        newlen: size_t)
                        -> c_int;

    // stats
    #[cfg_attr(any(target_os = "macos", target_os = "android", target_os = "ios"),
               link_name = "je_malloc_stats_print")]
    pub fn malloc_stats_print(write_cb: extern "C" fn(*mut c_void, *const c_char),
                              cbopaque: *mut c_void,
                              opts: *const c_char);
}

// These symbols are used by jemalloc on android but the really old android
// we're building on doesn't have them defined, so just make sure the symbols
// are available.
#[no_mangle]
#[cfg(target_os = "android")]
#[doc(hidden)]
pub extern "C" fn pthread_atfork(_prefork: *mut u8,
                                 _postfork_parent: *mut u8,
                                 _postfork_child: *mut u8)
                                 -> i32 {
    0
}
