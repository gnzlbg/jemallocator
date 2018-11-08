extern crate jemalloc_ctl;
extern crate jemallocator;
extern crate libc;

use jemalloc_ctl::raw::{get, set};
use jemallocator::Jemalloc;
use std::alloc::{GlobalAlloc, Layout};

#[global_allocator]
static A: Jemalloc = Jemalloc;

#[test]
fn smoke() {
    let layout = Layout::from_size_align(100, 8).unwrap();
    unsafe {
        let ptr = Jemalloc.alloc(layout.clone());
        assert!(!ptr.is_null());
        Jemalloc.dealloc(ptr, layout);
    }
}

#[test]
fn ctl_get_set() {
    let epoch = get::<u64>(b"epoch\0").unwrap();
    assert!(epoch > 0);
    set(b"epoch\0", epoch).unwrap();
}

#[test]
#[should_panic]
fn ctl_panic_empty_get() {
    let _ = get::<u64>(b"").unwrap();
}

#[test]
#[should_panic]
fn ctl_panic_empty_set() {
    let epoch = get::<u64>(b"epoch\0").unwrap();
    set(b"", epoch).unwrap();
}

#[test]
#[should_panic]
fn ctl_panic_non_null_terminated_get() {
    let _ = get::<u64>(b"epoch").unwrap();
}

#[test]
#[should_panic]
fn ctl_panic_non_null_terminated_set() {
    let epoch = get::<u64>(b"epoch\0").unwrap();
    set(b"epoch", epoch).unwrap();
}
