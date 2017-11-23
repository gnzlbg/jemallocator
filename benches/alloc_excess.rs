//! This benchmarks compares the cost of `alloc_excess` over plain alloc when
//! the excess is used / unused. The difference for the case in which it is used
//! should be approxiamtely equal to the cost of a `nallocx` call. If the
//! version in which the result is unused also exhibits this cost, then
//! `nallocx` is not being optimized out.
#![feature(test, global_allocator, allocator_api)]

extern crate jemallocator;
extern crate test;
extern crate libc;

use std::heap::{Alloc, Layout, Excess};

use test::Bencher;

use jemallocator::Jemalloc;

#[global_allocator]
static A: Jemalloc = Jemalloc;

macro_rules! rt_alloc {
    ($name:ident, $nbytes:expr, $align:expr) => {
        #[bench]
        fn $name(b: &mut Bencher) {
            b.iter(|| unsafe {
                let layout = Layout::from_size_align($nbytes, $align).unwrap();
                let ptr = Jemalloc.alloc(layout.clone()).unwrap();
                test::black_box(ptr);
                Jemalloc.dealloc(ptr, layout);
            });
        }
    }
}

macro_rules! rt_alloc_excess_unused {
    ($name:ident, $nbytes:expr, $align:expr) => {
        #[bench]
        fn $name(b: &mut Bencher) {
            b.iter(|| unsafe {
                let layout = Layout::from_size_align($nbytes, $align).unwrap();
                let Excess(ptr, _) = Jemalloc.alloc_excess(layout.clone()).unwrap();
                test::black_box(ptr);
                Jemalloc.dealloc(ptr, layout); 
            });
        }
    }
}

macro_rules! rt_alloc_excess_used {
    ($name:ident, $nbytes:expr, $align:expr) => {
        #[bench]
        fn $name(b: &mut Bencher) {
            b.iter(|| unsafe {
                let layout = Layout::from_size_align($nbytes, $align).unwrap();
                let Excess(ptr, excess) = Jemalloc.alloc_excess(layout.clone()).unwrap();
                test::black_box(ptr);
                test::black_box(excess);
                Jemalloc.dealloc(ptr, layout); 
            });
        }
    }
}


// 1byte alignment

// rt powers of 2
rt_alloc!(rt_pow2_1bytes_alloc_a1, 1, 1);
rt_alloc!(rt_pow2_2bytes_alloc_a1, 2, 1);
rt_alloc!(rt_pow2_4bytes_alloc_a1, 4, 1);
rt_alloc!(rt_pow2_8bytes_alloc_a1, 8, 1);
rt_alloc!(rt_pow2_16bytes_alloc_a1, 16, 1);
rt_alloc!(rt_pow2_32bytes_alloc_a1, 32, 1);
rt_alloc!(rt_pow2_64bytes_alloc_a1, 64, 1);
rt_alloc!(rt_pow2_128bytes_alloc_a1, 128, 1);
rt_alloc!(rt_pow2_256bytes_alloc_a1, 256, 1);
rt_alloc!(rt_pow2_512bytes_alloc_a1, 512, 1);
rt_alloc!(rt_pow2_1024bytes_alloc_a1, 1024, 1);
rt_alloc!(rt_pow2_2048bytes_alloc_a1, 2048, 1);
rt_alloc!(rt_pow2_4096bytes_alloc_a1, 4096, 1);
rt_alloc!(rt_pow2_8192bytes_alloc_a1, 8192, 1);
rt_alloc!(rt_pow2_16384bytes_alloc_a1, 16384, 1);
rt_alloc!(rt_pow2_32768bytes_alloc_a1, 32768, 1);
rt_alloc!(rt_pow2_65536bytes_alloc_a1, 65536, 1);
rt_alloc!(rt_pow2_131072bytes_alloc_a1, 131072, 1);
rt_alloc!(rt_pow2_4194304bytes_alloc_a1, 4194304, 1);

// rt even
rt_alloc!(rt_even_10bytes_alloc_a1, 10, 1);
rt_alloc!(rt_even_100bytes_alloc_a1, 100, 1);
rt_alloc!(rt_even_1000bytes_alloc_a1, 1000, 1);
rt_alloc!(rt_even_10000bytes_alloc_a1, 10000, 1);
rt_alloc!(rt_even_100000bytes_alloc_a1, 100000, 1);
rt_alloc!(rt_even_1000000bytes_alloc_a1, 1000000, 1);

// rt odd
rt_alloc!(rt_odd_10bytes_alloc_a1, 9, 1);
rt_alloc!(rt_odd_100bytes_alloc_a1, 99, 1);
rt_alloc!(rt_odd_1000bytes_alloc_a1, 999, 1);
rt_alloc!(rt_odd_10000bytes_alloc_a1, 9999, 1);
rt_alloc!(rt_odd_100000bytes_alloc_a1, 99999, 1);
rt_alloc!(rt_odd_1000000bytes_alloc_a1, 999999, 1);

// primes
rt_alloc!(rt_prime_3bytes_alloc_a1, 3, 1);
rt_alloc!(rt_prime_7bytes_alloc_a1, 7, 1);
rt_alloc!(rt_prime_13bytes_alloc_a1, 13, 1);
rt_alloc!(rt_prime_17bytes_alloc_a1, 17, 1);
rt_alloc!(rt_prime_31bytes_alloc_a1, 31, 1);
rt_alloc!(rt_prime_61bytes_alloc_a1, 61, 1);
rt_alloc!(rt_prime_97bytes_alloc_a1, 97, 1);
rt_alloc!(rt_prime_127bytes_alloc_a1, 127, 1);
rt_alloc!(rt_prime_257bytes_alloc_a1, 257, 1);
rt_alloc!(rt_prime_509bytes_alloc_a1, 509, 1);
rt_alloc!(rt_prime_1021bytes_alloc_a1, 1021, 1);
rt_alloc!(rt_prime_2039bytes_alloc_a1, 2039, 1);
rt_alloc!(rt_prime_4093bytes_alloc_a1, 4093, 1);
rt_alloc!(rt_prime_8191bytes_alloc_a1, 8191, 1);
rt_alloc!(rt_prime_16381bytes_alloc_a1, 16381, 1);
rt_alloc!(rt_prime_32749bytes_alloc_a1, 32749, 1);
rt_alloc!(rt_prime_65537bytes_alloc_a1, 65537, 1);
rt_alloc!(rt_prime_131071bytes_alloc_a1, 131071, 1);
rt_alloc!(rt_prime_4194301bytes_alloc_a1, 4194301, 1);


// rt powers of 2
rt_alloc_excess_unused!(rt_pow2_1bytes_alloc_excess_unused_a1, 1, 1);
rt_alloc_excess_unused!(rt_pow2_2bytes_alloc_excess_unused_a1, 2, 1);
rt_alloc_excess_unused!(rt_pow2_4bytes_alloc_excess_unused_a1, 4, 1);
rt_alloc_excess_unused!(rt_pow2_8bytes_alloc_excess_unused_a1, 8, 1);
rt_alloc_excess_unused!(rt_pow2_16bytes_alloc_excess_unused_a1, 16, 1);
rt_alloc_excess_unused!(rt_pow2_32bytes_alloc_excess_unused_a1, 32, 1);
rt_alloc_excess_unused!(rt_pow2_64bytes_alloc_excess_unused_a1, 64, 1);
rt_alloc_excess_unused!(rt_pow2_128bytes_alloc_excess_unused_a1, 128, 1);
rt_alloc_excess_unused!(rt_pow2_256bytes_alloc_excess_unused_a1, 256, 1);
rt_alloc_excess_unused!(rt_pow2_512bytes_alloc_excess_unused_a1, 512, 1);
rt_alloc_excess_unused!(rt_pow2_1024bytes_alloc_excess_unused_a1, 1024, 1);
rt_alloc_excess_unused!(rt_pow2_2048bytes_alloc_excess_unused_a1, 2048, 1);
rt_alloc_excess_unused!(rt_pow2_4096bytes_alloc_excess_unused_a1, 4096, 1);
rt_alloc_excess_unused!(rt_pow2_8192bytes_alloc_excess_unused_a1, 8192, 1);
rt_alloc_excess_unused!(rt_pow2_16384bytes_alloc_excess_unused_a1, 16384, 1);
rt_alloc_excess_unused!(rt_pow2_32768bytes_alloc_excess_unused_a1, 32768, 1);
rt_alloc_excess_unused!(rt_pow2_65536bytes_alloc_excess_unused_a1, 65536, 1);
rt_alloc_excess_unused!(rt_pow2_131072bytes_alloc_excess_unused_a1, 131072, 1);
rt_alloc_excess_unused!(rt_pow2_4194304bytes_alloc_excess_unused_a1, 4194304, 1);

// rt even
rt_alloc_excess_unused!(rt_even_10bytes_alloc_excess_unused_a1, 10, 1);
rt_alloc_excess_unused!(rt_even_100bytes_alloc_excess_unused_a1, 100, 1);
rt_alloc_excess_unused!(rt_even_1000bytes_alloc_excess_unused_a1, 1000, 1);
rt_alloc_excess_unused!(rt_even_10000bytes_alloc_excess_unused_a1, 10000, 1);
rt_alloc_excess_unused!(rt_even_100000bytes_alloc_excess_unused_a1, 100000, 1);
rt_alloc_excess_unused!(rt_even_1000000bytes_alloc_excess_unused_a1, 1000000, 1);

// rt odd
rt_alloc_excess_unused!(rt_odd_10bytes_alloc_excess_unused_a1, 9, 1);
rt_alloc_excess_unused!(rt_odd_100bytes_alloc_excess_unused_a1, 99, 1);
rt_alloc_excess_unused!(rt_odd_1000bytes_alloc_excess_unused_a1, 999, 1);
rt_alloc_excess_unused!(rt_odd_10000bytes_alloc_excess_unused_a1, 9999, 1);
rt_alloc_excess_unused!(rt_odd_100000bytes_alloc_excess_unused_a1, 99999, 1);
rt_alloc_excess_unused!(rt_odd_1000000bytes_alloc_excess_unused_a1, 999999, 1);

// primes
rt_alloc_excess_unused!(rt_prime_3bytes_alloc_excess_unused_a1, 3, 1);
rt_alloc_excess_unused!(rt_prime_7bytes_alloc_excess_unused_a1, 7, 1);
rt_alloc_excess_unused!(rt_prime_13bytes_alloc_excess_unused_a1, 13, 1);
rt_alloc_excess_unused!(rt_prime_17bytes_alloc_excess_unused_a1, 17, 1);
rt_alloc_excess_unused!(rt_prime_31bytes_alloc_excess_unused_a1, 31, 1);
rt_alloc_excess_unused!(rt_prime_61bytes_alloc_excess_unused_a1, 61, 1);
rt_alloc_excess_unused!(rt_prime_97bytes_alloc_excess_unused_a1, 97, 1);
rt_alloc_excess_unused!(rt_prime_127bytes_alloc_excess_unused_a1, 127, 1);
rt_alloc_excess_unused!(rt_prime_257bytes_alloc_excess_unused_a1, 257, 1);
rt_alloc_excess_unused!(rt_prime_509bytes_alloc_excess_unused_a1, 509, 1);
rt_alloc_excess_unused!(rt_prime_1021bytes_alloc_excess_unused_a1, 1021, 1);
rt_alloc_excess_unused!(rt_prime_2039bytes_alloc_excess_unused_a1, 2039, 1);
rt_alloc_excess_unused!(rt_prime_4093bytes_alloc_excess_unused_a1, 4093, 1);
rt_alloc_excess_unused!(rt_prime_8191bytes_alloc_excess_unused_a1, 8191, 1);
rt_alloc_excess_unused!(rt_prime_16381bytes_alloc_excess_unused_a1, 16381, 1);
rt_alloc_excess_unused!(rt_prime_32749bytes_alloc_excess_unused_a1, 32749, 1);
rt_alloc_excess_unused!(rt_prime_65537bytes_alloc_excess_unused_a1, 65537, 1);
rt_alloc_excess_unused!(rt_prime_131071bytes_alloc_excess_unused_a1, 131071, 1);
rt_alloc_excess_unused!(rt_prime_4194301bytes_alloc_excess_unused_a1, 4194301, 1);


// rt powers of 2
rt_alloc_excess_used!(rt_pow2_1bytes_alloc_excess_used_a1, 1, 1);
rt_alloc_excess_used!(rt_pow2_2bytes_alloc_excess_used_a1, 2, 1);
rt_alloc_excess_used!(rt_pow2_4bytes_alloc_excess_used_a1, 4, 1);
rt_alloc_excess_used!(rt_pow2_8bytes_alloc_excess_used_a1, 8, 1);
rt_alloc_excess_used!(rt_pow2_16bytes_alloc_excess_used_a1, 16, 1);
rt_alloc_excess_used!(rt_pow2_32bytes_alloc_excess_used_a1, 32, 1);
rt_alloc_excess_used!(rt_pow2_64bytes_alloc_excess_used_a1, 64, 1);
rt_alloc_excess_used!(rt_pow2_128bytes_alloc_excess_used_a1, 128, 1);
rt_alloc_excess_used!(rt_pow2_256bytes_alloc_excess_used_a1, 256, 1);
rt_alloc_excess_used!(rt_pow2_512bytes_alloc_excess_used_a1, 512, 1);
rt_alloc_excess_used!(rt_pow2_1024bytes_alloc_excess_used_a1, 1024, 1);
rt_alloc_excess_used!(rt_pow2_2048bytes_alloc_excess_used_a1, 2048, 1);
rt_alloc_excess_used!(rt_pow2_4096bytes_alloc_excess_used_a1, 4096, 1);
rt_alloc_excess_used!(rt_pow2_8192bytes_alloc_excess_used_a1, 8192, 1);
rt_alloc_excess_used!(rt_pow2_16384bytes_alloc_excess_used_a1, 16384, 1);
rt_alloc_excess_used!(rt_pow2_32768bytes_alloc_excess_used_a1, 32768, 1);
rt_alloc_excess_used!(rt_pow2_65536bytes_alloc_excess_used_a1, 65536, 1);
rt_alloc_excess_used!(rt_pow2_131072bytes_alloc_excess_used_a1, 131072, 1);
rt_alloc_excess_used!(rt_pow2_4194304bytes_alloc_excess_used_a1, 4194304, 1);

// rt even
rt_alloc_excess_used!(rt_even_10bytes_alloc_excess_used_a1, 10, 1);
rt_alloc_excess_used!(rt_even_100bytes_alloc_excess_used_a1, 100, 1);
rt_alloc_excess_used!(rt_even_1000bytes_alloc_excess_used_a1, 1000, 1);
rt_alloc_excess_used!(rt_even_10000bytes_alloc_excess_used_a1, 10000, 1);
rt_alloc_excess_used!(rt_even_100000bytes_alloc_excess_used_a1, 100000, 1);
rt_alloc_excess_used!(rt_even_1000000bytes_alloc_excess_used_a1, 1000000, 1);

// rt odd
rt_alloc_excess_used!(rt_odd_10bytes_alloc_excess_used_a1, 9, 1);
rt_alloc_excess_used!(rt_odd_100bytes_alloc_excess_used_a1, 99, 1);
rt_alloc_excess_used!(rt_odd_1000bytes_alloc_excess_used_a1, 999, 1);
rt_alloc_excess_used!(rt_odd_10000bytes_alloc_excess_used_a1, 9999, 1);
rt_alloc_excess_used!(rt_odd_100000bytes_alloc_excess_used_a1, 99999, 1);
rt_alloc_excess_used!(rt_odd_1000000bytes_alloc_excess_used_a1, 999999, 1);

// primes
rt_alloc_excess_used!(rt_prime_3bytes_alloc_excess_used_a1, 3, 1);
rt_alloc_excess_used!(rt_prime_7bytes_alloc_excess_used_a1, 7, 1);
rt_alloc_excess_used!(rt_prime_13bytes_alloc_excess_used_a1, 13, 1);
rt_alloc_excess_used!(rt_prime_17bytes_alloc_excess_used_a1, 17, 1);
rt_alloc_excess_used!(rt_prime_31bytes_alloc_excess_used_a1, 31, 1);
rt_alloc_excess_used!(rt_prime_61bytes_alloc_excess_used_a1, 61, 1);
rt_alloc_excess_used!(rt_prime_97bytes_alloc_excess_used_a1, 97, 1);
rt_alloc_excess_used!(rt_prime_127bytes_alloc_excess_used_a1, 127, 1);
rt_alloc_excess_used!(rt_prime_257bytes_alloc_excess_used_a1, 257, 1);
rt_alloc_excess_used!(rt_prime_509bytes_alloc_excess_used_a1, 509, 1);
rt_alloc_excess_used!(rt_prime_1021bytes_alloc_excess_used_a1, 1021, 1);
rt_alloc_excess_used!(rt_prime_2039bytes_alloc_excess_used_a1, 2039, 1);
rt_alloc_excess_used!(rt_prime_4093bytes_alloc_excess_used_a1, 4093, 1);
rt_alloc_excess_used!(rt_prime_8191bytes_alloc_excess_used_a1, 8191, 1);
rt_alloc_excess_used!(rt_prime_16381bytes_alloc_excess_used_a1, 16381, 1);
rt_alloc_excess_used!(rt_prime_32749bytes_alloc_excess_used_a1, 32749, 1);
rt_alloc_excess_used!(rt_prime_65537bytes_alloc_excess_used_a1, 65537, 1);
rt_alloc_excess_used!(rt_prime_131071bytes_alloc_excess_used_a1, 131071, 1);
rt_alloc_excess_used!(rt_prime_4194301bytes_alloc_excess_used_a1, 4194301, 1);


// alignemnt 8 bytes:

rt_alloc!(rt_pow2_1bytes_alloc_a8, 1, 8);
rt_alloc!(rt_pow2_2bytes_alloc_a8, 2, 8);
rt_alloc!(rt_pow2_4bytes_alloc_a8, 4, 8);
rt_alloc!(rt_pow2_8bytes_alloc_a8, 8, 8);
rt_alloc!(rt_pow2_16bytes_alloc_a8, 16, 8);
rt_alloc!(rt_pow2_32bytes_alloc_a8, 32, 8);
rt_alloc!(rt_pow2_64bytes_alloc_a8, 64, 8);
rt_alloc!(rt_pow2_128bytes_alloc_a8, 128, 8);
rt_alloc!(rt_pow2_256bytes_alloc_a8, 256, 8);
rt_alloc!(rt_pow2_512bytes_alloc_a8, 512, 8);
rt_alloc!(rt_pow2_1024bytes_alloc_a8, 1024, 8);
rt_alloc!(rt_pow2_2048bytes_alloc_a8, 2048, 8);
rt_alloc!(rt_pow2_4096bytes_alloc_a8, 4096, 8);
rt_alloc!(rt_pow2_8192bytes_alloc_a8, 8192, 8);
rt_alloc!(rt_pow2_16384bytes_alloc_a8, 16384, 8);
rt_alloc!(rt_pow2_32768bytes_alloc_a8, 32768, 8);
rt_alloc!(rt_pow2_65536bytes_alloc_a8, 65536, 8);
rt_alloc!(rt_pow2_131072bytes_alloc_a8, 131072, 8);
rt_alloc!(rt_pow2_4194304bytes_alloc_a8, 4194304, 8);

// rt even
rt_alloc!(rt_even_10bytes_alloc_a8, 10, 8);
rt_alloc!(rt_even_100bytes_alloc_a8, 100, 8);
rt_alloc!(rt_even_1000bytes_alloc_a8, 1000, 8);
rt_alloc!(rt_even_10000bytes_alloc_a8, 10000, 8);
rt_alloc!(rt_even_100000bytes_alloc_a8, 100000, 8);
rt_alloc!(rt_even_1000000bytes_alloc_a8, 1000000, 8);

// rt odd
rt_alloc!(rt_odd_10bytes_alloc_a8, 9, 8);
rt_alloc!(rt_odd_100bytes_alloc_a8, 99, 8);
rt_alloc!(rt_odd_1000bytes_alloc_a8, 999, 8);
rt_alloc!(rt_odd_10000bytes_alloc_a8, 9999, 8);
rt_alloc!(rt_odd_100000bytes_alloc_a8, 99999, 8);
rt_alloc!(rt_odd_1000000bytes_alloc_a8, 999999, 8);

// primes
rt_alloc!(rt_prime_3bytes_alloc_a8, 3, 8);
rt_alloc!(rt_prime_7bytes_alloc_a8, 7, 8);
rt_alloc!(rt_prime_13bytes_alloc_a8, 13, 8);
rt_alloc!(rt_prime_17bytes_alloc_a8, 17, 8);
rt_alloc!(rt_prime_31bytes_alloc_a8, 31, 8);
rt_alloc!(rt_prime_61bytes_alloc_a8, 61, 8);
rt_alloc!(rt_prime_97bytes_alloc_a8, 97, 8);
rt_alloc!(rt_prime_127bytes_alloc_a8, 127, 8);
rt_alloc!(rt_prime_257bytes_alloc_a8, 257, 8);
rt_alloc!(rt_prime_509bytes_alloc_a8, 509, 8);
rt_alloc!(rt_prime_1021bytes_alloc_a8, 1021, 8);
rt_alloc!(rt_prime_2039bytes_alloc_a8, 2039, 8);
rt_alloc!(rt_prime_4093bytes_alloc_a8, 4093, 8);
rt_alloc!(rt_prime_8191bytes_alloc_a8, 8191, 8);
rt_alloc!(rt_prime_16381bytes_alloc_a8, 16381, 8);
rt_alloc!(rt_prime_32749bytes_alloc_a8, 32749, 8);
rt_alloc!(rt_prime_65537bytes_alloc_a8, 65537, 8);
rt_alloc!(rt_prime_131071bytes_alloc_a8, 131071, 8);
rt_alloc!(rt_prime_4194301bytes_alloc_a8, 4194301, 8);


// rt powers of 2
rt_alloc_excess_unused!(rt_pow2_1bytes_alloc_excess_unused_a8, 1, 8);
rt_alloc_excess_unused!(rt_pow2_2bytes_alloc_excess_unused_a8, 2, 8);
rt_alloc_excess_unused!(rt_pow2_4bytes_alloc_excess_unused_a8, 4, 8);
rt_alloc_excess_unused!(rt_pow2_8bytes_alloc_excess_unused_a8, 8, 8);
rt_alloc_excess_unused!(rt_pow2_16bytes_alloc_excess_unused_a8, 16, 8);
rt_alloc_excess_unused!(rt_pow2_32bytes_alloc_excess_unused_a8, 32, 8);
rt_alloc_excess_unused!(rt_pow2_64bytes_alloc_excess_unused_a8, 64, 8);
rt_alloc_excess_unused!(rt_pow2_128bytes_alloc_excess_unused_a8, 128, 8);
rt_alloc_excess_unused!(rt_pow2_256bytes_alloc_excess_unused_a8, 256, 8);
rt_alloc_excess_unused!(rt_pow2_512bytes_alloc_excess_unused_a8, 512, 8);
rt_alloc_excess_unused!(rt_pow2_1024bytes_alloc_excess_unused_a8, 1024, 8);
rt_alloc_excess_unused!(rt_pow2_2048bytes_alloc_excess_unused_a8, 2048, 8);
rt_alloc_excess_unused!(rt_pow2_4096bytes_alloc_excess_unused_a8, 4096, 8);
rt_alloc_excess_unused!(rt_pow2_8192bytes_alloc_excess_unused_a8, 8192, 8);
rt_alloc_excess_unused!(rt_pow2_16384bytes_alloc_excess_unused_a8, 16384, 8);
rt_alloc_excess_unused!(rt_pow2_32768bytes_alloc_excess_unused_a8, 32768, 8);
rt_alloc_excess_unused!(rt_pow2_65536bytes_alloc_excess_unused_a8, 65536, 8);
rt_alloc_excess_unused!(rt_pow2_131072bytes_alloc_excess_unused_a8, 131072, 8);
rt_alloc_excess_unused!(rt_pow2_4194304bytes_alloc_excess_unused_a8, 4194304, 8);

// rt even
rt_alloc_excess_unused!(rt_even_10bytes_alloc_excess_unused_a8, 10, 8);
rt_alloc_excess_unused!(rt_even_100bytes_alloc_excess_unused_a8, 100, 8);
rt_alloc_excess_unused!(rt_even_1000bytes_alloc_excess_unused_a8, 1000, 8);
rt_alloc_excess_unused!(rt_even_10000bytes_alloc_excess_unused_a8, 10000, 8);
rt_alloc_excess_unused!(rt_even_100000bytes_alloc_excess_unused_a8, 100000, 8);
rt_alloc_excess_unused!(rt_even_1000000bytes_alloc_excess_unused_a8, 1000000, 8);

// rt odd
rt_alloc_excess_unused!(rt_odd_10bytes_alloc_excess_unused_a8, 9, 8);
rt_alloc_excess_unused!(rt_odd_100bytes_alloc_excess_unused_a8, 99, 8);
rt_alloc_excess_unused!(rt_odd_1000bytes_alloc_excess_unused_a8, 999, 8);
rt_alloc_excess_unused!(rt_odd_10000bytes_alloc_excess_unused_a8, 9999, 8);
rt_alloc_excess_unused!(rt_odd_100000bytes_alloc_excess_unused_a8, 99999, 8);
rt_alloc_excess_unused!(rt_odd_1000000bytes_alloc_excess_unused_a8, 999999, 8);

// primes
rt_alloc_excess_unused!(rt_prime_3bytes_alloc_excess_unused_a8, 3, 8);
rt_alloc_excess_unused!(rt_prime_7bytes_alloc_excess_unused_a8, 7, 8);
rt_alloc_excess_unused!(rt_prime_13bytes_alloc_excess_unused_a8, 13, 8);
rt_alloc_excess_unused!(rt_prime_17bytes_alloc_excess_unused_a8, 17, 8);
rt_alloc_excess_unused!(rt_prime_31bytes_alloc_excess_unused_a8, 31, 8);
rt_alloc_excess_unused!(rt_prime_61bytes_alloc_excess_unused_a8, 61, 8);
rt_alloc_excess_unused!(rt_prime_97bytes_alloc_excess_unused_a8, 97, 8);
rt_alloc_excess_unused!(rt_prime_127bytes_alloc_excess_unused_a8, 127, 8);
rt_alloc_excess_unused!(rt_prime_257bytes_alloc_excess_unused_a8, 257, 8);
rt_alloc_excess_unused!(rt_prime_509bytes_alloc_excess_unused_a8, 509, 8);
rt_alloc_excess_unused!(rt_prime_1021bytes_alloc_excess_unused_a8, 1021, 8);
rt_alloc_excess_unused!(rt_prime_2039bytes_alloc_excess_unused_a8, 2039, 8);
rt_alloc_excess_unused!(rt_prime_4093bytes_alloc_excess_unused_a8, 4093, 8);
rt_alloc_excess_unused!(rt_prime_8191bytes_alloc_excess_unused_a8, 8191, 8);
rt_alloc_excess_unused!(rt_prime_16381bytes_alloc_excess_unused_a8, 16381, 8);
rt_alloc_excess_unused!(rt_prime_32749bytes_alloc_excess_unused_a8, 32749, 8);
rt_alloc_excess_unused!(rt_prime_65537bytes_alloc_excess_unused_a8, 65537, 8);
rt_alloc_excess_unused!(rt_prime_131071bytes_alloc_excess_unused_a8, 131071, 8);
rt_alloc_excess_unused!(rt_prime_4194301bytes_alloc_excess_unused_a8, 4194301, 8);


// rt powers of 2
rt_alloc_excess_used!(rt_pow2_1bytes_alloc_excess_used_a8, 1, 8);
rt_alloc_excess_used!(rt_pow2_2bytes_alloc_excess_used_a8, 2, 8);
rt_alloc_excess_used!(rt_pow2_4bytes_alloc_excess_used_a8, 4, 8);
rt_alloc_excess_used!(rt_pow2_8bytes_alloc_excess_used_a8, 8, 8);
rt_alloc_excess_used!(rt_pow2_16bytes_alloc_excess_used_a8, 16, 8);
rt_alloc_excess_used!(rt_pow2_32bytes_alloc_excess_used_a8, 32, 8);
rt_alloc_excess_used!(rt_pow2_64bytes_alloc_excess_used_a8, 64, 8);
rt_alloc_excess_used!(rt_pow2_128bytes_alloc_excess_used_a8, 128, 8);
rt_alloc_excess_used!(rt_pow2_256bytes_alloc_excess_used_a8, 256, 8);
rt_alloc_excess_used!(rt_pow2_512bytes_alloc_excess_used_a8, 512, 8);
rt_alloc_excess_used!(rt_pow2_1024bytes_alloc_excess_used_a8, 1024, 8);
rt_alloc_excess_used!(rt_pow2_2048bytes_alloc_excess_used_a8, 2048, 8);
rt_alloc_excess_used!(rt_pow2_4096bytes_alloc_excess_used_a8, 4096, 8);
rt_alloc_excess_used!(rt_pow2_8192bytes_alloc_excess_used_a8, 8192, 8);
rt_alloc_excess_used!(rt_pow2_16384bytes_alloc_excess_used_a8, 16384, 8);
rt_alloc_excess_used!(rt_pow2_32768bytes_alloc_excess_used_a8, 32768, 8);
rt_alloc_excess_used!(rt_pow2_65536bytes_alloc_excess_used_a8, 65536, 8);
rt_alloc_excess_used!(rt_pow2_131072bytes_alloc_excess_used_a8, 131072, 8);
rt_alloc_excess_used!(rt_pow2_4194304bytes_alloc_excess_used_a8, 4194304, 8);

// rt even
rt_alloc_excess_used!(rt_even_10bytes_alloc_excess_used_a8, 10, 8);
rt_alloc_excess_used!(rt_even_100bytes_alloc_excess_used_a8, 100, 8);
rt_alloc_excess_used!(rt_even_1000bytes_alloc_excess_used_a8, 1000, 8);
rt_alloc_excess_used!(rt_even_10000bytes_alloc_excess_used_a8, 10000, 8);
rt_alloc_excess_used!(rt_even_100000bytes_alloc_excess_used_a8, 100000, 8);
rt_alloc_excess_used!(rt_even_1000000bytes_alloc_excess_used_a8, 1000000, 8);

// rt odd
rt_alloc_excess_used!(rt_odd_10bytes_alloc_excess_used_a8, 9, 8);
rt_alloc_excess_used!(rt_odd_100bytes_alloc_excess_used_a8, 99, 8);
rt_alloc_excess_used!(rt_odd_1000bytes_alloc_excess_used_a8, 999, 8);
rt_alloc_excess_used!(rt_odd_10000bytes_alloc_excess_used_a8, 9999, 8);
rt_alloc_excess_used!(rt_odd_100000bytes_alloc_excess_used_a8, 99999, 8);
rt_alloc_excess_used!(rt_odd_1000000bytes_alloc_excess_used_a8, 999999, 8);

// primes
rt_alloc_excess_used!(rt_prime_3bytes_alloc_excess_used_a8, 3, 8);
rt_alloc_excess_used!(rt_prime_7bytes_alloc_excess_used_a8, 7, 8);
rt_alloc_excess_used!(rt_prime_13bytes_alloc_excess_used_a8, 13, 8);
rt_alloc_excess_used!(rt_prime_17bytes_alloc_excess_used_a8, 17, 8);
rt_alloc_excess_used!(rt_prime_31bytes_alloc_excess_used_a8, 31, 8);
rt_alloc_excess_used!(rt_prime_61bytes_alloc_excess_used_a8, 61, 8);
rt_alloc_excess_used!(rt_prime_97bytes_alloc_excess_used_a8, 97, 8);
rt_alloc_excess_used!(rt_prime_127bytes_alloc_excess_used_a8, 127, 8);
rt_alloc_excess_used!(rt_prime_257bytes_alloc_excess_used_a8, 257, 8);
rt_alloc_excess_used!(rt_prime_509bytes_alloc_excess_used_a8, 509, 8);
rt_alloc_excess_used!(rt_prime_1021bytes_alloc_excess_used_a8, 1021, 8);
rt_alloc_excess_used!(rt_prime_2039bytes_alloc_excess_used_a8, 2039, 8);
rt_alloc_excess_used!(rt_prime_4093bytes_alloc_excess_used_a8, 4093, 8);
rt_alloc_excess_used!(rt_prime_8191bytes_alloc_excess_used_a8, 8191, 8);
rt_alloc_excess_used!(rt_prime_16381bytes_alloc_excess_used_a8, 16381, 8);
rt_alloc_excess_used!(rt_prime_32749bytes_alloc_excess_used_a8, 32749, 8);
rt_alloc_excess_used!(rt_prime_65537bytes_alloc_excess_used_a8, 65537, 8);
rt_alloc_excess_used!(rt_prime_131071bytes_alloc_excess_used_a8, 131071, 8);
rt_alloc_excess_used!(rt_prime_4194301bytes_alloc_excess_used_a8, 4194301, 8);


// alignment 16 bytes:

// rt powers of 2
rt_alloc!(rt_pow2_1bytes_alloc_a16, 1, 16);
rt_alloc!(rt_pow2_2bytes_alloc_a16, 2, 16);
rt_alloc!(rt_pow2_4bytes_alloc_a16, 4, 16);
rt_alloc!(rt_pow2_8bytes_alloc_a16, 8, 16);
rt_alloc!(rt_pow2_16bytes_alloc_a16, 16, 16);
rt_alloc!(rt_pow2_32bytes_alloc_a16, 32, 16);
rt_alloc!(rt_pow2_64bytes_alloc_a16, 64, 16);
rt_alloc!(rt_pow2_128bytes_alloc_a16, 128, 16);
rt_alloc!(rt_pow2_256bytes_alloc_a16, 256, 16);
rt_alloc!(rt_pow2_512bytes_alloc_a16, 512, 16);
rt_alloc!(rt_pow2_1024bytes_alloc_a16, 1024, 16);
rt_alloc!(rt_pow2_2048bytes_alloc_a16, 2048, 16);
rt_alloc!(rt_pow2_4096bytes_alloc_a16, 4096, 16);
rt_alloc!(rt_pow2_8192bytes_alloc_a16, 8192, 16);
rt_alloc!(rt_pow2_16384bytes_alloc_a16, 16384, 16);
rt_alloc!(rt_pow2_32768bytes_alloc_a16, 32768, 16);
rt_alloc!(rt_pow2_65536bytes_alloc_a16, 65536, 16);
rt_alloc!(rt_pow2_131072bytes_alloc_a16, 131072, 16);
rt_alloc!(rt_pow2_4194304bytes_alloc_a16, 4194304, 16);

// rt even
rt_alloc!(rt_even_10bytes_alloc_a16, 10, 16);
rt_alloc!(rt_even_100bytes_alloc_a16, 100, 16);
rt_alloc!(rt_even_1000bytes_alloc_a16, 1000, 16);
rt_alloc!(rt_even_10000bytes_alloc_a16, 10000, 16);
rt_alloc!(rt_even_100000bytes_alloc_a16, 100000, 16);
rt_alloc!(rt_even_1000000bytes_alloc_a16, 1000000, 16);

// rt odd
rt_alloc!(rt_odd_10bytes_alloc_a16, 9, 16);
rt_alloc!(rt_odd_100bytes_alloc_a16, 99, 16);
rt_alloc!(rt_odd_1000bytes_alloc_a16, 999, 16);
rt_alloc!(rt_odd_10000bytes_alloc_a16, 9999, 16);
rt_alloc!(rt_odd_100000bytes_alloc_a16, 99999, 16);
rt_alloc!(rt_odd_1000000bytes_alloc_a16, 999999, 16);

// primes
rt_alloc!(rt_prime_3bytes_alloc_a16, 3, 16);
rt_alloc!(rt_prime_7bytes_alloc_a16, 7, 16);
rt_alloc!(rt_prime_13bytes_alloc_a16, 13, 16);
rt_alloc!(rt_prime_17bytes_alloc_a16, 17, 16);
rt_alloc!(rt_prime_31bytes_alloc_a16, 31, 16);
rt_alloc!(rt_prime_61bytes_alloc_a16, 61, 16);
rt_alloc!(rt_prime_97bytes_alloc_a16, 97, 16);
rt_alloc!(rt_prime_127bytes_alloc_a16, 127, 16);
rt_alloc!(rt_prime_257bytes_alloc_a16, 257, 16);
rt_alloc!(rt_prime_509bytes_alloc_a16, 509, 16);
rt_alloc!(rt_prime_1021bytes_alloc_a16, 1021, 16);
rt_alloc!(rt_prime_2039bytes_alloc_a16, 2039, 16);
rt_alloc!(rt_prime_4093bytes_alloc_a16, 4093, 16);
rt_alloc!(rt_prime_8191bytes_alloc_a16, 8191, 16);
rt_alloc!(rt_prime_16381bytes_alloc_a16, 16381, 16);
rt_alloc!(rt_prime_32749bytes_alloc_a16, 32749, 16);
rt_alloc!(rt_prime_65537bytes_alloc_a16, 65537, 16);
rt_alloc!(rt_prime_131071bytes_alloc_a16, 131071, 16);
rt_alloc!(rt_prime_4194301bytes_alloc_a16, 4194301, 16);


// rt powers of 2
rt_alloc_excess_unused!(rt_pow2_1bytes_alloc_excess_unused_a16, 1, 16);
rt_alloc_excess_unused!(rt_pow2_2bytes_alloc_excess_unused_a16, 2, 16);
rt_alloc_excess_unused!(rt_pow2_4bytes_alloc_excess_unused_a16, 4, 16);
rt_alloc_excess_unused!(rt_pow2_8bytes_alloc_excess_unused_a16, 8, 16);
rt_alloc_excess_unused!(rt_pow2_16bytes_alloc_excess_unused_a16, 16, 16);
rt_alloc_excess_unused!(rt_pow2_32bytes_alloc_excess_unused_a16, 32, 16);
rt_alloc_excess_unused!(rt_pow2_64bytes_alloc_excess_unused_a16, 64, 16);
rt_alloc_excess_unused!(rt_pow2_128bytes_alloc_excess_unused_a16, 128, 16);
rt_alloc_excess_unused!(rt_pow2_256bytes_alloc_excess_unused_a16, 256, 16);
rt_alloc_excess_unused!(rt_pow2_512bytes_alloc_excess_unused_a16, 512, 16);
rt_alloc_excess_unused!(rt_pow2_1024bytes_alloc_excess_unused_a16, 1024, 16);
rt_alloc_excess_unused!(rt_pow2_2048bytes_alloc_excess_unused_a16, 2048, 16);
rt_alloc_excess_unused!(rt_pow2_4096bytes_alloc_excess_unused_a16, 4096, 16);
rt_alloc_excess_unused!(rt_pow2_8192bytes_alloc_excess_unused_a16, 8192, 16);
rt_alloc_excess_unused!(rt_pow2_16384bytes_alloc_excess_unused_a16, 16384, 16);
rt_alloc_excess_unused!(rt_pow2_32768bytes_alloc_excess_unused_a16, 32768, 16);
rt_alloc_excess_unused!(rt_pow2_65536bytes_alloc_excess_unused_a16, 65536, 16);
rt_alloc_excess_unused!(rt_pow2_131072bytes_alloc_excess_unused_a16, 131072, 16);
rt_alloc_excess_unused!(rt_pow2_4194304bytes_alloc_excess_unused_a16, 4194304, 16);

// rt even
rt_alloc_excess_unused!(rt_even_10bytes_alloc_excess_unused_a16, 10, 16);
rt_alloc_excess_unused!(rt_even_100bytes_alloc_excess_unused_a16, 100, 16);
rt_alloc_excess_unused!(rt_even_1000bytes_alloc_excess_unused_a16, 1000, 16);
rt_alloc_excess_unused!(rt_even_10000bytes_alloc_excess_unused_a16, 10000, 16);
rt_alloc_excess_unused!(rt_even_100000bytes_alloc_excess_unused_a16, 100000, 16);
rt_alloc_excess_unused!(rt_even_1000000bytes_alloc_excess_unused_a16, 1000000, 16);

// rt odd
rt_alloc_excess_unused!(rt_odd_10bytes_alloc_excess_unused_a16, 9, 16);
rt_alloc_excess_unused!(rt_odd_100bytes_alloc_excess_unused_a16, 99, 16);
rt_alloc_excess_unused!(rt_odd_1000bytes_alloc_excess_unused_a16, 999, 16);
rt_alloc_excess_unused!(rt_odd_10000bytes_alloc_excess_unused_a16, 9999, 16);
rt_alloc_excess_unused!(rt_odd_100000bytes_alloc_excess_unused_a16, 99999, 16);
rt_alloc_excess_unused!(rt_odd_1000000bytes_alloc_excess_unused_a16, 999999, 16);

// primes
rt_alloc_excess_unused!(rt_prime_3bytes_alloc_excess_unused_a16, 3, 16);
rt_alloc_excess_unused!(rt_prime_7bytes_alloc_excess_unused_a16, 7, 16);
rt_alloc_excess_unused!(rt_prime_13bytes_alloc_excess_unused_a16, 13, 16);
rt_alloc_excess_unused!(rt_prime_17bytes_alloc_excess_unused_a16, 17, 16);
rt_alloc_excess_unused!(rt_prime_31bytes_alloc_excess_unused_a16, 31, 16);
rt_alloc_excess_unused!(rt_prime_61bytes_alloc_excess_unused_a16, 61, 16);
rt_alloc_excess_unused!(rt_prime_97bytes_alloc_excess_unused_a16, 97, 16);
rt_alloc_excess_unused!(rt_prime_127bytes_alloc_excess_unused_a16, 127, 16);
rt_alloc_excess_unused!(rt_prime_257bytes_alloc_excess_unused_a16, 257, 16);
rt_alloc_excess_unused!(rt_prime_509bytes_alloc_excess_unused_a16, 509, 16);
rt_alloc_excess_unused!(rt_prime_1021bytes_alloc_excess_unused_a16, 1021, 16);
rt_alloc_excess_unused!(rt_prime_2039bytes_alloc_excess_unused_a16, 2039, 16);
rt_alloc_excess_unused!(rt_prime_4093bytes_alloc_excess_unused_a16, 4093, 16);
rt_alloc_excess_unused!(rt_prime_8191bytes_alloc_excess_unused_a16, 8191, 16);
rt_alloc_excess_unused!(rt_prime_16381bytes_alloc_excess_unused_a16, 16381, 16);
rt_alloc_excess_unused!(rt_prime_32749bytes_alloc_excess_unused_a16, 32749, 16);
rt_alloc_excess_unused!(rt_prime_65537bytes_alloc_excess_unused_a16, 65537, 16);
rt_alloc_excess_unused!(rt_prime_131071bytes_alloc_excess_unused_a16, 131071, 16);
rt_alloc_excess_unused!(rt_prime_4194301bytes_alloc_excess_unused_a16, 4194301, 16);

// rt powers of 2
rt_alloc_excess_used!(rt_pow2_1bytes_alloc_excess_used_a16, 1, 16);
rt_alloc_excess_used!(rt_pow2_2bytes_alloc_excess_used_a16, 2, 16);
rt_alloc_excess_used!(rt_pow2_4bytes_alloc_excess_used_a16, 4, 16);
rt_alloc_excess_used!(rt_pow2_8bytes_alloc_excess_used_a16, 8, 16);
rt_alloc_excess_used!(rt_pow2_16bytes_alloc_excess_used_a16, 16, 16);
rt_alloc_excess_used!(rt_pow2_32bytes_alloc_excess_used_a16, 32, 16);
rt_alloc_excess_used!(rt_pow2_64bytes_alloc_excess_used_a16, 64, 16);
rt_alloc_excess_used!(rt_pow2_128bytes_alloc_excess_used_a16, 128, 16);
rt_alloc_excess_used!(rt_pow2_256bytes_alloc_excess_used_a16, 256, 16);
rt_alloc_excess_used!(rt_pow2_512bytes_alloc_excess_used_a16, 512, 16);
rt_alloc_excess_used!(rt_pow2_1024bytes_alloc_excess_used_a16, 1024, 16);
rt_alloc_excess_used!(rt_pow2_2048bytes_alloc_excess_used_a16, 2048, 16);
rt_alloc_excess_used!(rt_pow2_4096bytes_alloc_excess_used_a16, 4096, 16);
rt_alloc_excess_used!(rt_pow2_8192bytes_alloc_excess_used_a16, 8192, 16);
rt_alloc_excess_used!(rt_pow2_16384bytes_alloc_excess_used_a16, 16384, 16);
rt_alloc_excess_used!(rt_pow2_32768bytes_alloc_excess_used_a16, 32768, 16);
rt_alloc_excess_used!(rt_pow2_65536bytes_alloc_excess_used_a16, 65536, 16);
rt_alloc_excess_used!(rt_pow2_131072bytes_alloc_excess_used_a16, 131072, 16);
rt_alloc_excess_used!(rt_pow2_4194304bytes_alloc_excess_used_a16, 4194304, 16);

// rt even
rt_alloc_excess_used!(rt_even_10bytes_alloc_excess_used_a16, 10, 16);
rt_alloc_excess_used!(rt_even_100bytes_alloc_excess_used_a16, 100, 16);
rt_alloc_excess_used!(rt_even_1000bytes_alloc_excess_used_a16, 1000, 16);
rt_alloc_excess_used!(rt_even_10000bytes_alloc_excess_used_a16, 10000, 16);
rt_alloc_excess_used!(rt_even_100000bytes_alloc_excess_used_a16, 100000, 16);
rt_alloc_excess_used!(rt_even_1000000bytes_alloc_excess_used_a16, 1000000, 16);

// rt odd
rt_alloc_excess_used!(rt_odd_10bytes_alloc_excess_used_a16, 9, 16);
rt_alloc_excess_used!(rt_odd_100bytes_alloc_excess_used_a16, 99, 16);
rt_alloc_excess_used!(rt_odd_1000bytes_alloc_excess_used_a16, 999, 16);
rt_alloc_excess_used!(rt_odd_10000bytes_alloc_excess_used_a16, 9999, 16);
rt_alloc_excess_used!(rt_odd_100000bytes_alloc_excess_used_a16, 99999, 16);
rt_alloc_excess_used!(rt_odd_1000000bytes_alloc_excess_used_a16, 999999, 16);

// primes
rt_alloc_excess_used!(rt_prime_3bytes_alloc_excess_used_a16, 3, 16);
rt_alloc_excess_used!(rt_prime_7bytes_alloc_excess_used_a16, 7, 16);
rt_alloc_excess_used!(rt_prime_13bytes_alloc_excess_used_a16, 13, 16);
rt_alloc_excess_used!(rt_prime_17bytes_alloc_excess_used_a16, 17, 16);
rt_alloc_excess_used!(rt_prime_31bytes_alloc_excess_used_a16, 31, 16);
rt_alloc_excess_used!(rt_prime_61bytes_alloc_excess_used_a16, 61, 16);
rt_alloc_excess_used!(rt_prime_97bytes_alloc_excess_used_a16, 97, 16);
rt_alloc_excess_used!(rt_prime_127bytes_alloc_excess_used_a16, 127, 16);
rt_alloc_excess_used!(rt_prime_257bytes_alloc_excess_used_a16, 257, 16);
rt_alloc_excess_used!(rt_prime_509bytes_alloc_excess_used_a16, 509, 16);
rt_alloc_excess_used!(rt_prime_1021bytes_alloc_excess_used_a16, 1021, 16);
rt_alloc_excess_used!(rt_prime_2039bytes_alloc_excess_used_a16, 2039, 16);
rt_alloc_excess_used!(rt_prime_4093bytes_alloc_excess_used_a16, 4093, 16);
rt_alloc_excess_used!(rt_prime_8191bytes_alloc_excess_used_a16, 8191, 16);
rt_alloc_excess_used!(rt_prime_16381bytes_alloc_excess_used_a16, 16381, 16);
rt_alloc_excess_used!(rt_prime_32749bytes_alloc_excess_used_a16, 32749, 16);
rt_alloc_excess_used!(rt_prime_65537bytes_alloc_excess_used_a16, 65537, 16);
rt_alloc_excess_used!(rt_prime_131071bytes_alloc_excess_used_a16, 131071, 16);
rt_alloc_excess_used!(rt_prime_4194301bytes_alloc_excess_used_a16, 4194301, 16);


// alignment 32 bytes:

// rt powers of 2
rt_alloc!(rt_pow2_1bytes_alloc_a32, 1, 32);
rt_alloc!(rt_pow2_2bytes_alloc_a32, 2, 32);
rt_alloc!(rt_pow2_4bytes_alloc_a32, 4, 32);
rt_alloc!(rt_pow2_8bytes_alloc_a32, 8, 32);
rt_alloc!(rt_pow2_16bytes_alloc_a32, 16, 32);
rt_alloc!(rt_pow2_32bytes_alloc_a32, 32, 32);
rt_alloc!(rt_pow2_64bytes_alloc_a32, 64, 32);
rt_alloc!(rt_pow2_128bytes_alloc_a32, 128, 32);
rt_alloc!(rt_pow2_256bytes_alloc_a32, 256, 32);
rt_alloc!(rt_pow2_512bytes_alloc_a32, 512, 32);
rt_alloc!(rt_pow2_1024bytes_alloc_a32, 1024, 32);
rt_alloc!(rt_pow2_2048bytes_alloc_a32, 2048, 32);
rt_alloc!(rt_pow2_4096bytes_alloc_a32, 4096, 32);
rt_alloc!(rt_pow2_8192bytes_alloc_a32, 8192, 32);
rt_alloc!(rt_pow2_16384bytes_alloc_a32, 16384, 32);
rt_alloc!(rt_pow2_32768bytes_alloc_a32, 32768, 32);
rt_alloc!(rt_pow2_65536bytes_alloc_a32, 65536, 32);
rt_alloc!(rt_pow2_131072bytes_alloc_a32, 131072, 32);
rt_alloc!(rt_pow2_4194304bytes_alloc_a32, 4194304, 32);

// rt even
rt_alloc!(rt_even_10bytes_alloc_a32, 10, 32);
rt_alloc!(rt_even_100bytes_alloc_a32, 100, 32);
rt_alloc!(rt_even_1000bytes_alloc_a32, 1000, 32);
rt_alloc!(rt_even_10000bytes_alloc_a32, 10000, 32);
rt_alloc!(rt_even_100000bytes_alloc_a32, 100000, 32);
rt_alloc!(rt_even_1000000bytes_alloc_a32, 1000000, 32);

// rt odd
rt_alloc!(rt_odd_10bytes_alloc_a32, 9, 32);
rt_alloc!(rt_odd_100bytes_alloc_a32, 99, 32);
rt_alloc!(rt_odd_1000bytes_alloc_a32, 999, 32);
rt_alloc!(rt_odd_10000bytes_alloc_a32, 9999, 32);
rt_alloc!(rt_odd_100000bytes_alloc_a32, 99999, 32);
rt_alloc!(rt_odd_1000000bytes_alloc_a32, 999999, 32);

// primes
rt_alloc!(rt_prime_3bytes_alloc_a32, 3, 32);
rt_alloc!(rt_prime_7bytes_alloc_a32, 7, 32);
rt_alloc!(rt_prime_13bytes_alloc_a32, 13, 32);
rt_alloc!(rt_prime_17bytes_alloc_a32, 17, 32);
rt_alloc!(rt_prime_31bytes_alloc_a32, 31, 32);
rt_alloc!(rt_prime_61bytes_alloc_a32, 61, 32);
rt_alloc!(rt_prime_97bytes_alloc_a32, 97, 32);
rt_alloc!(rt_prime_127bytes_alloc_a32, 127, 32);
rt_alloc!(rt_prime_257bytes_alloc_a32, 257, 32);
rt_alloc!(rt_prime_509bytes_alloc_a32, 509, 32);
rt_alloc!(rt_prime_1021bytes_alloc_a32, 1021, 32);
rt_alloc!(rt_prime_2039bytes_alloc_a32, 2039, 32);
rt_alloc!(rt_prime_4093bytes_alloc_a32, 4093, 32);
rt_alloc!(rt_prime_8191bytes_alloc_a32, 8191, 32);
rt_alloc!(rt_prime_16381bytes_alloc_a32, 16381, 32);
rt_alloc!(rt_prime_32749bytes_alloc_a32, 32749, 32);
rt_alloc!(rt_prime_65537bytes_alloc_a32, 65537, 32);
rt_alloc!(rt_prime_131071bytes_alloc_a32, 131071, 32);
rt_alloc!(rt_prime_4194301bytes_alloc_a32, 4194301, 32);


// rt powers of 2
rt_alloc_excess_unused!(rt_pow2_1bytes_alloc_excess_unused_a32, 1, 32);
rt_alloc_excess_unused!(rt_pow2_2bytes_alloc_excess_unused_a32, 2, 32);
rt_alloc_excess_unused!(rt_pow2_4bytes_alloc_excess_unused_a32, 4, 32);
rt_alloc_excess_unused!(rt_pow2_8bytes_alloc_excess_unused_a32, 8, 32);
rt_alloc_excess_unused!(rt_pow2_16bytes_alloc_excess_unused_a32, 16, 32);
rt_alloc_excess_unused!(rt_pow2_32bytes_alloc_excess_unused_a32, 32, 32);
rt_alloc_excess_unused!(rt_pow2_64bytes_alloc_excess_unused_a32, 64, 32);
rt_alloc_excess_unused!(rt_pow2_128bytes_alloc_excess_unused_a32, 128, 32);
rt_alloc_excess_unused!(rt_pow2_256bytes_alloc_excess_unused_a32, 256, 32);
rt_alloc_excess_unused!(rt_pow2_512bytes_alloc_excess_unused_a32, 512, 32);
rt_alloc_excess_unused!(rt_pow2_1024bytes_alloc_excess_unused_a32, 1024, 32);
rt_alloc_excess_unused!(rt_pow2_2048bytes_alloc_excess_unused_a32, 2048, 32);
rt_alloc_excess_unused!(rt_pow2_4096bytes_alloc_excess_unused_a32, 4096, 32);
rt_alloc_excess_unused!(rt_pow2_8192bytes_alloc_excess_unused_a32, 8192, 32);
rt_alloc_excess_unused!(rt_pow2_16384bytes_alloc_excess_unused_a32, 16384, 32);
rt_alloc_excess_unused!(rt_pow2_32768bytes_alloc_excess_unused_a32, 32768, 32);
rt_alloc_excess_unused!(rt_pow2_65536bytes_alloc_excess_unused_a32, 65536, 32);
rt_alloc_excess_unused!(rt_pow2_131072bytes_alloc_excess_unused_a32, 131072, 32);
rt_alloc_excess_unused!(rt_pow2_4194304bytes_alloc_excess_unused_a32, 4194304, 32);

// rt even
rt_alloc_excess_unused!(rt_even_10bytes_alloc_excess_unused_a32, 10, 32);
rt_alloc_excess_unused!(rt_even_100bytes_alloc_excess_unused_a32, 100, 32);
rt_alloc_excess_unused!(rt_even_1000bytes_alloc_excess_unused_a32, 1000, 32);
rt_alloc_excess_unused!(rt_even_10000bytes_alloc_excess_unused_a32, 10000, 32);
rt_alloc_excess_unused!(rt_even_100000bytes_alloc_excess_unused_a32, 100000, 32);
rt_alloc_excess_unused!(rt_even_1000000bytes_alloc_excess_unused_a32, 1000000, 32);

// rt odd
rt_alloc_excess_unused!(rt_odd_10bytes_alloc_excess_unused_a32, 9, 32);
rt_alloc_excess_unused!(rt_odd_100bytes_alloc_excess_unused_a32, 99, 32);
rt_alloc_excess_unused!(rt_odd_1000bytes_alloc_excess_unused_a32, 999, 32);
rt_alloc_excess_unused!(rt_odd_10000bytes_alloc_excess_unused_a32, 9999, 32);
rt_alloc_excess_unused!(rt_odd_100000bytes_alloc_excess_unused_a32, 99999, 32);
rt_alloc_excess_unused!(rt_odd_1000000bytes_alloc_excess_unused_a32, 999999, 32);

// primes
rt_alloc_excess_unused!(rt_prime_3bytes_alloc_excess_unused_a32, 3, 32);
rt_alloc_excess_unused!(rt_prime_7bytes_alloc_excess_unused_a32, 7, 32);
rt_alloc_excess_unused!(rt_prime_13bytes_alloc_excess_unused_a32, 13, 32);
rt_alloc_excess_unused!(rt_prime_17bytes_alloc_excess_unused_a32, 17, 32);
rt_alloc_excess_unused!(rt_prime_31bytes_alloc_excess_unused_a32, 31, 32);
rt_alloc_excess_unused!(rt_prime_61bytes_alloc_excess_unused_a32, 61, 32);
rt_alloc_excess_unused!(rt_prime_97bytes_alloc_excess_unused_a32, 97, 32);
rt_alloc_excess_unused!(rt_prime_127bytes_alloc_excess_unused_a32, 127, 32);
rt_alloc_excess_unused!(rt_prime_257bytes_alloc_excess_unused_a32, 257, 32);
rt_alloc_excess_unused!(rt_prime_509bytes_alloc_excess_unused_a32, 509, 32);
rt_alloc_excess_unused!(rt_prime_1021bytes_alloc_excess_unused_a32, 1021, 32);
rt_alloc_excess_unused!(rt_prime_2039bytes_alloc_excess_unused_a32, 2039, 32);
rt_alloc_excess_unused!(rt_prime_4093bytes_alloc_excess_unused_a32, 4093, 32);
rt_alloc_excess_unused!(rt_prime_8191bytes_alloc_excess_unused_a32, 8191, 32);
rt_alloc_excess_unused!(rt_prime_16381bytes_alloc_excess_unused_a32, 16381, 32);
rt_alloc_excess_unused!(rt_prime_32749bytes_alloc_excess_unused_a32, 32749, 32);
rt_alloc_excess_unused!(rt_prime_65537bytes_alloc_excess_unused_a32, 65537, 32);
rt_alloc_excess_unused!(rt_prime_131071bytes_alloc_excess_unused_a32, 131071, 32);
rt_alloc_excess_unused!(rt_prime_4194301bytes_alloc_excess_unused_a32, 4194301, 32);


// rt powers of 2
rt_alloc_excess_used!(rt_pow2_1bytes_alloc_excess_used_a32, 1, 32);
rt_alloc_excess_used!(rt_pow2_2bytes_alloc_excess_used_a32, 2, 32);
rt_alloc_excess_used!(rt_pow2_4bytes_alloc_excess_used_a32, 4, 32);
rt_alloc_excess_used!(rt_pow2_8bytes_alloc_excess_used_a32, 8, 32);
rt_alloc_excess_used!(rt_pow2_16bytes_alloc_excess_used_a32, 16, 32);
rt_alloc_excess_used!(rt_pow2_32bytes_alloc_excess_used_a32, 32, 32);
rt_alloc_excess_used!(rt_pow2_64bytes_alloc_excess_used_a32, 64, 32);
rt_alloc_excess_used!(rt_pow2_128bytes_alloc_excess_used_a32, 128, 32);
rt_alloc_excess_used!(rt_pow2_256bytes_alloc_excess_used_a32, 256, 32);
rt_alloc_excess_used!(rt_pow2_512bytes_alloc_excess_used_a32, 512, 32);
rt_alloc_excess_used!(rt_pow2_1024bytes_alloc_excess_used_a32, 1024, 32);
rt_alloc_excess_used!(rt_pow2_2048bytes_alloc_excess_used_a32, 2048, 32);
rt_alloc_excess_used!(rt_pow2_4096bytes_alloc_excess_used_a32, 4096, 32);
rt_alloc_excess_used!(rt_pow2_8192bytes_alloc_excess_used_a32, 8192, 32);
rt_alloc_excess_used!(rt_pow2_16384bytes_alloc_excess_used_a32, 16384, 32);
rt_alloc_excess_used!(rt_pow2_32768bytes_alloc_excess_used_a32, 32768, 32);
rt_alloc_excess_used!(rt_pow2_65536bytes_alloc_excess_used_a32, 65536, 32);
rt_alloc_excess_used!(rt_pow2_131072bytes_alloc_excess_used_a32, 131072, 32);
rt_alloc_excess_used!(rt_pow2_4194304bytes_alloc_excess_used_a32, 4194304, 32);

// rt even
rt_alloc_excess_used!(rt_even_10bytes_alloc_excess_used_a32, 10, 32);
rt_alloc_excess_used!(rt_even_100bytes_alloc_excess_used_a32, 100, 32);
rt_alloc_excess_used!(rt_even_1000bytes_alloc_excess_used_a32, 1000, 32);
rt_alloc_excess_used!(rt_even_10000bytes_alloc_excess_used_a32, 10000, 32);
rt_alloc_excess_used!(rt_even_100000bytes_alloc_excess_used_a32, 100000, 32);
rt_alloc_excess_used!(rt_even_1000000bytes_alloc_excess_used_a32, 1000000, 32);

// rt odd
rt_alloc_excess_used!(rt_odd_10bytes_alloc_excess_used_a32, 9, 32);
rt_alloc_excess_used!(rt_odd_100bytes_alloc_excess_used_a32, 99, 32);
rt_alloc_excess_used!(rt_odd_1000bytes_alloc_excess_used_a32, 999, 32);
rt_alloc_excess_used!(rt_odd_10000bytes_alloc_excess_used_a32, 9999, 32);
rt_alloc_excess_used!(rt_odd_100000bytes_alloc_excess_used_a32, 99999, 32);
rt_alloc_excess_used!(rt_odd_1000000bytes_alloc_excess_used_a32, 999999, 32);

// primes
rt_alloc_excess_used!(rt_prime_3bytes_alloc_excess_used_a32, 3, 32);
rt_alloc_excess_used!(rt_prime_7bytes_alloc_excess_used_a32, 7, 32);
rt_alloc_excess_used!(rt_prime_13bytes_alloc_excess_used_a32, 13, 32);
rt_alloc_excess_used!(rt_prime_17bytes_alloc_excess_used_a32, 17, 32);
rt_alloc_excess_used!(rt_prime_31bytes_alloc_excess_used_a32, 31, 32);
rt_alloc_excess_used!(rt_prime_61bytes_alloc_excess_used_a32, 61, 32);
rt_alloc_excess_used!(rt_prime_97bytes_alloc_excess_used_a32, 97, 32);
rt_alloc_excess_used!(rt_prime_127bytes_alloc_excess_used_a32, 127, 32);
rt_alloc_excess_used!(rt_prime_257bytes_alloc_excess_used_a32, 257, 32);
rt_alloc_excess_used!(rt_prime_509bytes_alloc_excess_used_a32, 509, 32);
rt_alloc_excess_used!(rt_prime_1021bytes_alloc_excess_used_a32, 1021, 32);
rt_alloc_excess_used!(rt_prime_2039bytes_alloc_excess_used_a32, 2039, 32);
rt_alloc_excess_used!(rt_prime_4093bytes_alloc_excess_used_a32, 4093, 32);
rt_alloc_excess_used!(rt_prime_8191bytes_alloc_excess_used_a32, 8191, 32);
rt_alloc_excess_used!(rt_prime_16381bytes_alloc_excess_used_a32, 16381, 32);
rt_alloc_excess_used!(rt_prime_32749bytes_alloc_excess_used_a32, 32749, 32);
rt_alloc_excess_used!(rt_prime_65537bytes_alloc_excess_used_a32, 65537, 32);
rt_alloc_excess_used!(rt_prime_131071bytes_alloc_excess_used_a32, 131071, 32);
rt_alloc_excess_used!(rt_prime_4194301bytes_alloc_excess_used_a32, 4194301, 32);
