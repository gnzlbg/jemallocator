//! Background thread operations.

use error::Result;
use raw::{get, get_mib, name_to_mib, set, set_mib};

const BACKGROUND_THREAD: &[u8] = b"background_thread\0";

/// Returns the state of internal background worker threads.
///
/// When enabled, background threads are created on demand (the number of
/// background threads will be no more than the number of CPUs or active
/// arenas). Threads run periodically and handle purging asynchronously.
///
/// ```
/// extern crate jemallocator;
/// extern crate jemalloc_ctl;
///
/// #[global_allocator]
/// static ALLOC: jemallocator::Jemalloc = jemallocator::Jemalloc;
///
/// fn main() {
/// #   #[cfg(not(target_os = "macos"))]
///     println!(
///         "background_thread: {}",
///         jemalloc_ctl::background_thread().unwrap()
///     );
/// }
/// ```
pub fn background_thread() -> Result<bool> {
    get(BACKGROUND_THREAD)
}

/// Enables or disables internal background worker threads.
///
/// When enabled, background threads are created on demand (the number of
/// background threads will be no more than the number of CPUs or active
/// arenas). Threads run periodically and handle purging asynchronously.
///
/// ```
/// extern crate jemallocator;
/// extern crate jemalloc_ctl;
///
/// #[global_allocator]
/// static ALLOC: jemallocator::Jemalloc = jemallocator::Jemalloc;
///
/// fn main() {
/// #   #[cfg(not(target_os = "macos"))] {
///     jemalloc_ctl::set_background_thread(true).unwrap();
///     assert!(jemalloc_ctl::background_thread().unwrap());
/// #   }
/// }
/// ```
pub fn set_background_thread(background_thread: bool) -> Result<()> {
    set(BACKGROUND_THREAD, background_thread)
}

/// A type providing access to the state of internal background worker threads.
///
/// When enabled, background threads are created on demand (the number of
/// background threads will be no more than the number of CPUs or active
/// arenas). Threads run periodically and handle purging asynchronously.
///
/// ```
/// extern crate jemallocator;
/// extern crate jemalloc_ctl;
///
/// #[global_allocator]
/// static ALLOC: jemallocator::Jemalloc = jemallocator::Jemalloc;
///
/// fn main() {
/// #   #[cfg(not(target_os = "macos"))] {
///     let mut background_thread
///         = jemalloc_ctl::BackgroundThread::new().unwrap();
///     background_thread.set(true).unwrap();
///     assert!(background_thread.get().unwrap());
/// #   }
/// }
/// ```
#[derive(Copy, Clone)]
pub struct BackgroundThread([usize; 1]);

impl BackgroundThread {
    /// Returns a new `BackgroundThread`.
    pub fn new() -> Result<Self> {
        let mut mib = [0; 1];
        name_to_mib(BACKGROUND_THREAD, &mut mib)?;
        Ok(BackgroundThread(mib))
    }

    /// Returns the current background thread state.
    pub fn get(self) -> Result<bool> {
        get_mib(&self.0)
    }

    /// Sets the background thread state.
    pub fn set(self, background_thread: bool) -> Result<()> {
        set_mib(&self.0, background_thread)
    }
}

const MAX_BACKGROUND_THREADS: &[u8] = b"max_background_threads\0";

/// Returns the maximum number of background threads that will be created.
///
/// ```
/// extern crate jemallocator;
/// extern crate jemalloc_ctl;
///
/// #[global_allocator]
/// static ALLOC: jemallocator::Jemalloc = jemallocator::Jemalloc;
///
/// fn main() {
/// #   #[cfg(not(target_os = "macos"))]
///     println!(
///         "max_background_threads: {}",
///         jemalloc_ctl::max_background_threads().unwrap()
///     );
/// }
/// ```
pub fn max_background_threads() -> Result<usize> {
    get(MAX_BACKGROUND_THREADS)
}

/// Sets the maximum number of background threads that will be created.
///
/// ```
/// extern crate jemallocator;
/// extern crate jemalloc_ctl;
///
/// #[global_allocator]
/// static ALLOC: jemallocator::Jemalloc = jemallocator::Jemalloc;
///
/// fn main() {
/// #   #[cfg(not(target_os = "macos"))] {
///     jemalloc_ctl::set_max_background_threads(1).unwrap();
///     assert_eq!(jemalloc_ctl::max_background_threads().unwrap(), 1);
/// # }
/// }
/// ```
pub fn set_max_background_threads(max_background_threads: usize) -> Result<()> {
    set(MAX_BACKGROUND_THREADS, max_background_threads)
}

/// A type providing access to the maximum number of background threads that
/// will be created.
///
/// ```
/// extern crate jemallocator;
/// extern crate jemalloc_ctl;
///
/// #[global_allocator]
/// static ALLOC: jemallocator::Jemalloc = jemallocator::Jemalloc;
///
/// fn main() {
/// #   #[cfg(not(target_os = "macos"))] {
///     let mut max_background_threads
///         = jemalloc_ctl::MaxBackgroundThreads::new().unwrap();
///     max_background_threads.set(1).unwrap();
///     assert_eq!(max_background_threads.get().unwrap(), 1);
/// # }
/// }
/// ```
#[derive(Copy, Clone)]
pub struct MaxBackgroundThreads([usize; 1]);

impl MaxBackgroundThreads {
    /// Returns a new `MaxBackgroundThreads`.
    pub fn new() -> Result<Self> {
        let mut mib = [0; 1];
        name_to_mib(MAX_BACKGROUND_THREADS, &mut mib)?;
        Ok(MaxBackgroundThreads(mib))
    }

    /// Returns the current background thread limit.
    pub fn get(self) -> Result<usize> {
        get_mib(&self.0)
    }

    /// Sets the background thread limit.
    pub fn set(self, max_background_threads: usize) -> Result<()> {
        set_mib(&self.0, max_background_threads)
    }
}
