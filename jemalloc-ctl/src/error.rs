//! Error type

use libc::c_int;
use {fmt, result};

/// Error of the `jemalloc_sys::mallct`-family of functions.
///
/// The `jemalloc-sys` crate: `mallctl`, `mallctlnametomib`, and `mallctlbymib``
/// functions return `0` on success; otherwise they return an error value.
#[derive(Copy, Clone, PartialEq)]
#[repr(i32)]
pub enum Error {
    /// Invalid argument.
    ///
    /// `newp` is not `NULL`, and `newlen` is too large or too small.
    /// Alternatively, `*oldlenp` is too large or too small; in this case as
    /// much data as possible are read despite the error.
    EINVAL = libc::EINVAL,
    /// Unknown/invalid `name` or `mib`.
    ENOENT = libc::ENOENT,
    /// Invalid access.
    ///
    /// Attempt to read or write `void` value, or attempt to write read-only
    /// value.
    EPERM = libc::EPERM,
    /// Memory allocation failure.
    EAGAIN = libc::EAGAIN,
    /// Interface with side-effects failed in some way not directly related to
    /// `mallctl*` read/write processing.
    EFAULT = libc::EFAULT,
}

/// Result type
pub type Result<T> = result::Result<T, Error>;

impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::EINVAL => write!(
                f,
                "`newp` is not `NULL`, and `newlen` is too large or too \
                 small. Alternatively, `*oldlenp` is too large or too \
                 small; in this case as much data as possible are read \
                 despite the error."
            ),
            Error::ENOENT => write!(f, "`name` or `mib` specifies an unknown/invalid value."),
            Error::EPERM => write!(
                f,
                "Attempt to read or write `void` value, or attempt to \
                 write read-only value."
            ),
            Error::EAGAIN => write!(f, "A memory allocation failure occurred."),
            Error::EFAULT => write!(
                f,
                "An interface with side effects failed in some way not \
                 directly related to `mallctl*()` read/write processing."
            ),
        }
    }
}

impl Error {
    fn new(e: c_int) -> Self {
        match e {
            libc::EINVAL => Error::EINVAL,
            libc::ENOENT => Error::ENOENT,
            libc::EPERM => Error::EPERM,
            libc::EAGAIN => Error::EAGAIN,
            libc::EFAULT => Error::EFAULT,
            v => panic!("unknown error code: {}", v),
        }
    }
}

pub(crate) fn cvt(ret: c_int) -> Result<()> {
    match ret {
        0 => Ok(()),
        v => Err(Error::new(v)),
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
