extern crate jemalloc_sys;
extern crate libc;

union U {
    x: &'static u8,
    y: &'static libc::c_char,
}

#[allow(non_upper_case_globals)]
#[cfg_attr(prefixed, export_name = "_rjem_malloc_conf")]
#[cfg_attr(not(prefixed), no_mangle)]
pub static malloc_conf: Option<&'static libc::c_char> = Some(unsafe {
    U {
        x: &b"abort:true\0"[0],
    }
    .y
});

#[test]
fn malloc_conf_set() {
    unsafe {
        assert_eq!(jemalloc_sys::malloc_conf, malloc_conf);
    }
}
