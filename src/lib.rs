// Copyright 2015 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Bindings for jemalloc as an allocator
//!
//! This crate provides bindings to jemalloc as a memory allocator for Rust.
//! This crate mainly exports, one type, `Jemalloc`, which implements the
//! `Alloc` trait and is suitable both as a memory allocator and as a
//! global allocator.

#![feature(allocator_api)]
#![deny(missing_docs)]

extern crate jemalloc_sys as ffi;
extern crate libc;

use std::mem;
use std::ptr;
use std::heap::{Alloc, Layout, Excess, CannotReallocInPlace, AllocErr, System};

use libc::{c_int, c_void};

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

/// Handle to the jemalloc allocator
///
/// This type and a reference to this type both implement the `Alloc` trait,
///
/// allowing usage of this `Jemalloc` type both in collections and as a global
/// allocator.
pub struct Jemalloc;

unsafe impl Alloc for Jemalloc {
    #[inline]
    unsafe fn alloc(&mut self, layout: Layout) -> Result<*mut u8, AllocErr> {
        (&*self).alloc(layout)
    }

    #[inline]
    unsafe fn alloc_zeroed(&mut self, layout: Layout)
        -> Result<*mut u8, AllocErr>
    {
        (&*self).alloc_zeroed(layout)
    }

    #[inline]
    unsafe fn dealloc(&mut self, ptr: *mut u8, layout: Layout) {
        (&*self).dealloc(ptr, layout)
    }

    #[inline]
    unsafe fn realloc(&mut self,
                      ptr: *mut u8,
                      old_layout: Layout,
                      new_layout: Layout) -> Result<*mut u8, AllocErr> {
        (&*self).realloc(ptr, old_layout, new_layout)
    }

    fn oom(&mut self, err: AllocErr) -> ! {
        (&*self).oom(err)
    }

    #[inline]
    fn usable_size(&self, layout: &Layout) -> (usize, usize) {
        (&self).usable_size(layout)
    }

    #[inline]
    unsafe fn alloc_excess(&mut self, layout: Layout) -> Result<Excess, AllocErr> {
        (&*self).alloc_excess(layout)
    }

    #[inline]
    unsafe fn realloc_excess(&mut self,
                             ptr: *mut u8,
                             layout: Layout,
                             new_layout: Layout) -> Result<Excess, AllocErr> {
        (&*self).realloc_excess(ptr, layout, new_layout)
    }

    #[inline]
    unsafe fn grow_in_place(&mut self,
                            ptr: *mut u8,
                            layout: Layout,
                            new_layout: Layout) -> Result<(), CannotReallocInPlace> {
        (&*self).grow_in_place(ptr, layout, new_layout)
    }

    #[inline]
    unsafe fn shrink_in_place(&mut self,
                              ptr: *mut u8,
                              layout: Layout,
                              new_layout: Layout) -> Result<(), CannotReallocInPlace> {
        (&*self).shrink_in_place(ptr, layout, new_layout)
    }
}

unsafe impl<'a> Alloc for &'a Jemalloc {
    #[inline]
    unsafe fn alloc(&mut self, layout: Layout) -> Result<*mut u8, AllocErr> {
        let flags = align_to_flags(layout.align());
        let ptr = ffi::mallocx(layout.size(), flags);
        if ptr.is_null() {
            Err(AllocErr::Exhausted { request: layout })
        } else {
            Ok(ptr as *mut u8)
        }
    }

    #[inline]
    unsafe fn alloc_zeroed(&mut self, layout: Layout)
        -> Result<*mut u8, AllocErr>
    {
        let ptr = if layout.align() <= MIN_ALIGN {
            ffi::calloc(layout.size(), 1)
        } else {
            let flags = align_to_flags(layout.align()) | ffi::MALLOCX_ZERO;
            ffi::mallocx(layout.size(), flags)
        };
        if ptr.is_null() {
            Err(AllocErr::Exhausted { request: layout })
        } else {
            Ok(ptr as *mut u8)
        }
    }

    #[inline]
    unsafe fn dealloc(&mut self, ptr: *mut u8, layout: Layout) {
        let flags = align_to_flags(layout.align());
        ffi::sdallocx(ptr as *mut c_void, layout.size(), flags)
    }

    #[inline]
    unsafe fn realloc(&mut self,
                      ptr: *mut u8,
                      old_layout: Layout,
                      new_layout: Layout) -> Result<*mut u8, AllocErr> {
        if old_layout.align() != new_layout.align() {
            return Err(AllocErr::Unsupported { details: "cannot change align" })
        }
        let flags = align_to_flags(new_layout.align());
        let ptr = ffi::rallocx(ptr as *mut c_void, new_layout.size(), flags);
        if ptr.is_null() {
            Err(AllocErr::Exhausted { request: new_layout })
        } else {
            Ok(ptr as *mut u8)
        }
    }

    fn oom(&mut self, err: AllocErr) -> ! {
        System.oom(err)
    }

    #[inline]
    fn usable_size(&self, layout: &Layout) -> (usize, usize) {
        let flags = align_to_flags(layout.align());
        unsafe {
            let max = ffi::nallocx(layout.size(), flags);
            (layout.size(), max)
        }
    }

    #[inline]
    unsafe fn grow_in_place(&mut self,
                            ptr: *mut u8,
                            old_layout: Layout,
                            new_layout: Layout) -> Result<(), CannotReallocInPlace> {
        self.shrink_in_place(ptr, old_layout, new_layout)
    }

    #[inline]
    unsafe fn shrink_in_place(&mut self,
                              ptr: *mut u8,
                              old_layout: Layout,
                              new_layout: Layout) -> Result<(), CannotReallocInPlace> {
        if old_layout.align() != new_layout.align() {
            return Err(CannotReallocInPlace)
        }
        let flags = align_to_flags(new_layout.align());
        let size = ffi::xallocx(ptr as *mut c_void, new_layout.size(), 0, flags);
        if size >= new_layout.size() {
            Err(CannotReallocInPlace)
        } else {
            Ok(())
        }
    }
}

/// Return the usable size of the allocation pointed to by ptr.
///
/// The return value may be larger than the size that was requested during allocation.
/// This function is not a mechanism for in-place `realloc()`;
/// rather it is provided solely as a tool for introspection purposes.
/// Any discrepancy between the requested allocation size
/// and the size reported by this function should not be depended on,
/// since such behavior is entirely implementation-dependent.
///
/// # Unsafety
///
/// `ptr` must have been allocated by `Jemalloc` and must not have been freed yet.
pub unsafe fn usable_size<T: Sized>(ptr: *const T) -> usize {
    ffi::malloc_usable_size(ptr as *const c_void)
}

/// Fetch the value of options `name`.
///
/// Please note that if you want to fetch a string, use char* instead of &str or
/// cstring.
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
/// Please note that if you want to set a string, use char* instead of &str or
/// cstring.
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
