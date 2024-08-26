use bindgen;
use std::{env, path::PathBuf};

const MB_HEADER: &str = "mb/include/mb.h";
const MB_WIN32_HEADER: &str = "mb/include/win32.h";
const MB_SOURCE: &str = "mb/src/mb.c";

const MB_LIB_NAME: &str = "miniblink";

fn main() {
    println!("cargo:rerun-if-changed={}", MB_WIN32_HEADER);
    println!("cargo:rerun-if-changed={}", MB_HEADER);
    println!("cargo:rerun-if-changed={}", MB_SOURCE);

    let cur_dir = std::env::current_dir().expect("cannot get current dir");
    let out_path = PathBuf::from(env::var("OUT_DIR").expect("cannot get env $OUT_DIR"));
    println!("cargo:warning={}", cur_dir.join("mb").join("include").to_str().unwrap());
    println!("cargo:warning={}", cur_dir.join(MB_SOURCE).to_str().unwrap());
    println!("cargo:warning={}", out_path.join(MB_LIB_NAME).to_str().unwrap());

    cc::Build::new()
        .include(cur_dir.join("mb").join("include"))
        .file(cur_dir.join(MB_SOURCE))
        .compile(MB_LIB_NAME);

    println!(
        "cargo:rustc-link-search=native={}",
        out_path.to_str().unwrap()
    );
    println!("cargo:rustc-link-lib={}", MB_LIB_NAME);

    bindgen::Builder::default()
        .header(MB_HEADER)
        .allowlist_file(MB_HEADER)
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()
        .expect("Unable to generate bindings")
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
