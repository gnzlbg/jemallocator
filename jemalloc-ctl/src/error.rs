//! Error type
#![cfg_attr(
    feature = "cargo-clippy",
    allow(clippy::cast_sign_loss, clippy::cast_possible_wrap)
)]

use libc::c_int;
use {fmt, num, result};

pub trait NonZeroT {
    type T;
}
impl NonZeroT for i32 {
    type T = num::NonZeroU32;
}
impl NonZeroT for i64 {
    type T = num::NonZeroU64;
}

pub type NonZeroCInt = <c_int as NonZeroT>::T;

/// Errors of the `jemalloc_sys::mallct`-family of functions.
///
/// The `jemalloc-sys` crate: `mallctl`, `mallctlnametomib`, and `mallctlbymib``
/// functions return `0` on success; otherwise they return an error value.
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq)]
pub struct Error(NonZeroCInt);

/// Result type
pub type Result<T> = result::Result<T, Error>;

impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.0.get() as c_int {
            libc::EINVAL => write!(
                f,
                "`newp` is not `NULL`, and `newlen` is too large or too \
                 small. Alternatively, `*oldlenp` is too large or too \
                 small; in this case as much data as possible are read \
                 despite the error."
            ),
            libc::ENOENT => write!(
                f,
                "`name` or `mib` specifies an unknown/invalid value."
            ),
            libc::EPERM => write!(
                f,
                "Attempt to read or write `void` value, or attempt to \
                 write read-only value."
            ),
            libc::EAGAIN => write!(f, "A memory allocation failure occurred."),
            libc::EFAULT => write!(
                f,
                "An interface with side effects failed in some way not \
                 directly related to `mallctl*()` read/write processing."
            ),
            v => write!(f, "Unknown error code: \"{}\".", v),
        }
    }
}

pub(crate) fn cvt(ret: c_int) -> Result<()> {
    match ret {
        0 => Ok(()),
        v => Err(Error(unsafe { NonZeroCInt::new_unchecked(v as _) })),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn size_of_result_error() {
        use mem::size_of;
        assert_eq!(size_of::<Result<()>>(), size_of::<Error>());
        assert_eq!(size_of::<Error>(), size_of::<libc::c_int>());
    }
}
