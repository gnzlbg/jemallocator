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
extern crate fs_extra;

use std::env;
use std::fs;
use std::fs::File;
use std::path::{Path, PathBuf};
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
    let num_jobs = env::var("NUM_JOBS").expect("NUM_JOBS was not set");
    let out_dir = PathBuf::from(env::var_os("OUT_DIR").expect("OUT_DIR was not set"));
    println!("TARGET={}", target.clone());
    println!("HOST={}", host.clone());
    println!("NUM_JOBS={}", num_jobs.clone());
    println!("OUT_DIR={:?}", out_dir);
    let build_dir = out_dir.join("build");
    println!("BUILD_DIR={:?}", build_dir);
    let src_dir = env::current_dir().expect("failed to get current directory");
    println!("SRC_DIR={:?}", src_dir);

    let unsupported_targets = [
        "rumprun",
        "bitrig",
        "openbsd",
        "msvc",
        "emscripten",
        "fuchsia",
        "redox",
        "wasm32",
    ];
    for i in &unsupported_targets {
        if target.contains(i) {
            panic!("jemalloc does not support target: {}", target);
        }
    }

    let mut enable_background_threads = env::var("CARGO_FEATURE_BACKGROUND_THREADS").is_ok();
    let mut enable_background_threads_runtime_support =
        env::var("CARGO_FEATURE_BACKGROUND_THREADS_RUNTIME_SUPPORT").is_ok();

    if enable_background_threads_runtime_support && target.contains("musl") {
        println!(
            "cargo:warning=\"`background_threads_runtime_support` not supported for `{}`\"",
            target
        );
        enable_background_threads_runtime_support = false;
        enable_background_threads = false;
    }

    if enable_background_threads {
        assert!(
            enable_background_threads_runtime_support,
            "enabling `background_threads` requires `background_threads_runtime_support`"
        );
    }

    if let Some(jemalloc) = env::var_os("JEMALLOC_OVERRIDE") {
        println!("jemalloc override set");
        let jemalloc = PathBuf::from(jemalloc);
        println!(
            "cargo:rustc-link-search=native={}",
            jemalloc.parent().unwrap().display()
        );
        let stem = jemalloc.file_stem().unwrap().to_str().unwrap();
        let name = jemalloc.file_name().unwrap().to_str().unwrap();
        let kind = if name.ends_with(".a") {
            "static"
        } else {
            "dylib"
        };
        println!("cargo:rustc-link-lib={}={}", kind, &stem[3..]);
        return;
    }

    fs::create_dir_all(&build_dir).unwrap();
    // Disable -Wextra warnings - jemalloc doesn't compile free of warnings with
    // it enabled: https://github.com/jemalloc/jemalloc/issues/1196
    let compiler = cc::Build::new().extra_warnings(false).get_compiler();
    let cflags = compiler
        .args()
        .iter()
        .map(|s| s.to_str().unwrap())
        .collect::<Vec<_>>()
        .join(" ");
    println!("CC={:?}", compiler.path());
    println!("CFLAGS={:?}", cflags);

    let jemalloc_src_dir = out_dir.join("jemalloc");
    println!("JEMALLOC_SRC_DIR={:?}", jemalloc_src_dir);

    if jemalloc_src_dir.exists() {
        fs::remove_dir_all(jemalloc_src_dir.clone()).unwrap();
    }

    // Copy jemalloc submodule to the OUT_DIR
    assert!(out_dir.exists(), "OUT_DIR does not exist");
    let mut copy_options = fs_extra::dir::CopyOptions::new();
    copy_options.overwrite = true;
    copy_options.copy_inside = true;
    fs_extra::dir::copy(
        Path::new("jemalloc"),
        jemalloc_src_dir.clone(),
        &copy_options,
    )
    .expect("failed to copy jemalloc source code to OUT_DIR");

    // Configuration files
    let config_files = ["configure", "VERSION"];

    // Verify that the configuration files are up-to-date
    if env::var_os("JEMALLOC_SYS_VERIFY_CONFIGURE").is_some() {
        assert!(!jemalloc_src_dir.join("configure").exists(),
                "the jemalloc source directory cannot contain configuration files like 'configure' and 'VERSION'");
        // Run autoconf:
        let mut cmd = Command::new("autoconf");
        cmd.current_dir(jemalloc_src_dir.clone());
        run(&mut cmd);

        for f in &config_files {
            use std::io::Read;
            let mut file = File::open(jemalloc_src_dir.join(f)).expect("file not found");
            let mut source_contents = String::new();
            file.read_to_string(&mut source_contents)
                .expect("failed to read file");
            let mut file =
                File::open(Path::new(&format!("configure/{}", f))).expect("file not found");
            let mut reference_contents = String::new();
            file.read_to_string(&mut reference_contents)
                .expect("failed to read file");
            if source_contents != reference_contents {
                panic!("the file \"{}\" differs from the jemalloc source and the reference in \"jemalloc-sys/configure/{}\"", jemalloc_src_dir.join(f).display(), f);
            }
        }
    } else {
        // Copy the configuration files to jemalloc's source directory
        for f in &config_files {
            fs::copy(
                Path::new(&format!("configure/{}", f)),
                jemalloc_src_dir.join(f),
            )
            .expect("failed to copy config file to OUT_DIR");
        }
    }

    // Run configure:
    let configure = jemalloc_src_dir.join("configure");
    let mut cmd = Command::new("sh");
    cmd.arg(
        configure
            .to_str()
            .unwrap()
            .replace("C:\\", "/c/")
            .replace("\\", "/"),
    )
    .current_dir(&build_dir)
    .env("CC", compiler.path())
    .env("CFLAGS", cflags.clone())
    .env("LDFLAGS", cflags.clone())
    .env("CPPFLAGS", cflags.clone())
    .arg("--disable-cxx");

    if target.contains("ios") {
        // newer iOS deviced have 16kb page sizes:
        // closed: https://github.com/alexcrichton/jemallocator/issues/68
        cmd.arg("--with-lg-page=14");
    }

    // collect malloc_conf string:
    let mut malloc_conf = String::new();

    if enable_background_threads_runtime_support {
        // jemalloc is compiled with background thread run-time support on
        // available platforms by default

        if enable_background_threads {
            // Background thread support does not enable background threads at
            // run-time, just support for enabling them via run-time configuration
            // options (they are disabled by default)

            // The `enable_background_threads` cargo feature forces background
            // threads to be enabled at run-time by default:
            malloc_conf += "background_thread:true";
        }
    } else {
        assert!(
            !enable_background_threads,
            "`background_threads` enabled but run-time support is disabled"
        );
        // Background thread run-time support is disabled by
        // disabling background threads at compile-time:
        malloc_conf += "background_thread:false";
    }

    if let Ok(malloc_conf_opts) = env::var("JEMALLOC_SYS_WITH_MALLOC_CONF") {
        if malloc_conf.is_empty() {
            malloc_conf += &malloc_conf_opts;
        } else {
            malloc_conf += &format!(",{}", malloc_conf_opts);
        }
    }

    if !malloc_conf.is_empty() {
        cmd.arg(format!("--with-malloc-conf={}", malloc_conf));
    }

    if let Ok(lg_page) = env::var("JEMALLOC_SYS_WITH_LG_PAGE") {
        cmd.arg(format!("--with-lg-page={}", lg_page));
    }

    if let Ok(lg_hugepage) = env::var("JEMALLOC_SYS_WITH_LG_HUGEPAGE") {
        cmd.arg(format!("--with-lg-hugepage={}", lg_hugepage));
    }

    if let Ok(lg_quantum) = env::var("JEMALLOC_SYS_WITH_LG_QUANTUM") {
        cmd.arg(format!("--with-lg-quantum={}", lg_quantum));
    }

    if let Ok(lg_vaddr) = env::var("JEMALLOC_SYS_WITH_LG_VADDR") {
        cmd.arg(format!("--with-lg-vaddr={}", lg_vaddr));
    }

    let mut use_prefix =
        env::var_os("CARGO_FEATURE_UNPREFIXED_MALLOC_ON_SUPPORTED_PLATFORMS").is_none();

    // “it was found that the `realpath` function in libc would allocate with libc malloc
    //  (not jemalloc malloc), and then the standard library would free with jemalloc free,
    //  causing a segfault.”
    // https://github.com/rust-lang/rust/commit/e3b414d8612314e74e2b0ebde1ed5c6997d28e8d
    // https://github.com/rust-lang/rust/commit/536011d929ecbd1170baf34e09580e567c971f95
    // https://github.com/rust-lang/rust/commit/9f3de647326fbe50e0e283b9018ab7c41abccde3
    // https://github.com/rust-lang/rust/commit/ed015456a114ae907a36af80c06f81ea93182a24
    if !use_prefix
        && (target.contains("android")
            || target.contains("dragonfly")
            || target.contains("musl")
            || target.contains("darwin"))
    {
        println!("cargo:warning=\"Unprefixed malloc() requested on unsupported platform\"");
        use_prefix = true;
    }

    if use_prefix {
        cmd.arg("--with-jemalloc-prefix=_rjem_");
        println!("cargo:rustc-cfg=prefixed");
        println!("JEMALLOC PREFIX SET TO: _rjem_");
    }

    cmd.arg("--with-private-namespace=_rjem_");

    if env::var_os("CARGO_FEATURE_DEBUG").is_some() {
        println!("CARGO_FEATURE_DEBUG set");
        cmd.arg("--enable-debug");
    }

    if env::var_os("CARGO_FEATURE_PROFILING").is_some() {
        println!("CARGO_FEATURE_PROFILING set");
        cmd.arg("--enable-prof");
    }

    if env::var_os("CARGO_FEATURE_STATS").is_some() {
        println!("CARGO_FEATURE_STATS set");
        cmd.arg("--enable-stats");
    }

    cmd.arg(format!("--host={}", gnu_target(&target)));
    cmd.arg(format!("--build={}", gnu_target(&host)));
    cmd.arg(format!("--prefix={}", out_dir.display()));

    run(&mut cmd);

    let make = if host.contains("bitrig")
        || host.contains("dragonfly")
        || host.contains("freebsd")
        || host.contains("netbsd")
        || host.contains("openbsd")
    {
        "gmake"
    } else {
        "make"
    };

    // Make:
    run(Command::new(make)
        .current_dir(&build_dir)
        .arg("-j")
        .arg(num_jobs.clone()));

    if env::var_os("JEMALLOC_SYS_RUN_JEMALLOC_TESTS").is_some() {
        println!("JEMALLOC_SYS_RUN_JEMALLOC_TESTS set: building and running jemalloc tests...");
        // Make tests:
        run(Command::new(make)
            .current_dir(&build_dir)
            .arg("-j")
            .arg(num_jobs.clone())
            .arg("tests"));

        // Run tests:
        run(Command::new(make).current_dir(&build_dir).arg("check"));
    }

    // Make install:
    run(Command::new(make)
        .current_dir(&build_dir)
        .arg("install_lib_static")
        .arg("install_include")
        .arg("-j")
        .arg(num_jobs.clone()));

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
    println!("cargo:rerun-if-changed=jemalloc");
}

fn run(cmd: &mut Command) {
    println!("running: {:?}", cmd);
    let status = match cmd.status() {
        Ok(status) => status,
        Err(e) => panic!("failed to execute command: {}", e),
    };
    if !status.success() {
        panic!(
            "command did not execute successfully: {:?}\n\
             expected success, got: {}",
            cmd, status
        );
    }
}
