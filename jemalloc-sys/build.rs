// Copyright 2015 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

extern crate cc;

use std::env;
use std::fs;
use std::path::PathBuf;
use std::process::Command;

fn gnu_target(target: &str) -> String {
    match target {
        "i686-pc-windows-msvc" => "i686-pc-win32".to_string(),
        "x86_64-pc-windows-msvc" => "x86_64-pc-win32".to_string(),
        "i686-pc-windows-gnu" => "i686-w64-mingw32".to_string(),
        "x86_64-pc-windows-gnu" => "x86_64-w64-mingw32".to_string(),
        s => s.to_string(),
    }
}

fn main() {
    let target = env::var("TARGET").expect("TARGET was not set");
    let host = env::var("HOST").expect("HOST was not set");
    let unsupported_targets = [
        "rumprun", "bitrig", "openbsd", "msvc",
        "emscripten", "fuchsia", "redox", "wasm32",
    ];
    for i in &unsupported_targets {
        if target.contains(i) {
            panic!("jemalloc does not support target: {}", target);
        }
    }

    let out_dir = PathBuf::from(env::var_os("OUT_DIR").unwrap());
    let build_dir = out_dir.join("build");
    let src_dir = env::current_dir().unwrap();

    if let Some(jemalloc) = env::var_os("JEMALLOC_OVERRIDE") {
        let jemalloc = PathBuf::from(jemalloc);
        println!("cargo:rustc-link-search=native={}",
                 jemalloc.parent().unwrap().display());
        let stem = jemalloc.file_stem().unwrap().to_str().unwrap();
        let name = jemalloc.file_name().unwrap().to_str().unwrap();
        let kind = if name.ends_with(".a") {"static"} else {"dylib"};
        println!("cargo:rustc-link-lib={}={}", kind, &stem[3..]);
        return
    }

    fs::create_dir_all(&build_dir).unwrap();

    let compiler = cc::Build::new().get_compiler();
    let cflags = compiler.args().iter().map(|s| s.to_str().unwrap())
                         .collect::<Vec<_>>().join(" ");


    let configure = src_dir.join("jemalloc/configure");
    let mut cmd = Command::new("sh");
    cmd.arg(configure.to_str().unwrap()
                   .replace("C:\\", "/c/")
                   .replace("\\", "/"))
       .current_dir(&build_dir)
       .env("CC", compiler.path())
       .env("CFLAGS", cflags.clone())
       .env("CPPFLAGS", cflags.clone())
       .arg("--disable-cxx") ;

    // jemalloc's configure doesn't detect this value
    // automatically for this target:
    if target == "sparc64-unknown-linux-gnu" {
        cmd.arg("--with-lg-quantum=4");
    }

    if target.contains("ios") {
        cmd.arg("--disable-tls");
    } else if target.contains("android") {
        // We force android to have prefixed symbols because apparently
        // replacement of the libc allocator doesn't quite work. When this was
        // tested (unprefixed symbols), it was found that the `realpath`
        // function in libc would allocate with libc malloc (not jemalloc
        // malloc), and then the standard library would free with jemalloc free,
        // causing a segfault.
        //
        // If the test suite passes, however, without symbol prefixes then we
        // should be good to go!
        cmd.arg("--with-jemalloc-prefix=je_");
        cmd.arg("--disable-tls");
    } else if target.contains("dragonfly") || target.contains("musl") {
        cmd.arg("--with-jemalloc-prefix=je_");
    }


    if env::var_os("CARGO_FEATURE_DEBUG").is_some() {
        cmd.arg("--enable-debug");
    }

    if env::var_os("CARGO_FEATURE_PROFILING").is_some() {
        cmd.arg("--enable-prof");
    }
    cmd.arg(format!("--host={}", gnu_target(&target)));
    cmd.arg(format!("--build={}", gnu_target(&host)));
    cmd.arg(format!("--prefix={}", out_dir.display()));

    run(&mut cmd);

    let make = if host.contains("bitrig") || host.contains("dragonfly") ||
        host.contains("freebsd") || host.contains("netbsd") ||
        host.contains("openbsd") {
        "gmake"
    } else {
        "make"
    };

    run(Command::new(make)
                .current_dir(&build_dir)
                .arg("install_lib_static")
                .arg("install_include")
                .arg("-j").arg(env::var("NUM_JOBS").unwrap()));

    println!("cargo:root={}", out_dir.display());

    // Linkage directives to pull in jemalloc and its dependencies.
    //
    // On some platforms we need to be sure to link in `pthread` which jemalloc
    // depends on, and specifically on android we need to also link to libgcc.
    // Currently jemalloc is compiled with gcc which will generate calls to
    // intrinsics that are libgcc specific (e.g. those intrinsics aren't present in
    // libcompiler-rt), so link that in to get that support.
    if target.contains("windows") {
        println!("cargo:rustc-link-lib=static=jemalloc");
    } else {
        println!("cargo:rustc-link-lib=static=jemalloc_pic");
    }
    println!("cargo:rustc-link-search=native={}/lib", build_dir.display());
    if target.contains("android") {
        println!("cargo:rustc-link-lib=gcc");
    } else if !target.contains("windows") {
        println!("cargo:rustc-link-lib=pthread");
    }
}

fn run(cmd: &mut Command) {
    println!("running: {:?}", cmd);
    let status = match cmd.status() {
        Ok(status) => status,
        Err(e) => panic!("failed to execute command: {}", e),
    };
    if !status.success() {
        panic!("command did not execute successfully: {:?}\n\
                expected success, got: {}", cmd, status);
    }
}
