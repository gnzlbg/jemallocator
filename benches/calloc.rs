//! This benchmarks compares the cost of `calloc(size, 1)` vs `mallocx(size, 0 |
//! MALLOCX_ZEROED)` for the case where `alignment <= MIN_ALIGN` (`flags == 0`)
//! by doing a round trip (`alloc(...) -> ptr; sdallocx(ptr)`).
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
    ($name:ident, $nbytes:expr) => {
        rt!($name, $nbytes, 0);
    };
    ($name:ident, $nbytes:expr, $flags:expr) => {
        #[bench]
        fn $name(b: &mut Bencher) {
            b.iter(|| unsafe {
                let ptr = jemalloc::mallocx($nbytes, $flags | jemalloc::MALLOCX_ZERO);
                test::black_box(ptr);
                jemalloc::sdallocx(ptr, $nbytes, $flags);
            });
        }
    }
}

macro_rules! rt_calloc {
    ($name:ident, $nbytes:expr) => {
        rt_calloc!($name, $nbytes, 0);
    };
    ($name:ident, $nbytes:expr, $flags:expr) => {
        #[bench]
        fn $name(b: &mut Bencher) {
            b.iter(|| unsafe {
                let ptr = jemalloc::calloc(1, $nbytes);
                test::black_box(ptr);
                jemalloc::sdallocx(ptr, $nbytes, $flags);
            });
        }
    }

}

// rt powers of 2
rt!(rt_pow2_1bytes_mallocx, 1);
rt!(rt_pow2_2bytes_mallocx, 2);
rt!(rt_pow2_4bytes_mallocx, 4);
rt!(rt_pow2_8bytes_mallocx, 8);
rt!(rt_pow2_16bytes_mallocx, 16);
rt!(rt_pow2_32bytes_mallocx, 32);
rt!(rt_pow2_64bytes_mallocx, 64);
rt!(rt_pow2_128bytes_mallocx, 128);
rt!(rt_pow2_256bytes_mallocx, 256);
rt!(rt_pow2_512bytes_mallocx, 512);
rt!(rt_pow2_1024bytes_mallocx, 1024);
rt!(rt_pow2_2048bytes_mallocx, 2048);
rt!(rt_pow2_4096bytes_mallocx, 4096);
rt!(rt_pow2_8192bytes_mallocx, 8192);
rt!(rt_pow2_16384bytes_mallocx, 16384);
rt!(rt_pow2_32768bytes_mallocx, 32768);
rt!(rt_pow2_65536bytes_mallocx, 65536);
rt!(rt_pow2_131072bytes_mallocx, 131072);
rt!(rt_pow2_4194304bytes_mallocx, 4194304);

// rt even
rt!(rt_even_10bytes_mallocx, 10);
rt!(rt_even_100bytes_mallocx, 100);
rt!(rt_even_1000bytes_mallocx, 1000);
rt!(rt_even_10000bytes_mallocx, 10000);
rt!(rt_even_100000bytes_mallocx, 100000);
rt!(rt_even_1000000bytes_mallocx, 1000000);

// rt odd
rt!(rt_odd_10bytes_mallocx, 9);
rt!(rt_odd_100bytes_mallocx, 99);
rt!(rt_odd_1000bytes_mallocx, 999);
rt!(rt_odd_10000bytes_mallocx, 9999);
rt!(rt_odd_100000bytes_mallocx, 99999);
rt!(rt_odd_1000000bytes_mallocx, 999999);

// primes
rt!(rt_prime_3bytes_mallocx, 3);
rt!(rt_prime_7bytes_mallocx, 7);
rt!(rt_prime_13bytes_mallocx, 13);
rt!(rt_prime_17bytes_mallocx, 17);
rt!(rt_prime_31bytes_mallocx, 31);
rt!(rt_prime_61bytes_mallocx, 61);
rt!(rt_prime_97bytes_mallocx, 97);
rt!(rt_prime_127bytes_mallocx, 127);
rt!(rt_prime_257bytes_mallocx, 257);
rt!(rt_prime_509bytes_mallocx, 509);
rt!(rt_prime_1021bytes_mallocx, 1021);
rt!(rt_prime_2039bytes_mallocx, 2039);
rt!(rt_prime_4093bytes_mallocx, 4093);
rt!(rt_prime_8191bytes_mallocx, 8191);
rt!(rt_prime_16381bytes_mallocx, 16381);
rt!(rt_prime_32749bytes_mallocx, 32749);
rt!(rt_prime_65537bytes_mallocx, 65537);
rt!(rt_prime_131071bytes_mallocx, 131071);
rt!(rt_prime_4194301bytes_mallocx, 4194301);

// rt_calloc powers of 2
rt_calloc!(rt_pow2_1bytes_calloc, 1);
rt_calloc!(rt_pow2_2bytes_calloc, 2);
rt_calloc!(rt_pow2_4bytes_calloc, 4);
rt_calloc!(rt_pow2_8bytes_calloc, 8);
rt_calloc!(rt_pow2_16bytes_calloc, 16);
rt_calloc!(rt_pow2_32bytes_calloc, 32);
rt_calloc!(rt_pow2_64bytes_calloc, 64);
rt_calloc!(rt_pow2_128bytes_calloc, 128);
rt_calloc!(rt_pow2_256bytes_calloc, 256);
rt_calloc!(rt_pow2_512bytes_calloc, 512);
rt_calloc!(rt_pow2_1024bytes_calloc, 1024);
rt_calloc!(rt_pow2_2048bytes_calloc, 2048);
rt_calloc!(rt_pow2_4096bytes_calloc, 4096);
rt_calloc!(rt_pow2_8192bytes_calloc, 8192);
rt_calloc!(rt_pow2_16384bytes_calloc, 16384);
rt_calloc!(rt_pow2_32768bytes_calloc, 32768);
rt_calloc!(rt_pow2_65536bytes_calloc, 65536);
rt_calloc!(rt_pow2_131072bytes_calloc, 131072);
rt_calloc!(rt_pow2_4194304bytes_calloc, 4194304);

// rt_calloc_even
rt_calloc!(rt_even_24bytes_calloc, 10);
rt_calloc!(rt_even_100bytes_calloc, 100);
rt_calloc!(rt_even_1000bytes_calloc, 1000);
rt_calloc!(rt_even_10000bytes_calloc, 10000);
rt_calloc!(rt_even_100000bytes_calloc, 100000);
rt_calloc!(rt_even_1000000bytes_calloc, 1000000);

// rt_calloc_odd
rt_calloc!(rt_odd_24bytes_calloc, 9);
rt_calloc!(rt_odd_100bytes_calloc, 99);
rt_calloc!(rt_odd_1000bytes_calloc, 999);
rt_calloc!(rt_odd_10000bytes_calloc, 9999);
rt_calloc!(rt_odd_100000bytes_calloc, 99999);
rt_calloc!(rt_odd_1000000bytes_calloc, 999999);

// primes
rt_calloc!(rt_prime_3bytes_calloc, 3);
rt_calloc!(rt_prime_7bytes_calloc, 7);
rt_calloc!(rt_prime_13bytes_calloc, 13);
rt_calloc!(rt_prime_17bytes_calloc, 17);
rt_calloc!(rt_prime_31bytes_calloc, 31);
rt_calloc!(rt_prime_61bytes_calloc, 61);
rt_calloc!(rt_prime_97bytes_calloc, 97);
rt_calloc!(rt_prime_127bytes_calloc, 127);
rt_calloc!(rt_prime_257bytes_calloc, 257);
rt_calloc!(rt_prime_509bytes_calloc, 509);
rt_calloc!(rt_prime_1021bytes_calloc, 1021);
rt_calloc!(rt_prime_2039bytes_calloc, 2039);
rt_calloc!(rt_prime_4093bytes_calloc, 4093);
rt_calloc!(rt_prime_8191bytes_calloc, 8191);
rt_calloc!(rt_prime_16381bytes_calloc, 16381);
rt_calloc!(rt_prime_32749bytes_calloc, 32749);
rt_calloc!(rt_prime_65537bytes_calloc, 65537);
rt_calloc!(rt_prime_131071bytes_calloc, 131071);
rt_calloc!(rt_prime_4194301bytes_calloc, 4194301);
