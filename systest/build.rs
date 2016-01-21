extern crate ctest;

use std::env;
use std::path::PathBuf;

fn main() {
    let root = PathBuf::from(env::var_os("DEP_JEMALLOC_ROOT").unwrap());

    let mut cfg = ctest::TestGenerator::new();
    cfg.header("jemalloc/jemalloc.h")
       .include(root.join("include"));
    cfg.generate("../jemalloc-sys/src/lib.rs", "all.rs");
}
