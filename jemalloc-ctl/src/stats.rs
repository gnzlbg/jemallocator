//! Global allocator statistics.
//!
//! `jemalloc` tracks a wide variety of statistics. Many of them are cached, and only refreshed when
//! the jemalloc "epoch" is advanced. See the [`Epoch`] type for more information.
//!
//! [`Epoch`]: ../struct.Epoch.html

use error::Result;
use keys::{Access, IntoName, Mib};

const ALLOCATED: &[u8] = b"stats.allocated\0";

/// Returns the total number of bytes allocated by the application.
///
/// This statistic is cached, and is only refreshed when the epoch is advanced. See the [`epoch`]
/// function for more information.
///
/// This corresponds to `stats.allocated` in jemalloc's API.
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
///     let a = jemalloc_ctl::stats::allocated().unwrap();
///     let _buf = vec![0; 1024 * 1024];
///     jemalloc_ctl::epoch().unwrap();
///     let b = jemalloc_ctl::stats::allocated().unwrap();
///     assert!(a < b);
/// }
/// ```
///
/// [`epoch`]: ../fn.epoch().html
pub fn allocated() -> Result<usize> {
    ALLOCATED.name().read()
}

/// A type providing access to the total number of bytes allocated by the application.
///
/// This statistic is cached, and is only refreshed when the epoch is advanced. See the [`Epoch`]
/// type for more information.
///
/// This corresponds to `stats.allocated` in jemalloc's API.
///
/// # Examples
///
/// ```rust
/// extern crate jemallocator;
/// extern crate jemalloc_ctl;
///
/// use jemalloc_ctl::Epoch;
/// use jemalloc_ctl::stats::Allocated;
///
/// #[global_allocator]
/// static ALLOC: jemallocator::Jemalloc = jemallocator::Jemalloc;
///
/// fn main() {
///     let epoch = Epoch::new().unwrap();
///     let allocated = Allocated::new().unwrap();
///
///     let a = allocated.get().unwrap();
///     let _buf = vec![0; 1024 * 1024];
///     epoch.advance().unwrap();
///     let b = allocated.get().unwrap();
///     assert!(a < b);
/// }
/// ```
///
/// [`Epoch`]: ../struct.Epoch.html
#[derive(Copy, Clone)]
pub struct Allocated(Mib<[usize; 2]>);

impl Allocated {
    /// Returns a new `Allocated`.
    pub fn new() -> Result<Self> {
        let mib = ALLOCATED.name().mib()?;
        Ok(Allocated(mib))
    }

    /// Returns the total number of bytes allocated by the application.
    pub fn get(self) -> Result<usize> {
        self.0.read()
    }
}

const ACTIVE: &[u8] = b"stats.active\0";

/// Returns the total number of bytes in active pages allocated by the application.
///
/// This is a multiple of the page size, and is greater than or equal to the value returned by
/// [`allocated`].
///
/// This statistic is cached, and is only refreshed when the epoch is advanced. See the [`epoch`]
/// type for more information.
///
/// This corresponds to `stats.active` in jemalloc's API.
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
///     let a = jemalloc_ctl::stats::active().unwrap();
///     let _buf = vec![0; 1024 * 1024];
///     jemalloc_ctl::epoch().unwrap();
///     let b = jemalloc_ctl::stats::active().unwrap();
///     assert!(a < b);
/// }
/// ```
///
/// [`epoch`]: ../fn.epoch().html
/// [`allocated`]: fn.allocated.hml
pub fn active() -> Result<usize> {
    ACTIVE.name().read()
}

/// A type providing access to the total number of bytes in active pages allocated by the
/// application.
///
/// This is a multiple of the page size, and greater than or equal to the value returned by
/// [`Allocated`].
///
/// This statistic is cached, and is only refreshed when the epoch is advanced. See the [`Epoch`]
/// type for more information.
///
/// This corresponds to `stats.active` in jemalloc's API.
///
/// # Examples
///
/// ```rust
/// extern crate jemallocator;
/// extern crate jemalloc_ctl;
///
/// use jemalloc_ctl::Epoch;
/// use jemalloc_ctl::stats::Active;
///
/// #[global_allocator]
/// static ALLOC: jemallocator::Jemalloc = jemallocator::Jemalloc;
///
/// fn main() {
///     let epoch = Epoch::new().unwrap();
///     let active = Active::new().unwrap();
///
///     let a = active.get().unwrap();
///     let _buf = vec![0; 1024 * 1024];
///     epoch.advance().unwrap();
///     let b = active.get().unwrap();
///     assert!(a < b);
/// }
/// ```
///
/// [`Epoch`]: ../struct.Epoch.html
/// [`Allocated`]: struct.Allocated.html
#[derive(Copy, Clone)]
pub struct Active(Mib<[usize; 2]>);

impl Active {
    /// Returns a new `Allocated`.
    pub fn new() -> Result<Self> {
        let mib = ACTIVE.name().mib()?;
        Ok(Active(mib))
    }

    /// Returns the total number of bytes in active pages allocated by the application.
    pub fn get(self) -> Result<usize> {
        self.0.read()
    }
}

const METADATA: &[u8] = b"stats.metadata\0";

/// Returns the total number of bytes dedicated to jemalloc metadata.
///
/// This statistic is cached, and is only refreshed when the epoch is advanced. See the [`epoch`]
/// function for more information.
///
/// This corresponds to `stats.metadata` in jemalloc's API.
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
///     jemalloc_ctl::epoch().unwrap();
///     println!("{} bytes of jemalloc metadata", jemalloc_ctl::stats::metadata().unwrap());
/// }
/// ```
///
/// [`epoch`]: ../fn.epoch.html
pub fn metadata() -> Result<usize> {
    METADATA.name().read()
}

/// A type providing access to the total number of bytes dedicated to jemalloc metadata.
///
/// This statistic is cached, and is only refreshed when the epoch is advanced. See the [`Epoch`]
/// type for more information.
///
/// This corresponds to `stats.metadata` in jemalloc's API.
///
/// # Examples
///
/// ```rust
/// extern crate jemallocator;
/// extern crate jemalloc_ctl;
///
/// use jemalloc_ctl::Epoch;
/// use jemalloc_ctl::stats::Metadata;
///
/// #[global_allocator]
/// static ALLOC: jemallocator::Jemalloc = jemallocator::Jemalloc;
///
/// fn main() {
///     let epoch = Epoch::new().unwrap();
///     let metadata = Metadata::new().unwrap();
///
///     epoch.advance().unwrap();
///     let size = metadata.get().unwrap();
///     println!("{} bytes of jemalloc metadata", size);
/// }
/// ```
///
/// [`Epoch`]: ../struct.Epoch.html
#[derive(Copy, Clone)]
pub struct Metadata(Mib<[usize; 2]>);

impl Metadata {
    /// Returns a new `Metadata`.
    pub fn new() -> Result<Self> {
        let mib = METADATA.name().mib()?;
        Ok(Metadata(mib))
    }

    /// Returns the total number of bytes dedicated to jemalloc metadata.
    pub fn get(self) -> Result<usize> {
        self.0.read()
    }
}

const RESIDENT: &[u8] = b"stats.resident\0";

/// Returns the total number of bytes in physically resident data pages mapped by the allocator.
///
/// This consists of all pages dedicated to allocator metadata, pages backing active allocations,
/// and unused dirty pages. It may overestimate the true value because pages may not actually be
/// physically resident if they correspond to demand-zeroed virtual memory that has not yet been
/// touched. This is a multiple of the page size, and is larger than the value returned by
/// [`active`].
///
/// This statistic is cached, and is only refreshed when the epoch is advanced. See the [`epoch`]
/// function for more information.
///
/// This corresponds to `stats.resident` in jemalloc's API.
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
///     jemalloc_ctl::epoch().unwrap();
///     println!("{} bytes of total resident data", jemalloc_ctl::stats::resident().unwrap());
/// }
/// ```
///
/// [`epoch`]: ../fn.epoch.html
/// [`active`]: fn.active.html
pub fn resident() -> Result<usize> {
    RESIDENT.name().read()
}

/// A type providing access to the total number of bytes in physically resident data pages mapped
/// by the allocator.
///
/// This consists of all pages dedicated to allocator metadata, pages backing active allocations,
/// and unused dirty pages. It may overestimate the true value because pages may not actually be
/// physically resident if they correspond to demand-zeroed virtual memory that has not yet been
/// touched. This is a multiple of the page size, and is larger than the value returned by
/// [`Active`].
///
/// This statistic is cached, and is only refreshed when the epoch is advanced. See the [`Epoch`]
/// type for more information.
///
/// This corresponds to `stats.resident` in jemalloc's API.
///
/// # Examples
///
/// ```rust
/// extern crate jemallocator;
/// extern crate jemalloc_ctl;
///
/// use jemalloc_ctl::Epoch;
/// use jemalloc_ctl::stats::Resident;
///
/// #[global_allocator]
/// static ALLOC: jemallocator::Jemalloc = jemallocator::Jemalloc;
///
/// fn main() {
///     let epoch = Epoch::new().unwrap();
///     let resident = Resident::new().unwrap();
///
///     epoch.advance().unwrap();
///     let size = resident.get().unwrap();
///     println!("{} bytes of total resident data", size);
/// }
/// ```
///
/// [`Epoch`]: ../struct.Epoch.html
/// [`Active`]: struct.Active.html
#[derive(Copy, Clone)]
pub struct Resident(Mib<[usize; 2]>);

impl Resident {
    /// Returns a new `Resident`.
    pub fn new() -> Result<Self> {
        let mib = RESIDENT.name().mib()?;
        Ok(Resident(mib))
    }

    /// Returns the total number of bytes in physically resident data pages mapped by the allocator.
    pub fn get(self) -> Result<usize> {
        self.0.read()
    }
}

const MAPPED: &[u8] = b"stats.mapped\0";

/// Returns the total number of bytes in active extents mapped by the allocator.
///
/// This does not include inactive extents, even those taht contain unused dirty pages, so there
/// is no strict ordering between this and the value returned by [`resident`]. This is a
/// multiple of the page size, and is larger than the value returned by [`active`].
///
/// This statistic is cached, and is only refreshed when the epoch is advanced. See the [`epoch`]
/// type for more information.
///
/// This corresponds to `stats.mapped` in jemalloc's API.
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
///     jemalloc_ctl::epoch().unwrap();
///     println!("{} bytes of total mapped data", jemalloc_ctl::stats::mapped().unwrap());
/// }
/// ```
///
/// [`epoch`]: ../fn.epoch.html
/// [`resident`]: fn.resident.html
/// [`active`]: fn.active.html
pub fn mapped() -> Result<usize> {
    MAPPED.name().read()
}

/// A type providing access to the total number of bytes in active extents mapped by the allocator.
///
/// This does not include inactive extents, even those that contain unused dirty pages, so there
/// is no strict ordering between this and the value returned by [`Resident`]. This is a
/// multiple of the page size, and is larger than the value returned by [`Active`].
///
/// This statistic is cached, and is only refreshed when the epoch is advanced. See the [`Epoch`]
/// type for more information.
///
/// This corresponds to `stats.mapped` in jemalloc's API.
///
/// # Examples
///
/// ```rust
/// extern crate jemallocator;
/// extern crate jemalloc_ctl;
///
/// use jemalloc_ctl::Epoch;
/// use jemalloc_ctl::stats::Mapped;
///
/// #[global_allocator]
/// static ALLOC: jemallocator::Jemalloc = jemallocator::Jemalloc;
///
/// fn main() {
///     let epoch = Epoch::new().unwrap();
///     let mapped = Mapped::new().unwrap();
///
///     epoch.advance().unwrap();
///     let size = mapped.get().unwrap();
///     println!("{} bytes of total mapped data", size);
/// }
/// ```
///
/// [`Epoch`]: ../struct.Epoch.html
/// [`Resident`]: struct.Resident.html
/// [`Active`]: struct.Active.html
#[derive(Copy, Clone)]
pub struct Mapped(Mib<[usize; 2]>);

impl Mapped {
    /// Returns a new `Mapped`.
    pub fn new() -> Result<Self> {
        let mib = MAPPED.name().mib()?;
        Ok(Mapped(mib))
    }

    /// Returns the total number of bytes in active extents mapped by the allocator.
    pub fn get(self) -> Result<usize> {
        self.0.read()
    }
}

const RETAINED: &[u8] = b"stats.retained\0";

/// Returns the total number of bytes in virtual memory mappings that were retained rather than being returned to the
/// operating system via e.g. `munmap(2)`.
///
/// Retained virtual memory is typically untouched, decommitted, or purged, so it has no strongly associated physical
/// memory. Retained memory is excluded from mapped memory statistics, e.g. [`mapped`].
///
/// This statistic is cached, and is only refreshed when the epoch is advanced. See the [`epoch`]
/// type for more information.
///
/// This corresponds to `stats.retained` in jemalloc's API.
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
///     jemalloc_ctl::epoch().unwrap();
///     println!("{} bytes of total retained data", jemalloc_ctl::stats::retained().unwrap());
/// }
/// ```
///
/// [`epoch`]: ../fn.epoch.html
/// [`mapped`]: fn.mapped.html
pub fn retained() -> Result<usize> {
    RETAINED.name().read()
}

/// A type providing access to the total number of bytes in virtual memory mappings that were retained rather than being
/// returned to the operating system via e.g. `munmap(2)`.
///
/// Retained virtual memory is typically untouched, decommitted, or purged, so it has no strongly associated physical
/// memory. Retained memory is excluded from mapped memory statistics, e.g. [`Mapped`].
///
/// This statistic is cached, and is only refreshed when the epoch is advanced. See the [`Epoch`]
/// type for more information.
///
/// This corresponds to `stats.retained` in jemalloc's API.
///
/// # Examples
///
/// ```rust
/// extern crate jemallocator;
/// extern crate jemalloc_ctl;
///
/// use jemalloc_ctl::Epoch;
/// use jemalloc_ctl::stats::Retained;
///
/// #[global_allocator]
/// static ALLOC: jemallocator::Jemalloc = jemallocator::Jemalloc;
///
/// fn main() {
///     let epoch = Epoch::new().unwrap();
///     let retained = Retained::new().unwrap();
///
///     epoch.advance().unwrap();
///     let size = retained.get().unwrap();
///     println!("{} bytes of total retained data", size);
/// }
/// ```
///
/// [`Epoch`]: ../struct.Epoch.html
/// [`Mapped`]: struct.Mapped.html
#[derive(Copy, Clone)]
pub struct Retained(Mib<[usize; 2]>);

impl Retained {
    /// Returns a new `Retained`.
    pub fn new() -> Result<Self> {
        let mib = RETAINED.name().mib()?;
        Ok(Retained(mib))
    }

    /// Returns the total number of bytes in virtual memory mappings that were retained.
    pub fn get(self) -> Result<usize> {
        self.0.read()
    }
}
