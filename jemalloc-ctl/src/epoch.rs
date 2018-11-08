//! Epoch access.

use error::Result;
use keys::{Access, IntoName, Mib};

const EPOCH: &[u8] = b"epoch\0";

/// Advances the jemalloc epoch, returning it.
///
/// Many of the statistics tracked by jemalloc are cached. The epoch controls
/// when they are refreshed.
///
/// # Example
///
/// Advancing the epoch:
///
/// ```
/// extern crate jemallocator;
/// extern crate jemalloc_ctl;
///
/// #[global_allocator]
/// static ALLOC: jemallocator::Jemalloc = jemallocator::Jemalloc;
///
/// fn main() {
///     let a = jemalloc_ctl::epoch().unwrap();
///     let b = jemalloc_ctl::epoch().unwrap();
///     assert_eq!(a + 1, b);
/// }
/// ```
pub fn epoch() -> Result<u64> {
    EPOCH.name().read_write(1)
}

/// A type providing access to the jemalloc epoch.
///
/// Many of the statistics tracked by jemalloc are cached. The epoch controls
/// when they are refreshed.
///
/// # Example
///
/// Advancing the epoch:
///
/// ```
/// extern crate jemallocator;
/// extern crate jemalloc_ctl;
///
/// use jemalloc_ctl::Epoch;
///
/// #[global_allocator]
/// static ALLOC: jemallocator::Jemalloc = jemallocator::Jemalloc;
///
/// fn main() {
///     let epoch = Epoch::new().unwrap();
///
///     let a = epoch.advance().unwrap();
///     let b = epoch.advance().unwrap();
///     assert_eq!(a + 1, b);
/// }
#[derive(Copy, Clone)]
pub struct Epoch(Mib<[usize; 1]>);

impl Epoch {
    /// Returns a new `Epoch`.
    pub fn new() -> Result<Self> {
        let mib = EPOCH.name().mib()?;
        Ok(Epoch(mib))
    }

    /// Advances the epoch, returning it.
    ///
    /// The epoch advances by 1 every time it is advanced, so the value can be
    /// used to determine if another thread triggered a referesh.
    pub fn advance(self) -> Result<u64> {
        self.0.read_write(1)
    }
}
