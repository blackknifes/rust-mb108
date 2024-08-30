use self::error::Result;
use mb108::{common::Rect, run, webview};
use mb108_sys::mbWake;
use std::time::Duration;

use super::*;

#[cfg(target_os = "windows")]
#[cfg(target_arch = "x86")]
const DLL: &str = "mb/bin/win_x32/mb108_x32.dll";

#[cfg(target_os = "windows")]
#[cfg(target_arch = "x86_64")]
const DLL: &str = "mb/bin/win_x64/mb108_x64.dll";

#[cfg(not(target_os = "windows"))]
const DLL: &str = "mb/bin/linux_x64/miniblink.so";

fn get_dll_path() -> String {
    std::env::current_dir()
        .expect("cannot get current dir")
        .join("mb108-sys")
        .join(DLL)
        .to_str()
        .expect("cannot get dll path")
        .to_owned()
}

#[test]
fn test_popup() -> std::result::Result<(), Box<dyn std::error::Error>> {
    main()
}

#[mb108::main(dll = get_dll_path)]
async fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    // println!("{}", mb108::report()?);
    mb108::exit();

    // let webview = webview::WebView::popup(Rect {
    //     x: 0,
    //     y: 0,
    //     width: 800,
    //     height: 600,
    // });
    // webview
    //     .load_url("https://baidu.com")
    //     .expect("cannot load url");
    // webview.show();
    Ok(())
}
