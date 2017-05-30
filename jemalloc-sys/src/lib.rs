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

extern "C" {
    #[cfg_attr(any(target_os = "macos", target_os = "android", target_os = "ios"),
               link_name = "je_mallocx")]
    pub fn mallocx(size: size_t, flags: c_int) -> *mut c_void;
    #[cfg_attr(any(target_os = "macos", target_os = "android", target_os = "ios"),
               link_name = "je_calloc")]
    pub fn calloc(size: size_t, flags: c_int) -> *mut c_void;
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

#[cfg(test)]
mod test {
    use libc::{c_void, c_char};
    use core::{ptr, mem};

    #[test]
    fn test_basic_alloc() {
        unsafe {
            let exp_size = super::nallocx(100, 0);
            assert!(exp_size >= 100);

            let mut ptr = super::mallocx(100, 0);
            assert!(!ptr.is_null());
            assert_eq!(exp_size, super::malloc_usable_size(ptr));
            ptr = super::rallocx(ptr, 50, 0);
            let size = super::xallocx(ptr, 30, 20, 0);
            assert!(size >= 50);
            super::sdallocx(ptr, 50, 0);
        }
    }

    #[test]
    fn test_mallctl() {
        let ptr = unsafe { super::mallocx(100, 0) };
        let mut allocated: usize = 0;
        let mut val_len = mem::size_of_val(&allocated);
        let field = "stats.allocated\0";
        let mut code;
        code = unsafe {
            super::mallctl(field.as_ptr() as *const _,
                           &mut allocated as *mut _ as *mut c_void,
                           &mut val_len,
                           ptr::null_mut(),
                           0)
        };
        assert_eq!(code, 0);
        assert!(allocated > 0);

        let mut mib = [0, 0];
        let mut mib_len = 2;
        code = unsafe {
            super::mallctlnametomib(field.as_ptr() as *const _, mib.as_mut_ptr(), &mut mib_len)
        };
        assert_eq!(code, 0);
        let mut allocated_by_mib = 0;
        let code = unsafe {
            super::mallctlbymib(mib.as_ptr(),
                                mib_len,
                                &mut allocated_by_mib as *mut _ as *mut c_void,
                                &mut val_len,
                                ptr::null_mut(),
                                0)
        };
        assert_eq!(code, 0);
        assert_eq!(allocated_by_mib, allocated);

        unsafe { super::sdallocx(ptr, 100, 0) };
    }

    #[test]
    fn test_stats() {
        struct PrintCtx {
            called_times: usize,
        }

        extern "C" fn write_cb(ctx: *mut c_void, _: *const c_char) {
            let print_ctx = unsafe { &mut *(ctx as *mut PrintCtx) };
            print_ctx.called_times += 1;
        }

        let mut ctx = PrintCtx { called_times: 0 };
        unsafe {
            super::malloc_stats_print(write_cb, &mut ctx as *mut _ as *mut c_void, ptr::null());
        }
        assert_ne!(ctx.called_times,
                   0,
                   "print should be triggered at lease once.");
    }
}
