// Copyright 2015 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

extern crate gcc;

use std::env;
use std::fs;
use std::path::PathBuf;
use std::process::Command;

fn main() {
    let target = env::var("TARGET").unwrap();
    let host = env::var("HOST").unwrap();
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

    let compiler = gcc::Config::new().get_compiler();
    let cflags = compiler.args().iter().map(|s| s.to_str().unwrap())
                         .collect::<Vec<_>>().join(" ");

    let mut cmd = Command::new("sh");
    cmd.arg(src_dir.join("jemalloc/configure").to_str().unwrap()
                   .replace("C:\\", "/c/")
                   .replace("\\", "/"))
       .current_dir(&build_dir)
       .env("CC", compiler.path())
       .env("CFLAGS", cflags);

    if target.contains("ios") {
        cmd.arg("--disable-tls");
    } else if target.contains("android") {
        cmd.arg("--disable-tls");
    }

    cmd.arg("--with-jemalloc-prefix=_rjem_");

    if cfg!(feature = "debug-jemalloc") {
        cmd.arg("--enable-debug");
    }

    if env::var_os("CARGO_FEATURE_PROFILING").is_some() {
        cmd.arg("--enable-prof");
    }
    cmd.arg(format!("--host={}", target.replace("windows-gnu", "w64-mingw32")));
    cmd.arg(format!("--build={}", host.replace("windows-gnu", "w64-mingw32")));
    cmd.arg(format!("--prefix={}", out_dir.display()));

    run(&mut cmd);
    run(Command::new("make")
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
