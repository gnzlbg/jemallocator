#![feature(test, global_allocator, allocator_api)]

extern crate jemallocator;
extern crate test;
extern crate libc;

use std::heap::{Alloc, Layout};

use test::Bencher;

use jemallocator::Jemalloc;

#[global_allocator]
static A: Jemalloc = Jemalloc;

// FIXME: replace with utils::mallocx_align
fn mallocx_align(a: usize) -> c_int {
    a.trailing_zeros() as c_int
}

// FIXME: replace with utils::align_to_flags
fn align_to_flags(align: usize) -> c_int {
    if align <= MIN_ALIGN {
        0
    } else {
        mallocx_align(align)
    }
}


macro_rules! rt_mallocx {
    ($name:ident, $nbytes:expr, $align:expr) => {
        #[bench]
        fn $name(b: &mut Bencher) {
            b.iter(|| unsafe {
                let flags = align_to_flags($align);
                let ptr = jemalloc::mallocx($nbytes, flags);
                test::black_box(ptr);
                jemalloc::sdallocx(ptr, $nbytes, flags);
            });
        }
    }
}

macro_rules! rt_mallocx_nallocx {
    ($name:ident, $nbytes:expr, $align:expr) => {
        #[bench]
        fn $name(b: &mut Bencher) {
            b.iter(|| unsafe {
                let flags = align_to_flags($align);
                let ptr = jemalloc::mallocx($nbytes, flags);
                test::black_box(ptr);
                let rsz = jemalloc::nallocx($nbytes, flags);
                test::black_box(rsz);
                jemalloc::sdallocx(ptr, rsz, flags);
            });
        }
    }
}

macro_rules! rt_alloc_layout_checked {
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

macro_rules! rt_alloc_layout_unchecked {
    ($name:ident, $nbytes:expr, $align:expr) => {
        #[bench]
        fn $name(b: &mut Bencher) {
            b.iter(|| unsafe {
                let layout = Layout::from_size_align_unchecked($nbytes, $align);
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

macro_rules! rt_mallocx_zeroed {
    ($name:ident, $nbytes:expr, $align:expr) => {
        #[bench]
        fn $name(b: &mut Bencher) {
            b.iter(|| unsafe {
                let flags = align_to_flags($align);
                let ptr = jemalloc::mallocx($nbytes, flags | jemalloc::MALLOCX_ZERO);
                test::black_box(ptr);
                jemalloc::sdallocx(ptr, $nbytes, flags);
            });
        }
    }
}

macro_rules! rt_calloc {
    ($name:ident, $nbytes:expr, $align:expr) => {
        #[bench]
        fn $name(b: &mut Bencher) {
            b.iter(|| unsafe {
                let flags = align_to_flags($align);
                let ptr = jemalloc::calloc(1, $nbytes);
                test::black_box(ptr);
                jemalloc::sdallocx(ptr, $nbytes, flags);
            });
        }
    }
}

macro_rules! rt_realloc_naive {
    ($name:ident, $nbytes:expr, $align:expr) => {
        #[bench]
        fn $name(b: &mut Bencher) {
            b.iter(|| unsafe {
                let layout = Layout::from_size_align($nbytes, $align).unwrap();
                let ptr = Jemalloc.alloc(layout.clone()).unwrap();
                test::black_box(ptr);

                // navie realloc:
                let new_layout = Layout::from_size_align(2 * $nbytes, $align).unwrap();
                let ptr = {
                    let new_ptr = Jemalloc.alloc(new_layout.clone()).unwrap();
                    ptr::copy_nonoverlapping(
                        ptr as *const u8, new_ptr, layout.size());
                    Jemalloc.dealloc(ptr, layout);
                    new_ptr
                };
                test::black_box(ptr);

                Jemalloc.dealloc(ptr, new_layout);
            });
        }
    }
}

macro_rules! rt_realloc {
    ($name:ident, $nbytes:expr, $align:expr) => {
        #[bench]
        fn $name(b: &mut Bencher) {
            b.iter(|| unsafe {
                let layout = Layout::from_size_align($nbytes, $align).unwrap();
                let ptr = Jemalloc.alloc(layout.clone()).unwrap();
                test::black_box(ptr);

                let new_layout = Layout::from_size_align(2 * $nbytes, $align).unwrap();
                let ptr = Jemalloc.realloc(ptr, layout, new_layout.clone()).unwrap();
                test::black_box(ptr);

                Jemalloc.dealloc(ptr, new_layout);
            });
        }
    }
}

macro_rules! rt_realloc_excess_unused {
    ($name:ident, $nbytes:expr, $align:expr) => {
        #[bench]
        fn $name(b: &mut Bencher) {
            b.iter(|| unsafe {
                let layout = Layout::from_size_align($nbytes, $align).unwrap();
                let ptr = Jemalloc.alloc(layout.clone()).unwrap();
                test::black_box(ptr);

                let new_layout = Layout::from_size_align(2 * $nbytes, $align).unwrap();
                let Excess(ptr, _) = Jemalloc.realloc_excess(
                    ptr, layout, new_layout.clone()
                ).unwrap();
                test::black_box(ptr);

                Jemalloc.dealloc(ptr, new_layout);
            });
        }
    }
}

macro_rules! rt_realloc_excess_checked_used {
    ($name:ident, $nbytes:expr, $align:expr) => {
        #[bench]
        fn $name(b: &mut Bencher) {
            b.iter(|| unsafe {
                let layout = Layout::from_size_align($nbytes, $align).unwrap();
                let ptr = Jemalloc.alloc(layout.clone()).unwrap();
                test::black_box(ptr);

                let new_layout = Layout::from_size_align(2 * $nbytes, $align).unwrap();
                let Excess(ptr, excess) = Jemalloc.realloc_excess(
                    ptr, layout, new_layout.clone()
                ).unwrap();
                test::black_box(ptr);
                test::black_box(excess);

                Jemalloc.dealloc(ptr, new_layout);
            });
        }
    }
}

// Powers of two, 1byte alignment:
rt_calloc!(rt_pow2_1bytes_1align_calloc, 1, 1);
rt_mallocx!(rt_pow2_1bytes_1align_mallocx, 1, 1);
rt_mallocx_zeroed!(rt_pow2_1bytes_1align_mallocx_zeroed, 1, 1);
rt_mallocx_nallocx!(rt_pow2_1bytes_1align_mallocx_nallocx, 1, 1);
rt_alloc_layout_checked!(rt_pow2_1bytes_1align_alloc_layout_checked, 1, 1);
rt_alloc_layout_unchecked!(rt_pow2_1bytes_1align_alloc_layout_unchecked, 1, 1);
rt_alloc_excess_unused(rt_pow2_1bytes_1align_alloc_excess_unused, 1, 1);
rt_alloc_excess_used(rt_pow2_1bytes_1align_alloc_excess_used, 1, 1);
rt_realloc_naive!(rt_pow2_1bytes_1align_realloc_naive, 1, 1);
rt_realloc!(rt_pow2_1bytes_1align_realloc_naive, 1, 1);
rt_realloc_excess_unused(rt_pow2_1bytes_1align_realloc_excess_unused, 1, 1);
rt_realloc_excess_used(rt_pow2_1bytes_1align_realloc_excess_used, 1, 1);

rt_calloc!(rt_pow2_2bytes_1align_calloc, 2, 1);
rt_mallocx!(rt_pow2_2bytes_1align_mallocx, 2, 1);
rt_mallocx_zeroed!(rt_pow2_2bytes_1align_mallocx_zeroed, 2, 1);
rt_mallocx_nallocx!(rt_pow2_2bytes_1align_mallocx_nallocx, 2, 1);
rt_alloc_layout_checked!(rt_pow2_2bytes_1align_alloc_layout_checked, 2, 1);
rt_alloc_layout_unchecked!(rt_pow2_2bytes_1align_alloc_layout_unchecked, 2, 1);
rt_alloc_excess_unused(rt_pow2_2bytes_1align_alloc_excess_unused, 2, 1);
rt_alloc_excess_used(rt_pow2_2bytes_1align_alloc_excess_used, 2, 1);
rt_realloc_naive!(rt_pow2_2bytes_1align_realloc_naive, 2, 1);
rt_realloc!(rt_pow2_2bytes_1align_realloc_naive, 2, 1);
rt_realloc_excess_unused(rt_pow2_2bytes_1align_realloc_excess_unused, 2, 1);
rt_realloc_excess_used(rt_pow2_2bytes_1align_realloc_excess_used, 2, 1);
