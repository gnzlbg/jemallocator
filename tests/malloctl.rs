extern crate libc;
extern crate jemallocator as alloc;

#[test]
fn smoke() {
    let ptr = alloc::__rust_allocate(100, 8);
    assert!(!ptr.is_null());
    alloc::__rust_deallocate(ptr, 100, 8);
}

#[test]
fn test_mallctl() {
    let mut epoch: u64 = 0;
    unsafe {
        assert_eq!(alloc::mallctl_fetch(b"", &mut epoch), Err(libc::EINVAL));
        assert_eq!(alloc::mallctl_fetch(b"epoch", &mut epoch),
                   Err(libc::EINVAL));
        alloc::mallctl_fetch(b"epoch\0", &mut epoch).unwrap();
        assert!(epoch > 0);
        assert_eq!(alloc::mallctl_set(b"", epoch), Err(libc::EINVAL));
        assert_eq!(alloc::mallctl_set(b"epoch", epoch), Err(libc::EINVAL));
        alloc::mallctl_set(b"epoch\0", epoch).unwrap();
    }
}
