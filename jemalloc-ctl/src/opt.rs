//! Information about the run-time `jemalloc` configuration.
//!
//! These settings are controlled by the `MALLOC_CONF` environment variable.
use error::Result;
use libc::c_uint;
use raw::{get, get_mib, get_str, get_str_mib, name_to_mib};

const ABORT: &[u8] = b"opt.abort\0";

/// Determines if `jemalloc` will call `abort(3)` on most warnings.
///
/// This is disabled by default unless `--enable-debug` was specified during build configuration.
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
///     println!("abort on warning: {}", jemalloc_ctl::opt::abort().unwrap());
/// }
/// ```
pub fn abort() -> Result<bool> {
    get(ABORT)
}

/// A type determining if `jemalloc` will call `abort(3)` on most warnings.
///
/// This is disabled by default unless `--enable-debug` was specified during build configuration.
///
/// # Examples
///
/// ```
/// extern crate jemallocator;
/// extern crate jemalloc_ctl;
///
/// use jemalloc_ctl::opt::Abort;
///
/// #[global_allocator]
/// static ALLOC: jemallocator::Jemalloc = jemallocator::Jemalloc;
///
/// fn main() {
///     let abort = Abort::new().unwrap();
///
///     println!("abort on warning: {}", abort.get().unwrap());
/// }
/// ```
#[derive(Copy, Clone)]
pub struct Abort([usize; 2]);

impl Abort {
    /// Returns a new `Abort`.
    pub fn new() -> Result<Self> {
        let mut mib = [0; 2];
        name_to_mib(ABORT, &mut mib)?;
        Ok(Abort(mib))
    }

    /// Returns the abort-on-warning behavior.
    pub fn get(self) -> Result<bool> {
        get_mib(&self.0)
    }
}

const DSS: &[u8] = b"opt.dss\0";

/// Returns the dss (`sbrk(2)`) allocation precedence as related to `mmap(2)` allocation.
///
/// The following settings are supported if `sbrk(2)` is supported by the operating system:
/// "disabled", "primary", and "secondary"; otherwise only "disabled" is supported. The default is
/// "secondary" if `sbrk(2)` is supported by the operating system; "disabled" otherwise.
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
///     println!("dss priority: {}", jemalloc_ctl::opt::dss().unwrap());
/// }
/// ```
pub fn dss() -> Result<&'static str> {
    get_str(DSS)
}

/// A type providing access to the dss (`sbrk(2)`) allocation precedence as related to `mmap(2)`
/// allocation.
///
/// The following settings are supported if `sbrk(2)` is supported by the operating system:
/// "disabled", "primary", and "secondary"; otherwise only "disabled" is supported. The default is
/// "secondary" if `sbrk(2)` is supported by the operating system; "disabled" otherwise.
///
/// # Examples
///
/// ```
/// extern crate jemallocator;
/// extern crate jemalloc_ctl;
///
/// use jemalloc_ctl::opt::Dss;
///
/// #[global_allocator]
/// static ALLOC: jemallocator::Jemalloc = jemallocator::Jemalloc;
///
/// fn main() {
///     let dss = Dss::new().unwrap();
///
///     println!("dss priority: {}", dss.get().unwrap());
/// }
/// ```
#[derive(Copy, Clone)]
pub struct Dss([usize; 2]);

impl Dss {
    /// Returns a new `Dss`.
    pub fn new() -> Result<Self> {
        let mut mib = [0; 2];
        name_to_mib(DSS, &mut mib)?;
        Ok(Dss(mib))
    }

    /// Returns the dss allocation precedence.
    pub fn get(self) -> Result<&'static str> {
        get_str_mib(&self.0)
    }
}

const NARENAS: &[u8] = b"opt.narenas\0";

/// Returns the maximum number of arenas to use for automatic multiplexing of threads and arenas.
///
/// The default is four times the number of CPUs, or one if there is a single CPU.
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
///     println!("number of arenas: {}", jemalloc_ctl::opt::narenas().unwrap());
/// }
/// ```
pub fn narenas() -> Result<c_uint> {
    get(NARENAS)
}

/// A type providing access to the maximum number of arenas to use for automatic multiplexing of
/// threads and arenas.
///
/// The default is four times the number of CPUs, or one if there is a single CPU.
///
/// # Examples
///
/// ```
/// extern crate jemallocator;
/// extern crate jemalloc_ctl;
///
/// use jemalloc_ctl::opt::NArenas;
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

const JUNK: &[u8] = b"opt.junk\0";

/// Returns `jemalloc`'s junk filling mode.
///
/// Requires `--enable-fill` to have been specified during build configuration.
///
/// If set to "alloc", each byte of uninitialized allocated memory will be set to `0x5a`. If set to
/// "free", each byte of deallocated memory will be set to `0x5a`. If set to "true", both allocated
/// and deallocated memory will be initialized, and if set to "false" junk filling will be disabled.
/// This is intended for debugging and will impact performance negatively.
///
/// The default is "false", unless `--enable-debug` was specified during build configuration, in
/// which case the default is "true".
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
///     println!("junk filling: {}", jemalloc_ctl::opt::junk().unwrap());
/// }
/// ```
pub fn junk() -> Result<&'static str> {
    get_str(JUNK)
}

/// A type providing access to `jemalloc`'s junk filling mode.
///
/// Requires `--enable-fill` to have been specified during build configuration.
///
/// If set to "alloc", each byte of uninitialized allocated memory will be set to `0x5a`. If set to
/// "free", each byte of deallocated memory will be set to `0x5a`. If set to "true", both allocated
/// and deallocated memory will be initialized, and if set to "false" junk filling will be disabled.
/// This is intended for debugging and will impact performance negatively.
///
/// The default is "false", unless `--enable-debug` was specified during build configuration, in
/// which case the default is "true".
///
/// # Examples
///
/// ```
/// extern crate jemallocator;
/// extern crate jemalloc_ctl;
///
/// use jemalloc_ctl::opt::Junk;
///
/// #[global_allocator]
/// static ALLOC: jemallocator::Jemalloc = jemallocator::Jemalloc;
///
/// fn main() {
///     let junk = Junk::new().unwrap();
///
///     println!("junk filling: {}", junk.get().unwrap());
/// }
/// ```
#[derive(Copy, Clone)]
pub struct Junk([usize; 2]);

impl Junk {
    /// Returns a new `Junk`.
    pub fn new() -> Result<Self> {
        let mut mib = [0; 2];
        name_to_mib(JUNK, &mut mib)?;
        Ok(Junk(mib))
    }

    /// Returns jemalloc's junk filling mode.
    pub fn get(self) -> Result<&'static str> {
        get_str_mib(&self.0)
    }
}

const ZERO: &[u8] = b"opt.zero\0";

/// Returns jemalloc's zeroing behavior.
///
/// Requires `--enable-fill` to have been specified during build configuration.
///
/// If enabled, `jemalloc` will initialize each byte of uninitialized allocated memory to 0. This is
/// intended for debugging and will impact performance negatively. It is disabled by default.
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
///     println!("zeroing: {}", jemalloc_ctl::opt::zero().unwrap());
/// }
/// ```
pub fn zero() -> Result<bool> {
    get(ZERO)
}

/// A type providing access to jemalloc's zeroing behavior.
///
/// Requires `--enable-fill` to have been specified during build configuration.
///
/// If enabled, `jemalloc` will initialize each byte of uninitialized allocated memory to 0. This is
/// intended for debugging and will impact performance negatively. It is disabled by default.
///
/// # Examples
///
/// ```
/// extern crate jemallocator;
/// extern crate jemalloc_ctl;
///
/// use jemalloc_ctl::opt::Zero;
///
/// #[global_allocator]
/// static ALLOC: jemallocator::Jemalloc = jemallocator::Jemalloc;
///
/// fn main() {
///     let zero = Zero::new().unwrap();
///
///     println!("zeroing: {}", zero.get().unwrap());
/// }
/// ```
#[derive(Copy, Clone)]
pub struct Zero([usize; 2]);

impl Zero {
    /// Returns a new `Zero`.
    pub fn new() -> Result<Self> {
        let mut mib = [0; 2];
        name_to_mib(ZERO, &mut mib)?;
        Ok(Zero(mib))
    }

    /// Returns the `jemalloc` zeroing behavior.
    pub fn get(self) -> Result<bool> {
        get_mib(&self.0)
    }
}

const TCACHE: &[u8] = b"opt.tcache\0";

/// Determines if thread-local allocation caching is enabled.
///
/// Thread-specific caching allows many allocations to be satisfied without performing any thread
/// synchronization, at the cost of increased memory use. This is enabled by default.
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
///     println!("thread-local caching: {}", jemalloc_ctl::opt::tcache().unwrap());
/// }
/// ```
pub fn tcache() -> Result<bool> {
    get(TCACHE)
}

/// A type providing access to thread-local allocation caching behavior.
///
/// Thread-specific caching allows many allocations to be satisfied without performing any thread
/// synchronization, at the cost of increased memory use. This is enabled by default.
///
/// # Examples
///
/// ```
/// extern crate jemallocator;
/// extern crate jemalloc_ctl;
///
/// use jemalloc_ctl::opt::Tcache;
///
/// #[global_allocator]
/// static ALLOC: jemallocator::Jemalloc = jemallocator::Jemalloc;
///
/// fn main() {
///     let tcache = Tcache::new().unwrap();
///
///     println!("thread-local caching: {}", tcache.get().unwrap());
/// }
/// ```
#[derive(Copy, Clone)]
pub struct Tcache([usize; 2]);

impl Tcache {
    /// Returns a new `Tcache`.
    pub fn new() -> Result<Self> {
        let mut mib = [0; 2];
        name_to_mib(TCACHE, &mut mib)?;
        Ok(Tcache(mib))
    }

    /// Returns the thread-local caching behavior.
    pub fn get(self) -> Result<bool> {
        get_mib(&self.0)
    }
}

const LG_TCACHE_MAX: &[u8] = b"opt.lg_tcache_max\0";

/// Returns the maximum size class (log base 2) to cache in the thread-specific cache (tcache).
///
/// At a minimum, all small size classes are cached, and at a maximum all large size classes are
/// cached. The default maximum is 32 KiB (2^15).
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
///     println!("max cached allocation size: {}", 1 << jemalloc_ctl::opt::lg_tcache_max().unwrap());
/// }
/// ```
pub fn lg_tcache_max() -> Result<usize> {
    get(LG_TCACHE_MAX)
}

/// A type providing access to the maximum size class (log base 2) to cache in the thread-specific
/// cache (tcache).
///
/// At a minimum, all small size classes are cached, and at a maximum all large size classes are
/// cached. The default maximum is 32 KiB (2^15).
///
/// # Examples
///
/// ```
/// extern crate jemallocator;
/// extern crate jemalloc_ctl;
///
/// use jemalloc_ctl::opt::LgTcacheMax;
///
/// #[global_allocator]
/// static ALLOC: jemallocator::Jemalloc = jemallocator::Jemalloc;
///
/// fn main() {
///     let lg_tcache_max = LgTcacheMax::new().unwrap();
///
///     println!("max cached allocation size: {}", 1 << lg_tcache_max.get().unwrap());
/// }
/// ```
#[derive(Copy, Clone)]
pub struct LgTcacheMax([usize; 2]);

impl LgTcacheMax {
    /// Returns a new `LgTcacheMax`.
    pub fn new() -> Result<Self> {
        let mut mib = [0; 2];
        name_to_mib(LG_TCACHE_MAX, &mut mib)?;
        Ok(LgTcacheMax(mib))
    }

    /// Returns the maximum cached size class.
    pub fn get(self) -> Result<usize> {
        get_mib(&self.0)
    }
}

const BACKGROUND_THREAD: &[u8] = b"opt.background_thread\0";

/// Returns whether `jemalloc` is initialized with background worker threads
/// enabled.
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
///         "initialized with background threads enabled: {}",
///         jemalloc_ctl::opt::background_thread().unwrap()
///     );
/// }
/// ```
pub fn background_thread() -> Result<bool> {
    get(BACKGROUND_THREAD)
}

/// A type determining if `jemalloc` will be initialized with background worker
/// threads enabled.
///
/// # Examples
///
/// ```
/// extern crate jemallocator;
/// extern crate jemalloc_ctl;
///
/// use jemalloc_ctl::opt::BackgroundThread;
///
/// #[global_allocator]
/// static ALLOC: jemallocator::Jemalloc = jemallocator::Jemalloc;
///
/// fn main() {
///     let background_thread = BackgroundThread::new().unwrap();
///
///     println!(
///        "initialized with background threads enabled: {}",
///        background_thread.get().unwrap()
///     );
/// }
/// ```
#[derive(Copy, Clone)]
pub struct BackgroundThread([usize; 2]);

impl BackgroundThread {
    /// Returns a new `BackgroundThread`.
    pub fn new() -> Result<Self> {
        let mut mib = [0; 2];
        name_to_mib(BACKGROUND_THREAD, &mut mib)?;
        Ok(BackgroundThread(mib))
    }

    /// Returns the background thread initialization behavior.
    pub fn get(self) -> Result<bool> {
        get_mib(&self.0)
    }
}
