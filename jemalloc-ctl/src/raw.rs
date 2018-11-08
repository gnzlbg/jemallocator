//! Raw `malloctl` getter/setters

use error::{cvt, Error, Result};
use libc::c_char;
use {mem, ptr, slice};

/// Translates `name` to a `mib` (Management Information Base)
///
/// `mib`s are used to avoid repeated name lookups for applications that
/// repeatedly query the same portion of `jemalloc`s `mallctl` namespace.
///
/// On success, `mib` contains an array of integers. It is possible to pass
/// `mib` with a length smaller than the number of period-separated name
/// components. This results in a partial MIB that can be used as the basis for
/// constructing a complete MIB.
///
/// For name components that are integers (e.g. the `2` in `arenas.bin.2.size`),
/// the corresponding MIB component will always be that integer. Therefore, it
/// is legitimate to construct code like the following:
///
/// ```
/// extern crate libc;
/// extern crate jemallocator;
/// extern crate jemalloc_ctl;
///
/// #[global_allocator]
/// static ALLOC: jemallocator::Jemalloc = jemallocator::Jemalloc;
///
/// fn main() {
///     use jemalloc_ctl::raw;
///     use libc::{c_uint, c_char};
///     unsafe {
///         let mut mib = [0; 4];
///         let nbins: c_uint = raw::get(b"arenas.nbins\0").unwrap();
///         raw::name_to_mib(b"arenas.bin.0.size\0", &mut mib).unwrap();
///         for i in 0..4 {
///             mib[2] = i;
///             let bin_size: usize = raw::get_mib(&mut mib).unwrap();
///             println!("arena bin {} has size {}", i, bin_size);
///         }
///     }
/// }
/// ```
pub fn name_to_mib(name: &[u8], mib: &mut [usize]) -> Result<()> {
    unsafe {
        validate_name(name);

        let mut len = mib.len();
        cvt(jemalloc_sys::mallctlnametomib(
            name as *const _ as *const c_char,
            mib.as_mut_ptr(),
            &mut len,
        ))?;
        assert_eq!(mib.len(), len);
        Ok(())
    }
}

/// Uses the MIB `mib` as key to the _MALLCTL NAMESPACE_ and reads its value.
///
/// The [`name_to_mib`] API translates a string of the key (e.g. `arenas.nbins`)
/// to a `mib` (Management Information Base).
pub fn get_mib<T: Copy>(mib: &[usize]) -> Result<T> {
    unsafe {
        let mut value = MaybeUninit { init: () };
        let mut len = mem::size_of::<T>();
        cvt(jemalloc_sys::mallctlbymib(
            mib.as_ptr(),
            mib.len(),
            &mut value.init as *mut _ as *mut _,
            &mut len,
            ptr::null_mut(),
            0,
        ))?;
        assert_eq!(len, mem::size_of::<T>());
        Ok(value.maybe_uninit)
    }
}

/// Uses the null-terminated string `name` as key to the _MALLCTL NAMESPACE_ and
/// reads its value.
pub fn get<T: Copy>(name: &[u8]) -> Result<T> {
    unsafe {
        validate_name(name);

        let mut value = MaybeUninit { init: () };
        let mut len = mem::size_of::<T>();
        cvt(jemalloc_sys::mallctl(
            name as *const _ as *const c_char,
            &mut value.init as *mut _ as *mut _,
            &mut len,
            ptr::null_mut(),
            0,
        ))?;
        assert_eq!(len, mem::size_of::<T>());
        Ok(value.maybe_uninit)
    }
}

/// Uses the MIB `mib` as key to the _MALLCTL NAMESPACE_ and sets its `value`.
///
/// The [`name_to_mib`] API translates a string of the key (e.g. `arenas.nbins`)
/// to a `mib` (Management Information Base).
pub fn set_mib<T>(mib: &[usize], mut value: T) -> Result<()> {
    unsafe {
        cvt(jemalloc_sys::mallctlbymib(
            mib.as_ptr(),
            mib.len(),
            ptr::null_mut(),
            ptr::null_mut(),
            &mut value as *mut _ as *mut _,
            mem::size_of::<T>(),
        ))
    }
}

/// Uses the null-terminated string `name` as the key to the _MALLCTL NAMESPACE_
/// and sets it `value`
pub fn set<T>(name: &[u8], mut value: T) -> Result<()> {
    unsafe {
        validate_name(name);

        cvt(jemalloc_sys::mallctl(
            name as *const _ as *const c_char,
            ptr::null_mut(),
            ptr::null_mut(),
            &mut value as *mut _ as *mut _,
            mem::size_of::<T>(),
        ))
    }
}

/// Uses the MIB `mib` as key to the _MALLCTL NAMESPACE_ and sets its `value`
/// returning its previous value.
///
/// The [`name_to_mib`] API translates a string of the key (e.g. `arenas.nbins`)
/// to a `mib` (Management Information Base).
pub fn get_set_mib<T>(mib: &[usize], mut value: T) -> Result<T> {
    unsafe {
        let mut len = mem::size_of::<T>();
        cvt(jemalloc_sys::mallctlbymib(
            mib.as_ptr(),
            mib.len(),
            &mut value as *mut _ as *mut _,
            &mut len,
            &mut value as *mut _ as *mut _,
            len,
        ))?;
        assert_eq!(len, mem::size_of::<T>());
        Ok(value)
    }
}

/// Uses the null-terminated string `name` as key to the _MALLCTL NAMESPACE_ and
/// sets its `value` returning its previous value.
pub fn get_set<T>(name: &[u8], mut value: T) -> Result<T> {
    unsafe {
        validate_name(name);

        let mut len = mem::size_of::<T>();
        cvt(jemalloc_sys::mallctl(
            name as *const _ as *const c_char,
            &mut value as *mut _ as *mut _,
            &mut len,
            &mut value as *mut _ as *mut _,
            len,
        ))?;
        assert_eq!(len, mem::size_of::<T>());
        Ok(value)
    }
}

/// Uses the MIB `mib` as key to the _MALLCTL NAMESPACE_ and reads its value of
/// type `&str`
///
/// The [`name_to_mib`] API translates a string of the key (e.g. `arenas.nbins`)
/// to a `mib` (Management Information Base).
pub fn get_str_mib(mib: &[usize]) -> Result<&'static str> {
    unsafe {
        let ptr: *const c_char = get_mib(mib)?;
        ptr2str(ptr)
    }
}

/// Uses the null-terminated string `name` as key to the _MALLCTL NAMESPACE_ and
/// reads its value of type `&str`.
pub fn get_str(name: &[u8]) -> Result<&'static str> {
    unsafe {
        validate_name(name);

        let ptr: *const c_char = get(name)?;
        ptr2str(ptr)
    }
}

unsafe fn ptr2str(ptr: *const c_char) -> Result<&'static str> {
    let len = libc::strlen(ptr);
    let byte_slice = slice::from_raw_parts(ptr as *const u8, (len + 1) as _);
    core::str::from_utf8(byte_slice).map_err(|_| Error::EINVAL)
}

fn validate_name(name: &[u8]) {
    assert!(!name.is_empty(), "empty byte string");
    assert_eq!(
        *name.last().unwrap(),
        b'\0',
        "non-null terminated byte string"
    );
}

union MaybeUninit<T: Copy> {
    init: (),
    maybe_uninit: T,
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_ptr2str() {
        unsafe {
            //{ // This is undefined behavior:
            //    let cstr = b"";
            //    let rstr = ptr2str(cstr as *const _ as *const c_char);
            //    assert!(rstr.is_err());
            // }
            {
                let cstr = b"\0";
                let rstr = ptr2str(cstr as *const _ as *const c_char);
                assert!(rstr.is_ok());
                let rstr = rstr.unwrap();
                assert!(rstr.len() == 1);
                assert_eq!(rstr, "\0");
            }
            {
                let cstr = b"foo  baaar\0";
                let rstr = ptr2str(cstr as *const _ as *const c_char);
                assert!(rstr.is_ok());
                let rstr = rstr.unwrap();
                assert!(rstr.len() == "foo  baaar\0".len());
                assert_eq!(rstr, "foo  baaar\0");
            }
        }
    }
}
