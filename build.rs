use std::env;
use std::process::Command;
use std::path::Path;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    println!("the out dir is {}",out_dir);
    Command::new("gcc").args(&["src/hello.c", "-c", "-fPIC", "-o"])
        .arg(&format!("{}/hello.o", out_dir))
        .status().unwrap();
    Command::new("ar").args(&["crus", "libhello.a", "hello.o"])
        .current_dir(&Path::new(&out_dir))
        .status().unwrap();
    println!("cargo:rustc-link-search=native={}", out_dir);
    println!("cargo:rustc-link-lib=static=hello");
    println!("cargo:rerun-if-changed=src/hello.c");
}