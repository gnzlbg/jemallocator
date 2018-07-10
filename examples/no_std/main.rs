#![feature(test, allocator_api)]
#![no_std]

extern crate test;
use test::black_box;

extern crate jemallocator;
use jemallocator::Jemalloc;

use core::alloc::{Layout};

#[cfg_attr(not(feature = "no_global"), global_allocator)]
static mut ALLOC: Jemalloc = Jemalloc;

fn main() {
    let mut size = 1;
    let max_bytes = 100_000_000;
    loop {
        if size > max_bytes {
            break;
        }
        for &align in &[1_usize, 2, 4, 8, 16, 32] {
            unsafe {
                run_global_alloc(size+1, align);

                #[cfg(feature = "alloc_trait")] {
                    run_alloc(size+1, align);
                }
            }
        }
        size *= 5;
    }
}

unsafe fn run_global_alloc(bytes: usize, align: usize) {
    use core::alloc::{GlobalAlloc};
    // test alloc
    {
        let init_l = Layout::from_size_align(bytes, align).unwrap();
        let new_l = Layout::from_size_align(3*bytes, align).unwrap();
        let x: *mut u8 = ALLOC.alloc(init_l);
        let x = black_box(x);
        touch(x, init_l.size());
        let x = ALLOC.realloc(x, init_l, new_l.size());
        touch(x, new_l.size());
        let x = black_box(x);
        ALLOC.dealloc(x, new_l);
    }
    // test alloc_zeroed
    {
        let init_l = Layout::from_size_align(bytes, align).unwrap();
        let new_l = Layout::from_size_align(2*bytes, align).unwrap();
        let x: *mut u8 = ALLOC.alloc_zeroed(init_l);
        let x = black_box(x);
        touch(x, init_l.size());
        let x = ALLOC.realloc(x, init_l, new_l.size());
        touch(x, new_l.size());
        let x = black_box(x);
        ALLOC.dealloc(x, new_l);
    }
}

#[cfg(feature = "alloc_trait")]
unsafe fn run_alloc(bytes: usize, align: usize) {
    use core::alloc::{Alloc, Excess};
    // test alloc
    {
        let init_l = Layout::from_size_align(bytes, align).unwrap();
        let new_l = Layout::from_size_align(3*bytes, align).unwrap();
        let Excess(x, e) = ALLOC.alloc_excess(init_l).unwrap();
        let x = black_box(x);
        touch(x.as_ptr(), init_l.size());
        ALLOC.grow_in_place(x, init_l, e).unwrap();
        let excess_l = Layout::from_size_align(e, align).unwrap();
        touch(x.as_ptr(), excess_l.size());
        ALLOC.shrink_in_place(x, excess_l, init_l.size()).unwrap();
        touch(x.as_ptr(), init_l.size());
        let Excess(x, e) = ALLOC.realloc_excess(x, init_l, new_l.size()).unwrap();
        let x = black_box(x);
        touch(x.as_ptr(), new_l.size());
        ALLOC.grow_in_place(x, new_l, e).unwrap();
        let excess_l = Layout::from_size_align(e, align).unwrap();
        touch(x.as_ptr(), e);
        ALLOC.shrink_in_place(x, excess_l, new_l.size()).unwrap();
        touch(x.as_ptr(), new_l.size());
        ALLOC.dealloc(x, new_l);
    }
}

unsafe fn touch(mut ptr: *mut u8, size: usize) {
    black_box(&mut ptr);
    for i in 0..size {
        ptr.wrapping_add(i).write(1);
    }
    black_box(&mut ptr);
}
