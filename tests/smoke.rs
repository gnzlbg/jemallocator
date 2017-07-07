#![feature(global_allocator)]

extern crate jemallocator;

use jemallocator::Jemalloc;

#[global_allocator]
static A: Jemalloc = Jemalloc;

#[test]
fn smoke() {
    let mut a = Vec::new();
    a.push(3);
}
