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
    // Standard API
    #[link_name = "_rjem_malloc"]
    pub fn malloc(size: size_t) -> *mut c_void;
    #[link_name = "_rjem_calloc"]
    pub fn calloc(number: size_t, size: size_t) -> *mut c_void;
    #[link_name = "_rjem_posix_memalign"]
    pub fn posix_memalign(ptr: *mut *mut c_void, alignment: size_t, size: size_t) -> c_int;
    #[link_name = "_rjem_aligned_alloc"]
    pub fn aligned_alloc(alignment: size_t, size: size_t) -> *mut c_void;
    #[link_name = "_rjem_realloc"]
    pub fn realloc(ptr: *mut c_void, size: size_t) -> *mut c_void;
    #[link_name = "_rjem_free"]
    pub fn free(ptr: *mut c_void);

    // Non-standard API
    #[link_name = "_rjem_mallocx"]
    pub fn mallocx(size: size_t, flags: c_int) -> *mut c_void;
    #[link_name = "_rjem_rallocx"]
    pub fn rallocx(ptr: *mut c_void, size: size_t, flags: c_int) -> *mut c_void;
    #[link_name = "_rjem_xallocx"]
    pub fn xallocx(ptr: *mut c_void, size: size_t, extra: size_t, flags: c_int) -> size_t;
    #[link_name = "_rjem_sallocx"]
    pub fn sallocx(ptr: *const c_void, flags: c_int) -> size_t;
    #[link_name = "_rjem_dallocx"]
    pub fn dallocx(ptr: *mut c_void, flags: c_int);
    #[link_name = "_rjem_sdallocx"]
    pub fn sdallocx(ptr: *mut c_void, size: size_t, flags: c_int);
    #[link_name = "_rjem_nallocx"]
    pub fn nallocx(size: size_t, flags: c_int) -> size_t;
    #[link_name = "_rjem_malloc_usable_size"]
    pub fn malloc_usable_size(ptr: *const c_void) -> size_t;

    // mallctl
    #[link_name = "_rjem_mallctl"]
    pub fn mallctl(name: *const c_char,
                   oldp: *mut c_void,
                   oldpenp: *mut size_t,
                   newp: *mut c_void,
                   newlen: size_t)
                   -> c_int;
    #[link_name = "_rjem_mallctlnametomib"]
    pub fn mallctlnametomib(name: *const c_char, mibp: *mut size_t, miblenp: *mut size_t) -> c_int;
    #[link_name = "_rjem_mallctlbymib"]
    pub fn mallctlbymib(mib: *const size_t,
                        miblen: size_t,
                        oldp: *mut c_void,
                        oldpenp: *mut size_t,
                        newp: *mut c_void,
                        newlen: size_t)
                        -> c_int;

    // stats
    #[link_name = "_rjem_malloc_stats_print"]
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

/// Computes `flags` from `align`.
///
/// Equivalent to the MALLOCX_ALIGN(a) macro.
#[inline]
pub fn MALLOCX_ALIGN(aling: usize) -> c_int {
    aling.trailing_zeros() as c_int
}
