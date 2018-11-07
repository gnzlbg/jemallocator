//! Thread specific operations.

use error::Result;
use raw::{get, get_mib, name_to_mib};

const ALLOCATEDP: &[u8] = b"thread.allocatedp\0";

/// Returns a thread-local pointer to the total number of bytes allocated by the current thread.
///
/// Unlike [`stats::allocated`], the value returned by this type is not the number of bytes
/// *currently* allocated, but rather the number of bytes that have *ever* been allocated by this
/// thread.
///
/// This function doesn't return the value directly, but actually a pointer to the value. This
/// allows for very fast repeated lookup, since there is no function call overhead. The pointer type
/// cannot be sent to other threads, but `allocated` can be called on different threads and will
/// return the appropriate pointer for each of them.
///
/// This corresponds to `thread.allocatedp` in jemalloc's API.
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
///     let allocated = jemalloc_ctl::thread::allocatedp().unwrap();
///
///     let a = allocated.get();
///     let buf = vec![0; 1024 * 1024];
///     let b = allocated.get();
///     drop(buf);
///     let c = allocated.get();
///
///     assert!(a < b);
///     assert_eq!(b, c);
/// }
/// ```
pub fn allocatedp() -> Result<ThreadLocal<u64>> {
    get(ALLOCATEDP).map(ThreadLocal)
}

/// A type providing access to the total number of bytes allocated by the current thread.
///
/// Unlike [`stats::Allocated`], the value returned by this type is not the number of bytes
/// *currently* allocated, but rather the number of bytes that have *ever* been allocated by this
/// thread.
///
/// The `get` method doesn't return the value directly, but actually a pointer to the value. This
/// allows for very fast repeated lookup, since there is no function call overhead. The pointer type
/// cannot be sent to other threads, but `Allocated::get` can be called on different threads and
/// will return the appropriate pointer for each of them.
///
/// # Example
///
/// ```
/// extern crate jemallocator;
/// extern crate jemalloc_ctl;
///
/// use jemalloc_ctl::thread::AllocatedP;
///
/// #[global_allocator]
/// static ALLOC: jemallocator::Jemalloc = jemallocator::Jemalloc;
///
/// fn main() {
///     let allocated = AllocatedP::new().unwrap();
///     let allocated = allocated.get().unwrap();
///
///     let a = allocated.get();
///     let buf = vec![0; 1024 * 1024];
///     let b = allocated.get();
///     drop(buf);
///     let c = allocated.get();
///
///     assert!(a < b);
///     assert_eq!(b, c);
/// }
/// ```
///
/// [`stats::Allocated`]: ../stats/struct.Allocated.html
#[derive(Copy, Clone)]
pub struct AllocatedP([usize; 2]);

impl AllocatedP {
    /// Returns a new `Allocated`.
    pub fn new() -> Result<Self> {
        let mut mib = [0; 2];
        name_to_mib(ALLOCATEDP, &mut mib)?;
        Ok(AllocatedP(mib))
    }

    /// Returns a thread-local pointer to the total number of bytes allocated by this thread.
    pub fn get(&self) -> Result<ThreadLocal<u64>> {
        get_mib(&self.0).map(ThreadLocal)
    }
}

const DEALLOCATEDP: &[u8] = b"thread.deallocatedp\0";

/// Returns a pointer to the total number of bytes deallocated by the current thread.
///
/// This function doesn't return the value directly, but actually a pointer to the value. This
/// allows for very fast repeated lookup, since there is no function call overhead. The pointer type
/// cannot be sent to other threads, but `deallocatedp` can be called on different threads and will
/// return the appropriate pointer for each of them.
///
/// This corresponds to `thread.deallocatedp` in jemalloc's API.
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
///     let deallocated = jemalloc_ctl::thread::deallocatedp().unwrap();
///
///     let a = deallocated.get();
///     let buf = vec![0; 1024 * 1024];
///     let b = deallocated.get();
///     drop(buf);
///     let c = deallocated.get();
///
///     assert_eq!(a, b);
///     assert!(b < c);
/// }
/// ```
pub fn deallocatedp() -> Result<ThreadLocal<u64>> {
    get(DEALLOCATEDP).map(ThreadLocal)
}

/// A type providing access to the total number of bytes deallocated by the current thread.
///
/// The `get` method doesn't return the value directly, but actually a pointer to the value. This
/// allows for very fast repeated lookup, since there is no function call overhead. The pointer type
/// cannot be sent to other threads, but `DeallocatedP::get` can be called on different threads and
/// will return the appropriate pointer for each of them.
///
/// # Example
///
/// ```
/// extern crate jemallocator;
/// extern crate jemalloc_ctl;
///
/// use jemalloc_ctl::thread::DeallocatedP;
///
/// #[global_allocator]
/// static ALLOC: jemallocator::Jemalloc = jemallocator::Jemalloc;
///
/// fn main() {
///     let deallocated = DeallocatedP::new().unwrap();
///     let deallocated = deallocated.get().unwrap();
///
///     let a = deallocated.get();
///     let buf = vec![0; 1024 * 1024];
///     let b = deallocated.get();
///     drop(buf);
///     let c = deallocated.get();
///
///     assert_eq!(a, b);
///     assert!(b < c);
/// }
/// ```
#[derive(Copy, Clone)]
pub struct DeallocatedP([usize; 2]);

impl DeallocatedP {
    /// Returns a new `Deallocated`.
    pub fn new() -> Result<Self> {
        let mut mib = [0; 2];
        name_to_mib(DEALLOCATEDP, &mut mib)?;
        Ok(DeallocatedP(mib))
    }

    /// Returns a thread-local pointer to the total number of bytes deallocated by this thread.
    pub fn get(&self) -> Result<ThreadLocal<u64>> {
        let ptr = get_mib::<*mut u64>(&self.0)?;
        Ok(ThreadLocal(ptr))
    }
}

/// A thread-local pointer.
///
/// It is neither `Sync` nor `Send`.
// NB we need *const here specifically since it's !Sync + !Send
#[derive(Copy, Clone)]
pub struct ThreadLocal<T>(*const T);

impl<T> ThreadLocal<T>
where
    T: Copy,
{
    /// Returns the current value at the pointer.
    #[inline]
    pub fn get(self) -> T {
        unsafe { *self.0 }
    }
}
