#![feature(global_allocator, allocator_api)]

extern crate libc;
extern crate jemallocator;

use std::heap::{Alloc, Layout};
use jemallocator::Jemalloc;

#[global_allocator]
static A: Jemalloc = Jemalloc;

#[test]
fn smoke() {
    let layout = Layout::from_size_align(100, 8).unwrap();
    unsafe {
        let ptr = Jemalloc.alloc(layout.clone()).unwrap_or_else(|_| panic!());
        Jemalloc.dealloc(ptr, layout);
    }
}

#[test]
fn test_mallctl() {
    let mut epoch: u64 = 0;
    unsafe {
        assert_eq!(jemallocator::mallctl_fetch(b"", &mut epoch), Err(libc::EINVAL));
        assert_eq!(jemallocator::mallctl_fetch(b"epoch", &mut epoch),
                   Err(libc::EINVAL));
        jemallocator::mallctl_fetch(b"epoch\0", &mut epoch).unwrap();
        assert!(epoch > 0);
        assert_eq!(jemallocator::mallctl_set(b"", epoch), Err(libc::EINVAL));
        assert_eq!(jemallocator::mallctl_set(b"epoch", epoch), Err(libc::EINVAL));
        jemallocator::mallctl_set(b"epoch\0", epoch).unwrap();
    }
}
