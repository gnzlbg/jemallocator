//! Information about the jemalloc compile-time configuration
use error::Result;
use raw::{get_str, get_str_mib, name_to_mib};

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
    get_str(MALLOC_CONF)
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
pub struct MallocConf([usize; 2]);

impl MallocConf {
    /// Returns a new `MallocConf`.
    pub fn new() -> Result<Self> {
        let mut mib = [0; 2];
        name_to_mib(MALLOC_CONF, &mut mib)?;
        Ok(MallocConf(mib))
    }

    /// Returns the embedded configure-time-specified run-time options config.
    pub fn get(self) -> Result<&'static str> {
        get_str_mib(&self.0)
    }
}
