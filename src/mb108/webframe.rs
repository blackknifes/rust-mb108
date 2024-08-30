use crate::error::Result;
use crate::utils::{from_bool_int, to_bool_int, to_cstr_ptr};
use mb108_sys::{
    mbGetGlobalExecByFrame, mbInsertCSSByFrame, mbIsMainFrame, mbPrintSettings, mbUtilPrint,
    mbUtilsSilentPrint, mbWebFrameHandle, mbWebView,
};

use super::javascript::{ExecState, JsValue};

#[derive(Clone, Copy)]
pub struct PrintSettings {
    pub dpi: i32,
    pub width: u32,
    pub height: u32,
    pub margin_top: i32,
    pub margin_bottom: i32,
    pub margin_left: i32,
    pub margin_right: i32,
    pub is_print_page_head_and_footer: bool,
    pub is_print_backgroud: bool,
    pub is_landscape: bool,
    pub is_print_to_multi_page: bool,
}

impl PrintSettings {
    pub fn new_4k() -> Self {
        Self {
            dpi: 300,                            // 高质量打印常用的 DPI
            width: 4096,                         // 4K 纸张宽度（像素）
            height: 2160,                        // 4K 纸张高度（像素）
            margin_top: 50,                      // 50 像素上边距
            margin_bottom: 50,                   // 50 像素下边距
            margin_left: 50,                     // 50 像素左边距
            margin_right: 50,                    // 50 像素右边距
            is_print_page_head_and_footer: true, // 打印页眉和页脚
            is_print_backgroud: false,           // 不打印背景
            is_landscape: false,                 // 纵向打印
            is_print_to_multi_page: false,       // 不打印成多页
        }
    }

    pub(crate) fn into_mb(&self) -> mbPrintSettings {
        mbPrintSettings {
            structSize: std::mem::size_of::<mbPrintSettings>() as i32,
            dpi: self.dpi,
            width: self.width as i32,
            height: self.height as i32,
            marginTop: self.margin_top,
            marginBottom: self.margin_bottom,
            marginLeft: self.margin_left,
            marginRight: self.margin_right,
            isPrintPageHeadAndFooter: to_bool_int(self.is_print_page_head_and_footer),
            isPrintBackgroud: to_bool_int(self.is_print_backgroud),
            isLandscape: to_bool_int(self.is_landscape),
            isPrintToMultiPage: to_bool_int(self.is_print_to_multi_page),
        }
    }
}

impl std::default::Default for PrintSettings {
    fn default() -> Self {
        Self::new_4k()
    }
}

pub struct WebFrame {
    pub(crate) webview: mbWebView,
    pub(crate) frame: mbWebFrameHandle,
}

impl WebFrame {
    pub(crate) fn from_mb(webview: mbWebView, frame: mbWebFrameHandle) -> Self {
        Self { webview, frame }
    }

    pub async fn run_js(&self, script: &str, is_in_closure: bool) -> JsValue {
        todo!()
    }

    pub fn is_main(&self) -> bool {
        unsafe { from_bool_int(mbIsMainFrame.unwrap()(self.webview, self.frame)) }
    }

    pub fn print(&self, settings: PrintSettings) -> bool {
        unsafe {
            let settings = settings.into_mb();
            from_bool_int(mbUtilPrint.unwrap()(self.webview, self.frame, &settings))
        }
    }

    // pub async fn print_to_pdf(&self, settings: PrintSettings) -> Vec<Vec<u8>> {
    //     todo!()
    // }

    // pub async fn print_to_bitmap(&self, size: Size) -> Vec<u8>
    // {
    //     todo!()
    // }

    // pub async fn popup_dialog_and_download()
    // {
    //     todo!()
    // }

    // pub async fn get_pdf_page_data(&self) -> Vec<u8> {}

    pub fn get_exec_state(&self) -> ExecState {
        unsafe { ExecState::from_mb(mbGetGlobalExecByFrame.unwrap()(self.webview, self.frame)) }
    }

    pub fn insert_css_by_frame(&self, css: &str) -> Result<()> {
        unsafe {
            mbInsertCSSByFrame.unwrap()(self.webview, self.frame, to_cstr_ptr(css)?.to_utf8());
            Ok(())
        }
    }
}
