#![feature(test)]

extern crate test;
use test::black_box;

extern crate jemallocator;
use jemallocator::Jemalloc;

#[global_allocator]
static ALLOC: Jemalloc = Jemalloc;

fn main() {
    let max_bytes: u64 = 5_000_000_000;

    let _ = run_test(vector_zero_initialized, max_bytes);
    let _ = run_test(vector_ones_initialized, max_bytes);
    let _ = run_test(vector_grow, max_bytes);
}

fn run_test<F>(f: F, max_bytes: u64)
    where F: Fn(u64) -> () + Send + 'static
{
    let _ = std::thread::spawn(move || {
        let mut start = 1;
        loop {
            if start > max_bytes {
                break;
            }
            f(start);
            start *= 5;
        }
    });
}

#[inline(never)]
fn vector_zero_initialized(bytes: u64) {
    let v = vec![0_u8; bytes as usize];
    black_box(v);
}

#[inline(never)]
fn vector_ones_initialized(bytes: u64) {
    let v = vec![1_u8; bytes as usize];
    black_box(v);
}

#[inline(never)]
fn vector_grow(bytes: u64) {
    let mut v: Vec<u64> = Vec::new();
    for i in 0..bytes / 8 {
        v.push(i);
        black_box(&v);
    }
    black_box(v);
}
