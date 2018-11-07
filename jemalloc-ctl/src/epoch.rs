//! Epoch access.

use error::Result;
use raw::{get_set, get_set_mib, name_to_mib};

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
    get_set(EPOCH, 1)
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
pub struct Epoch([usize; 1]);

impl Epoch {
    /// Returns a new `Epoch`.
    pub fn new() -> Result<Self> {
        let mut mib = [0; 1];
        name_to_mib(EPOCH, &mut mib)?;
        Ok(Epoch(mib))
    }

    /// Advances the epoch, returning it.
    ///
    /// The epoch advances by 1 every time it is advanced, so the value can be
    /// used to determine if another thread triggered a referesh.
    pub fn advance(self) -> Result<u64> {
        get_set_mib(&self.0, 1)
    }
}
