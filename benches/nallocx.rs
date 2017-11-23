//! This benchmarks compares a round-trip (`mallocx(size, flags) -> ptr;
//! sdallocx(ptr)`) for `flags == 0` (`alignment <= MIN_ALIGN <= 16 bytes`) and
//! `flags == 5` (`alignment == 32 bytes`)with and without an intermediate call
//! to `nallocx` to asses the cost of a `nallocx` call. The difference between
//! the `_mallocx` and `_nallocx` timings and variance represents this costs.
#![feature(test, global_allocator)]

extern crate jemallocator;
extern crate test;
extern crate libc;

use jemallocator::ffi as jemalloc;

use test::Bencher;

use jemallocator::Jemalloc;

#[global_allocator]
static A: Jemalloc = Jemalloc;

macro_rules! rt {
    ($name:ident, $nbytes:expr, $flags:expr) => {
        #[bench]
        fn $name(b: &mut Bencher) {
            b.iter(|| unsafe {
                let ptr = jemalloc::mallocx($nbytes, $flags);
                jemalloc::sdallocx(ptr, $nbytes, $flags);
            });
        }
    }
}

macro_rules! rt_nallocx {
    ($name:ident, $nbytes:expr, $flags:expr) => {
        #[bench]
        fn $name(b: &mut Bencher) {
            b.iter(|| unsafe {
                let ptr = jemalloc::mallocx($nbytes, $flags);
                let rsz = jemalloc::nallocx($nbytes, $flags);
                jemalloc::sdallocx(ptr, rsz, $flags);
            });
        }
    }

}

// rt powers of 2
rt!(rt_pow2_1bytes_mallocx_f0, 1, 0);
rt!(rt_pow2_2bytes_mallocx_f0, 2, 0);
rt!(rt_pow2_4bytes_mallocx_f0, 4, 0);
rt!(rt_pow2_8bytes_mallocx_f0, 8, 0);
rt!(rt_pow2_16bytes_mallocx_f0, 16, 0);
rt!(rt_pow2_32bytes_mallocx_f0, 32, 0);
rt!(rt_pow2_64bytes_mallocx_f0, 64, 0);
rt!(rt_pow2_128bytes_mallocx_f0, 128, 0);
rt!(rt_pow2_256bytes_mallocx_f0, 256, 0);
rt!(rt_pow2_512bytes_mallocx_f0, 512, 0);
rt!(rt_pow2_1024bytes_mallocx_f0, 1024, 0);
rt!(rt_pow2_2048bytes_mallocx_f0, 2048, 0);
rt!(rt_pow2_4096bytes_mallocx_f0, 4096, 0);
rt!(rt_pow2_8192bytes_mallocx_f0, 8192, 0);
rt!(rt_pow2_16384bytes_mallocx_f0, 16384, 0);
rt!(rt_pow2_32768bytes_mallocx_f0, 32768, 0);
rt!(rt_pow2_65536bytes_mallocx_f0, 65536, 0);
rt!(rt_pow2_131072bytes_mallocx_f0, 131072, 0);
rt!(rt_pow2_4194304bytes_mallocx_f0, 4194304, 0);

// rt even
rt!(rt_even_10bytes_mallocx_f0, 10, 0);
rt!(rt_even_100bytes_mallocx_f0, 100, 0);
rt!(rt_even_1000bytes_mallocx_f0, 1000, 0);
rt!(rt_even_10000bytes_mallocx_f0, 10000, 0);
rt!(rt_even_100000bytes_mallocx_f0, 100000, 0);
rt!(rt_even_1000000bytes_mallocx_f0, 1000000, 0);

// rt odd
rt!(rt_odd_10bytes_mallocx_f0, 9, 0);
rt!(rt_odd_100bytes_mallocx_f0, 99, 0);
rt!(rt_odd_1000bytes_mallocx_f0, 999, 0);
rt!(rt_odd_10000bytes_mallocx_f0, 9999, 0);
rt!(rt_odd_100000bytes_mallocx_f0, 99999, 0);
rt!(rt_odd_1000000bytes_mallocx_f0, 999999, 0);

// primes
rt!(rt_prime_3bytes_mallocx_f0, 3, 0);
rt!(rt_prime_7bytes_mallocx_f0, 7, 0);
rt!(rt_prime_13bytes_mallocx_f0, 13, 0);
rt!(rt_prime_17bytes_mallocx_f0, 17, 0);
rt!(rt_prime_31bytes_mallocx_f0, 31, 0);
rt!(rt_prime_61bytes_mallocx_f0, 61, 0);
rt!(rt_prime_97bytes_mallocx_f0, 97, 0);
rt!(rt_prime_127bytes_mallocx_f0, 127, 0);
rt!(rt_prime_257bytes_mallocx_f0, 257, 0);
rt!(rt_prime_509bytes_mallocx_f0, 509, 0);
rt!(rt_prime_1021bytes_mallocx_f0, 1021, 0);
rt!(rt_prime_2039bytes_mallocx_f0, 2039, 0);
rt!(rt_prime_4093bytes_mallocx_f0, 4093, 0);
rt!(rt_prime_8191bytes_mallocx_f0, 8191, 0);
rt!(rt_prime_16381bytes_mallocx_f0, 16381, 0);
rt!(rt_prime_32749bytes_mallocx_f0, 32749, 0);
rt!(rt_prime_65537bytes_mallocx_f0, 65537, 0);
rt!(rt_prime_131071bytes_mallocx_f0, 131071, 0);
rt!(rt_prime_4194301bytes_mallocx_f0, 4194301, 0);

// rt_nallocx powers of 2
rt_nallocx!(rt_pow2_1bytes_nallocx_f0, 1, 0);
rt_nallocx!(rt_pow2_2bytes_nallocx_f0, 2, 0);
rt_nallocx!(rt_pow2_4bytes_nallocx_f0, 4, 0);
rt_nallocx!(rt_pow2_8bytes_nallocx_f0, 8, 0);
rt_nallocx!(rt_pow2_16bytes_nallocx_f0, 16, 0);
rt_nallocx!(rt_pow2_32bytes_nallocx_f0, 32, 0);
rt_nallocx!(rt_pow2_64bytes_nallocx_f0, 64, 0);
rt_nallocx!(rt_pow2_128bytes_nallocx_f0, 128, 0);
rt_nallocx!(rt_pow2_256bytes_nallocx_f0, 256, 0);
rt_nallocx!(rt_pow2_512bytes_nallocx_f0, 512, 0);
rt_nallocx!(rt_pow2_1024bytes_nallocx_f0, 1024, 0);
rt_nallocx!(rt_pow2_2048bytes_nallocx_f0, 2048, 0);
rt_nallocx!(rt_pow2_4096bytes_nallocx_f0, 4096, 0);
rt_nallocx!(rt_pow2_8192bytes_nallocx_f0, 8192, 0);
rt_nallocx!(rt_pow2_16384bytes_nallocx_f0, 16384, 0);
rt_nallocx!(rt_pow2_32768bytes_nallocx_f0, 32768, 0);
rt_nallocx!(rt_pow2_65536bytes_nallocx_f0, 65536, 0);
rt_nallocx!(rt_pow2_131072bytes_nallocx_f0, 131072, 0);
rt_nallocx!(rt_pow2_4194304bytes_nallocx_f0, 4194304, 0);

// rt_nallocx_even
rt_nallocx!(rt_even_24bytes_nallocx_f0, 10, 0);
rt_nallocx!(rt_even_100bytes_nallocx_f0, 100, 0);
rt_nallocx!(rt_even_1000bytes_nallocx_f0, 1000, 0);
rt_nallocx!(rt_even_10000bytes_nallocx_f0, 10000, 0);
rt_nallocx!(rt_even_100000bytes_nallocx_f0, 100000, 0);
rt_nallocx!(rt_even_1000000bytes_nallocx_f0, 1000000, 0);

// rt_nallocx_odd
rt_nallocx!(rt_odd_24bytes_nallocx_f0, 9, 0);
rt_nallocx!(rt_odd_100bytes_nallocx_f0, 99, 0);
rt_nallocx!(rt_odd_1000bytes_nallocx_f0, 999, 0);
rt_nallocx!(rt_odd_10000bytes_nallocx_f0, 9999, 0);
rt_nallocx!(rt_odd_100000bytes_nallocx_f0, 99999, 0);
rt_nallocx!(rt_odd_1000000bytes_nallocx_f0, 999999, 0);

// primes
rt_nallocx!(rt_prime_3bytes_nallocx_f0, 3, 0);
rt_nallocx!(rt_prime_7bytes_nallocx_f0, 7, 0);
rt_nallocx!(rt_prime_13bytes_nallocx_f0, 13, 0);
rt_nallocx!(rt_prime_17bytes_nallocx_f0, 17, 0);
rt_nallocx!(rt_prime_31bytes_nallocx_f0, 31, 0);
rt_nallocx!(rt_prime_61bytes_nallocx_f0, 61, 0);
rt_nallocx!(rt_prime_97bytes_nallocx_f0, 97, 0);
rt_nallocx!(rt_prime_127bytes_nallocx_f0, 127, 0);
rt_nallocx!(rt_prime_257bytes_nallocx_f0, 257, 0);
rt_nallocx!(rt_prime_509bytes_nallocx_f0, 509, 0);
rt_nallocx!(rt_prime_1021bytes_nallocx_f0, 1021, 0);
rt_nallocx!(rt_prime_2039bytes_nallocx_f0, 2039, 0);
rt_nallocx!(rt_prime_4093bytes_nallocx_f0, 4093, 0);
rt_nallocx!(rt_prime_8191bytes_nallocx_f0, 8191, 0);
rt_nallocx!(rt_prime_16381bytes_nallocx_f0, 16381, 0);
rt_nallocx!(rt_prime_32749bytes_nallocx_f0, 32749, 0);
rt_nallocx!(rt_prime_65537bytes_nallocx_f0, 65537, 0);
rt_nallocx!(rt_prime_131071bytes_nallocx_f0, 131071, 0);
rt_nallocx!(rt_prime_4194301bytes_nallocx_f0, 4194301, 0);

// with flags == 5 => 32 byte alignment:

// rt powers of 2
rt!(rt_pow2_1bytes_mallocx_f5, 1, 5);
rt!(rt_pow2_2bytes_mallocx_f5, 2, 5);
rt!(rt_pow2_4bytes_mallocx_f5, 4, 5);
rt!(rt_pow2_8bytes_mallocx_f5, 8, 5);
rt!(rt_pow2_16bytes_mallocx_f5, 16, 5);
rt!(rt_pow2_32bytes_mallocx_f5, 32, 5);
rt!(rt_pow2_64bytes_mallocx_f5, 64, 5);
rt!(rt_pow2_128bytes_mallocx_f5, 128, 5);
rt!(rt_pow2_256bytes_mallocx_f5, 256, 5);
rt!(rt_pow2_512bytes_mallocx_f5, 512, 5);
rt!(rt_pow2_1024bytes_mallocx_f5, 1024, 5);
rt!(rt_pow2_2048bytes_mallocx_f5, 2048, 5);
rt!(rt_pow2_4096bytes_mallocx_f5, 4096, 5);
rt!(rt_pow2_8192bytes_mallocx_f5, 8192, 5);
rt!(rt_pow2_16384bytes_mallocx_f5, 16384, 5);
rt!(rt_pow2_32768bytes_mallocx_f5, 32768, 5);
rt!(rt_pow2_65536bytes_mallocx_f5, 65536, 5);
rt!(rt_pow2_131072bytes_mallocx_f5, 131072, 5);
rt!(rt_pow2_4194304bytes_mallocx_f5, 4194304, 5);

// rt even
rt!(rt_even_10bytes_mallocx_f5, 10, 5);
rt!(rt_even_100bytes_mallocx_f5, 100, 5);
rt!(rt_even_1000bytes_mallocx_f5, 1000, 5);
rt!(rt_even_10000bytes_mallocx_f5, 10000, 5);
rt!(rt_even_100000bytes_mallocx_f5, 100000, 5);
rt!(rt_even_1000000bytes_mallocx_f5, 1000000, 5);

// rt odd
rt!(rt_odd_10bytes_mallocx_f5, 9, 5);
rt!(rt_odd_100bytes_mallocx_f5, 99, 5);
rt!(rt_odd_1000bytes_mallocx_f5, 999, 5);
rt!(rt_odd_10000bytes_mallocx_f5, 9999, 5);
rt!(rt_odd_100000bytes_mallocx_f5, 99999, 5);
rt!(rt_odd_1000000bytes_mallocx_f5, 999999, 5);

// primes
rt!(rt_prime_3bytes_mallocx_f5, 3, 5);
rt!(rt_prime_7bytes_mallocx_f5, 7, 5);
rt!(rt_prime_13bytes_mallocx_f5, 13, 5);
rt!(rt_prime_17bytes_mallocx_f5, 17, 5);
rt!(rt_prime_31bytes_mallocx_f5, 31, 5);
rt!(rt_prime_61bytes_mallocx_f5, 61, 5);
rt!(rt_prime_97bytes_mallocx_f5, 97, 5);
rt!(rt_prime_127bytes_mallocx_f5, 127, 5);
rt!(rt_prime_257bytes_mallocx_f5, 257, 5);
rt!(rt_prime_509bytes_mallocx_f5, 509, 5);
rt!(rt_prime_1021bytes_mallocx_f5, 1021, 5);
rt!(rt_prime_2039bytes_mallocx_f5, 2039, 5);
rt!(rt_prime_4093bytes_mallocx_f5, 4093, 5);
rt!(rt_prime_8191bytes_mallocx_f5, 8191, 5);
rt!(rt_prime_16381bytes_mallocx_f5, 16381, 5);
rt!(rt_prime_32749bytes_mallocx_f5, 32749, 5);
rt!(rt_prime_65537bytes_mallocx_f5, 65537, 5);
rt!(rt_prime_131071bytes_mallocx_f5, 131071, 5);
rt!(rt_prime_4194301bytes_mallocx_f5, 4194301, 5);

// rt_nallocx powers of 2
rt_nallocx!(rt_pow2_1bytes_nallocx_f5, 1, 5);
rt_nallocx!(rt_pow2_2bytes_nallocx_f5, 2, 5);
rt_nallocx!(rt_pow2_4bytes_nallocx_f5, 4, 5);
rt_nallocx!(rt_pow2_8bytes_nallocx_f5, 8, 5);
rt_nallocx!(rt_pow2_16bytes_nallocx_f5, 16, 5);
rt_nallocx!(rt_pow2_32bytes_nallocx_f5, 32, 5);
rt_nallocx!(rt_pow2_64bytes_nallocx_f5, 64, 5);
rt_nallocx!(rt_pow2_128bytes_nallocx_f5, 128, 5);
rt_nallocx!(rt_pow2_256bytes_nallocx_f5, 256, 5);
rt_nallocx!(rt_pow2_512bytes_nallocx_f5, 512, 5);
rt_nallocx!(rt_pow2_1024bytes_nallocx_f5, 1024, 5);
rt_nallocx!(rt_pow2_2048bytes_nallocx_f5, 2048, 5);
rt_nallocx!(rt_pow2_4096bytes_nallocx_f5, 4096, 5);
rt_nallocx!(rt_pow2_8192bytes_nallocx_f5, 8192, 5);
rt_nallocx!(rt_pow2_16384bytes_nallocx_f5, 16384, 5);
rt_nallocx!(rt_pow2_32768bytes_nallocx_f5, 32768, 5);
rt_nallocx!(rt_pow2_65536bytes_nallocx_f5, 65536, 5);
rt_nallocx!(rt_pow2_131072bytes_nallocx_f5, 131072, 5);
rt_nallocx!(rt_pow2_4194304bytes_nallocx_f5, 4194304, 5);

// rt_nallocx_even
rt_nallocx!(rt_even_24bytes_nallocx_f5, 10, 5);
rt_nallocx!(rt_even_100bytes_nallocx_f5, 100, 5);
rt_nallocx!(rt_even_1000bytes_nallocx_f5, 1000, 5);
rt_nallocx!(rt_even_10000bytes_nallocx_f5, 10000, 5);
rt_nallocx!(rt_even_100000bytes_nallocx_f5, 100000, 5);
rt_nallocx!(rt_even_1000000bytes_nallocx_f5, 1000000, 5);

// rt_nallocx_odd
rt_nallocx!(rt_odd_24bytes_nallocx_f5, 9, 5);
rt_nallocx!(rt_odd_100bytes_nallocx_f5, 99, 5);
rt_nallocx!(rt_odd_1000bytes_nallocx_f5, 999, 5);
rt_nallocx!(rt_odd_10000bytes_nallocx_f5, 9999, 5);
rt_nallocx!(rt_odd_100000bytes_nallocx_f5, 99999, 5);
rt_nallocx!(rt_odd_1000000bytes_nallocx_f5, 999999, 5);

// primes
rt_nallocx!(rt_prime_3bytes_nallocx_f5, 3, 5);
rt_nallocx!(rt_prime_7bytes_nallocx_f5, 7, 5);
rt_nallocx!(rt_prime_13bytes_nallocx_f5, 13, 5);
rt_nallocx!(rt_prime_17bytes_nallocx_f5, 17, 5);
rt_nallocx!(rt_prime_31bytes_nallocx_f5, 31, 5);
rt_nallocx!(rt_prime_61bytes_nallocx_f5, 61, 5);
rt_nallocx!(rt_prime_97bytes_nallocx_f5, 97, 5);
rt_nallocx!(rt_prime_127bytes_nallocx_f5, 127, 5);
rt_nallocx!(rt_prime_257bytes_nallocx_f5, 257, 5);
rt_nallocx!(rt_prime_509bytes_nallocx_f5, 509, 5);
rt_nallocx!(rt_prime_1021bytes_nallocx_f5, 1021, 5);
rt_nallocx!(rt_prime_2039bytes_nallocx_f5, 2039, 5);
rt_nallocx!(rt_prime_4093bytes_nallocx_f5, 4093, 5);
rt_nallocx!(rt_prime_8191bytes_nallocx_f5, 8191, 5);
rt_nallocx!(rt_prime_16381bytes_nallocx_f5, 16381, 5);
rt_nallocx!(rt_prime_32749bytes_nallocx_f5, 32749, 5);
rt_nallocx!(rt_prime_65537bytes_nallocx_f5, 65537, 5);
rt_nallocx!(rt_prime_131071bytes_nallocx_f5, 131071, 5);
rt_nallocx!(rt_prime_4194301bytes_nallocx_f5, 4194301, 5);
