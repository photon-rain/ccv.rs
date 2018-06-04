extern crate pkg_config;

use std::process::Command;
use std::env;
use std::path::Path;

use std::fs::File;
use std::io::Write;

fn main() {
    let cur_dir = env::var("CARGO_MANIFEST_DIR").unwrap();

    let mut status = File::create("/tmp/cargo.log").unwrap();
    status.write_fmt(format_args!("Starting\n")).unwrap();

    let lib_dir = Path::new(&cur_dir)
        .join("ccv")
        .join("lib");
    status.write_fmt(format_args!("lib: {:?}\n", lib_dir)).unwrap();

    let configure_cmd = lib_dir.join("configure");
    status.write_fmt(format_args!("configure: {:?}\n", configure_cmd)).unwrap();
    Command::new(configure_cmd)
        .args(&["CFLAGS=-fPIC","LIBS=jpeg"])
        .current_dir(lib_dir.clone())
        .status()
        .expect("Error in lib/configure");

    let make_cmd = "make";
    status.write_fmt(format_args!("make: {:?}\n", make_cmd)).unwrap();

    Command::new(make_cmd)
        .args(&["-C", lib_dir.to_str().unwrap()])
        .current_dir(lib_dir.clone())
        .status()
        .expect("Error in make");

    println!("cargo:rustc-link-search=native={}", lib_dir.to_str().unwrap());
    println!("cargo:rustc-link-lib=static=ccv");
    //println!("cargo:rustc-link-lib=ccv");

    println!("cargo:rustc-link-lib=jpeg");
    println!("cargo:rustc-link-lib=m");
    println!("cargo:rustc-link-lib=png16");


}