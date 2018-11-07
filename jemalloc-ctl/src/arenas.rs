//! Arena operations.

use error::Result;
use libc::c_uint;
use raw::{get, get_mib, name_to_mib};

const NARENAS: &[u8] = b"arenas.narenas\0";

/// Returns the current limit on the number of arenas.
///
/// # Examples
///
/// ```
/// extern crate jemallocator;
/// extern crate jemalloc_ctl;
///
/// #[global_allocator]
/// static ALLOC: jemallocator::Jemalloc = jemallocator::Jemalloc;
///
/// fn main() {
///     println!(
///         "number of arenas: {}",
///         jemalloc_ctl::arenas::narenas().unwrap()
///     );
/// }
/// ```
pub fn narenas() -> Result<c_uint> {
    get(NARENAS)
}

/// A type providing access to the current limit on the number of arenas.
///
/// # Examples
///
/// ```
/// extern crate jemallocator;
/// extern crate jemalloc_ctl;
///
/// use jemalloc_ctl::arenas::NArenas;
///
/// #[global_allocator]
/// static ALLOC: jemallocator::Jemalloc = jemallocator::Jemalloc;
///
/// fn main() {
///     let narenas = NArenas::new().unwrap();
///
///     println!("number of arenas: {}", narenas.get().unwrap());
/// }
/// ```
#[derive(Copy, Clone)]
pub struct NArenas([usize; 2]);

impl NArenas {
    /// Returns a new `NArenas`.
    pub fn new() -> Result<Self> {
        let mut mib = [0; 2];
        name_to_mib(NARENAS, &mut mib)?;
        Ok(NArenas(mib))
    }

    /// Returns the maximum number of arenas.
    pub fn get(self) -> Result<c_uint> {
        get_mib(&self.0)
    }
}
