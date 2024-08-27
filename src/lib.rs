pub mod delegate;
pub mod error;
pub mod mb108;

mod utils;

#[cfg(test)]
mod tests {
    use mb108::{common::Rect, run, webview};

    use super::*;

    #[cfg(target_os = "windows")]
    #[cfg(target_arch = "x86")]
    const DLL: &str = "mb/bin/win_x32/mb108_x32.dll";

    #[cfg(target_os = "windows")]
    #[cfg(target_arch = "x86_64")]
    const DLL: &str = "mb/bin/win_x64/mb108_x64.dll";

    #[cfg(not(target_os = "windows"))]
    const DLL: &str = "mb/bin/linux_x64/miniblink.so";

    fn init() {
        let dll = std::env::current_dir()
            .expect("cannot get current dir")
            .join("mb108-sys")
            .join(DLL)
            .to_str()
            .expect("cannot get dll path")
            .to_owned();

        mb108::init(&dll, None).expect("init failed");
    }

    // #[test]
    // fn test_post_task() {
    //     init();

    //     mb108::post_task(mb108::thread::ThreadId::Main, || {
    //         println!("测试");
    //         Ok(())
    //     });

    //     run();
    // }

    #[test]
    fn test_popup() {
        init();
        let webview = webview::WebView::popup(Rect {
            x: 0,
            y: 0,
            width: 800,
            height: 600,
        });
        // webview.load_url("https://baidu.com").expect("cannot load url");
        webview
            .load_html_with_base_url("<html><body>测试</body></html>", "https://baidu.com")
            .expect("cannot load url");
        webview.show();
        run();
    }
}
