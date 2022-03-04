use std::env;

fn main() {
    let target = env::var("TARGET").unwrap();
    let out_dir = env::var("OUT_DIR").unwrap();

    println!("cargo:rerun-if-changed=src_c/");
    let mut config = cc::Build::new();
    config.file("src_c/th.cc");
    config.compile("libth.a");
    println!("cargo:rustc-link-search=native={}", out_dir);
    println!("cargo:rustc-link-lib=static=th");
    println!("cargo:rerun-if-changed=src_c/th.c");
    // according to https://github.com/alexcrichton/cc-rs/blob/master/src/lib.rs#L2189
    if target.contains("apple") || target.contains("freebsd") || target.contains("openbsd") {
        println!("cargo:rustc-link-lib=dylib=c++");
    } else if target.contains("linux") {
        println!("cargo:rustc-link-lib=dylib=stdc++");
    }
}
