use bindgen;
use std::{
    env,
    path::{Path, PathBuf},
};

const MB_HEADER: &str = "mb/include/mb.h";
const MB_WIN32_HEADER: &str = "mb/include/win32.h";
const MB_SOURCE: &str = "mb/src/mb.c";
const MB_NAME: &str = "miniblink";

#[cfg(feature = "enable_report")]
const MB_REPORT_HEADER: &str = "mb/include/mb_report.h";
#[cfg(feature = "enable_report")]
const MB_REPORT_SOURCE: &str = "mb/src/mb_report.c";

fn build_gen(
    dir: impl AsRef<Path>,
    output: impl AsRef<Path>,
    name: &str,
    header: &str,
    source: &str,
    binding_name: &str,
) {
    println!("cargo:rerun-if-changed={}", header);
    println!("cargo:rerun-if-changed={}", source);

    cc::Build::new()
        .include(dir.as_ref().join("mb").join("include"))
        .file(dir.as_ref().join(source))
        .compile(name);

    println!("cargo:rustc-link-lib={}", name);

    bindgen::Builder::default()
        .header(header)
        .allowlist_file(header)
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()
        .expect(&format!("Unable to generate bindings: {}", binding_name))
        .write_to_file(output.as_ref().join(binding_name))
        .expect(&format!("Couldn't write bindings: {}!", binding_name));
}

fn main() {
    //当前目录
    let dir = std::env::current_dir().expect("cannot get current dir");
    //输出目录
    let output = PathBuf::from(env::var("OUT_DIR").expect("cannot get env $OUT_DIR"));
    //添加output目录为库搜索路径
    println!(
        "cargo:rustc-link-search=native={}",
        output.to_str().unwrap()
    );
    //win32头文件修改时重新编译
    println!("cargo:rerun-if-changed={}", MB_WIN32_HEADER);

    //生成bindings
    build_gen(
        &dir,
        &output,
        MB_NAME,
        MB_HEADER,
        MB_SOURCE,
        "bindings.rs",
    );

    //编译并链接mb_report库，生成rust绑定
    #[cfg(feature = "enable_report")]
    {
        //生成report绑定
        build_gen(
            &dir,
            &output,
            "mb_report",
            MB_REPORT_HEADER,
            MB_REPORT_SOURCE,
            "report_bindings.rs",
        );
    }
}
