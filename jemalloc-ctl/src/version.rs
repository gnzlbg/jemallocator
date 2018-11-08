//! Version operations.

use error::Result;
use keys::{Access, IntoName, MibStr};

const VERSION: &[u8] = b"version\0";

/// Returns the jemalloc version string.
///
/// # Note
///
/// The version of jemalloc currently shipped with the Rust distribution has a bogus version string.
///
/// # Example
///
/// ```
/// extern crate jemallocator;
/// extern crate jemalloc_ctl;
///
/// #[global_allocator]
/// static ALLOC: jemallocator::Jemalloc = jemallocator::Jemalloc;
///
/// fn main() {
///     println!("jemalloc version {}", jemalloc_ctl::version().unwrap());
/// }
/// ```
pub fn version() -> Result<&'static str> {
    VERSION.name().read()
}

/// A type providing access to the jemalloc version string.
///
/// # Example
///
/// ```
/// extern crate jemallocator;
/// extern crate jemalloc_ctl;
///
/// use jemalloc_ctl::Version;
///
/// #[global_allocator]
/// static ALLOC: jemallocator::Jemalloc = jemallocator::Jemalloc;
///
/// fn main() {
///     let version = Version::new().unwrap();
///
///     println!("jemalloc version {}", version.get().unwrap());
/// }
#[derive(Copy, Clone)]
pub struct Version(MibStr<[usize; 1]>);

impl Version {
    /// Returns a new `Version`.
    pub fn new() -> Result<Self> {
        let mib = VERSION.name().mib_str()?;
        Ok(Version(mib))
    }

    /// Returns the jemalloc version string.
    pub fn get(self) -> Result<&'static str> {
        self.0.read()
    }
}
