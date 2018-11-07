extern crate jemalloc_ctl;
extern crate jemallocator;
extern crate libc;

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
fn test_jemalloc_ctl_get_set() {
    use jemalloc_ctl::{
        raw::{get, set},
        Error,
    };
    unsafe {
        {
            // get}
            assert_eq!(get::<u64>(b""), Err(Error::EINVAL));
            assert_eq!(get::<u64>(b"epoch"), Err(Error::EINVAL));
            let epoch = get::<u64>(b"epoch\0").unwrap();
            assert!(epoch > 0);
            assert_eq!(set(b"", epoch), Err(Error::EINVAL));
            assert_eq!(set(b"epoch", epoch), Err(Error::EINVAL));
            set(b"epoch\0", epoch).unwrap();
        }
    }
}
