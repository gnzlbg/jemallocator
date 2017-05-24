// Copyright 2015 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![feature(allocator)]
#![allocator]
#![no_std]

extern crate jemalloc_sys as ffi;
extern crate libc;

use libc::{c_int, size_t, c_void};
use core::{mem, ptr};

// The minimum alignment guaranteed by the architecture. This value is used to
// add fast paths for low alignment values. In practice, the alignment is a
// constant at the call site and the branch will be optimized out.
#[cfg(all(any(target_arch = "arm",
              target_arch = "mips",
              target_arch = "mipsel",
              target_arch = "powerpc")))]
const MIN_ALIGN: usize = 8;
#[cfg(all(any(target_arch = "x86",
              target_arch = "x86_64",
              target_arch = "aarch64",
              target_arch = "powerpc64",
              target_arch = "powerpc64le")))]
const MIN_ALIGN: usize = 16;

const MALLOCX_ZERO: c_int = 0x40;

// MALLOCX_ALIGN(a) macro
fn mallocx_align(a: usize) -> c_int {
    a.trailing_zeros() as c_int
}

fn align_to_flags(align: usize) -> c_int {
    if align <= MIN_ALIGN {
        0
    } else {
        mallocx_align(align)
    }
}

#[no_mangle]
pub extern "C" fn __rust_allocate(size: usize, align: usize) -> *mut u8 {
    let flags = align_to_flags(align);
    unsafe { ffi::mallocx(size as size_t, flags) as *mut u8 }
}

#[no_mangle]
pub extern "C" fn __rust_allocate_zeroed(size: usize, align: usize) -> *mut u8 {
    let flags = align_to_flags(align) | MALLOCX_ZERO;
    unsafe { ffi::mallocx(size as size_t, flags) as *mut u8 }
}

#[no_mangle]
pub extern "C" fn __rust_reallocate(ptr: *mut u8,
                                    _old_size: usize,
                                    size: usize,
                                    align: usize)
                                    -> *mut u8 {
    let flags = align_to_flags(align);
    unsafe { ffi::rallocx(ptr as *mut c_void, size as size_t, flags) as *mut u8 }
}

#[no_mangle]
pub extern "C" fn __rust_reallocate_inplace(ptr: *mut u8,
                                            _old_size: usize,
                                            size: usize,
                                            align: usize)
                                            -> usize {
    let flags = align_to_flags(align);
    unsafe { ffi::xallocx(ptr as *mut c_void, size as size_t, 0, flags) as usize }
}

#[no_mangle]
pub extern "C" fn __rust_deallocate(ptr: *mut u8, old_size: usize, align: usize) {
    let flags = align_to_flags(align);
    unsafe { ffi::sdallocx(ptr as *mut c_void, old_size as size_t, flags) }
}

#[no_mangle]
pub extern "C" fn __rust_usable_size(size: usize, align: usize) -> usize {
    let flags = align_to_flags(align);
    unsafe { ffi::nallocx(size as size_t, flags) as usize }
}

// These symbols are used by jemalloc on android but the really old android
// we're building on doesn't have them defined, so just make sure the symbols
// are available.
#[no_mangle]
#[cfg(target_os = "android")]
pub extern "C" fn pthread_atfork(_prefork: *mut u8,
                                 _postfork_parent: *mut u8,
                                 _postfork_child: *mut u8)
                                 -> i32 {
    0
}

/// Fetch the value of options `name`.
///
/// Please note that if you want to fetch a string, use char* instead of &str or cstring.
pub unsafe fn mallctl_fetch<T>(name: &[u8], t: &mut T) -> Result<(), i32> {
    // make sure name is a valid c string.
    if name.is_empty() || *name.last().unwrap() != 0 {
        return Err(libc::EINVAL);
    }
    let mut t_size = mem::size_of::<T>();
    let t_ptr = t as *mut T as *mut _;
    let code = ffi::mallctl(name.as_ptr() as *const _,
                            t_ptr,
                            &mut t_size,
                            ptr::null_mut(),
                            0);
    if code != 0 {
        return Err(code);
    }
    Ok(())
}

/// Set a value to option `name`.
///
/// Please note that if you want to set a string, use char* instead of &str or cstring.
pub unsafe fn mallctl_set<T>(name: &[u8], mut t: T) -> Result<(), i32> {
    // make sure name is a valid c string.
    if name.is_empty() || *name.last().unwrap() != 0 {
        return Err(libc::EINVAL);
    }
    let size = mem::size_of::<T>();
    let code = ffi::mallctl(name.as_ptr() as *const _,
                            ptr::null_mut(),
                            ptr::null_mut(),
                            &mut t as *mut T as *mut _,
                            size);
    if code != 0 {
        return Err(code);
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use libc;

    #[test]
    fn smoke() {
        let ptr = super::__rust_allocate(100, 8);
        assert!(!ptr.is_null());
        super::__rust_deallocate(ptr, 100, 8);
    }

    #[test]
    fn test_mallctl() {
        let mut epoch: u64 = 0;
        unsafe {
            assert_eq!(super::mallctl_fetch(b"", &mut epoch), Err(libc::EINVAL));
            assert_eq!(super::mallctl_fetch(b"epoch", &mut epoch),
                       Err(libc::EINVAL));
            super::mallctl_fetch(b"epoch\0", &mut epoch).unwrap();
            assert!(epoch > 0);
            assert_eq!(super::mallctl_set(b"", epoch), Err(libc::EINVAL));
            assert_eq!(super::mallctl_set(b"epoch", epoch), Err(libc::EINVAL));
            super::mallctl_set(b"epoch\0", epoch).unwrap();
        }
    }
}
