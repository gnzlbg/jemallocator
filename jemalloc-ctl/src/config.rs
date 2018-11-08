//! Information about the jemalloc compile-time configuration
use error::Result;
use keys::{Access, IntoName, MibStr};

const MALLOC_CONF: &[u8] = b"config.malloc_conf\0";

/// Returns the embeddec configure-time-specified run-time options config.
///
/// The string will be empty unless `--with-malloc-conf` was specified during
/// build configuration.
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
///         "default malloc conf: {}",
///         jemalloc_ctl::config::malloc_conf().unwrap()
///     );
/// }
/// ```
pub fn malloc_conf() -> Result<&'static str> {
    MALLOC_CONF.name().read()
}

/// A type providing access to the embedded configure-time-specified run-time
/// options config.
///
/// The string will be empty unless `--with-malloc-conf` was specified during
/// build configuration.
///
/// # Examples
///
/// ```
/// extern crate jemallocator;
/// extern crate jemalloc_ctl;
///
/// use jemalloc_ctl::config::MallocConf;
///
/// #[global_allocator]
/// static ALLOC: jemallocator::Jemalloc = jemallocator::Jemalloc;
///
/// fn main() {
///     let malloc_conf = MallocConf::new().unwrap();
///
///     println!("default malloc conf: {}", malloc_conf.get().unwrap());
/// }
/// ```
#[derive(Copy, Clone)]
pub struct MallocConf(MibStr<[usize; 2]>);

impl MallocConf {
    /// Returns a new `MallocConf`.
    pub fn new() -> Result<Self> {
        let mib = MALLOC_CONF.name().mib_str()?;
        Ok(MallocConf(mib))
    }

    /// Returns the embedded configure-time-specified run-time options config.
    pub fn get(self) -> Result<&'static str> {
        self.0.read()
    }
}
