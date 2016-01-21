// Copyright 2015 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

extern crate libc;

use libc::{c_int, c_void, size_t};

extern {
    pub fn mallocx(size: size_t, flags: c_int) -> *mut c_void;
    pub fn rallocx(ptr: *mut c_void, size: size_t,
                   flags: c_int) -> *mut c_void;
    pub fn xallocx(ptr: *mut c_void, size: size_t, extra: size_t,
                      flags: c_int) -> size_t;
    pub fn sdallocx(ptr: *mut c_void, size: size_t, flags: c_int);
    pub fn nallocx(size: size_t, flags: c_int) -> size_t;
}
