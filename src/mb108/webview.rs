use super::{common::Rect, Proxy};
use crate::error::{Error, Result};
use crate::utils::{from_bool_int, to_bool_int, to_cstr16_ptr, to_cstr_ptr};
use mb108_sys::{
    _mbMenuItemId_kMbMenuCopyImageId, _mbMenuItemId_kMbMenuCutId, _mbMenuItemId_kMbMenuGoBackId,
    _mbMenuItemId_kMbMenuGoForwardId, _mbMenuItemId_kMbMenuInspectElementAtId,
    _mbMenuItemId_kMbMenuPasteId, _mbMenuItemId_kMbMenuPrintId, _mbMenuItemId_kMbMenuReloadId,
    _mbMenuItemId_kMbMenuSelectedAllId, _mbMenuItemId_kMbMenuSelectedTextId,
    _mbMenuItemId_kMbMenuUndoId, _mbWindowType_MB_WINDOW_TYPE_POPUP, mbAddPluginDirectory,
    mbCreateWebWindow, mbGetCaretRect, mbGetCursorInfoType, mbGetHostHWND,
    mbGetPlatformWindowHandle, mbGetTitle, mbGetUrl, mbGetZoomFactor, mbIsAudioMuted,
    mbLoadHtmlWithBaseUrl, mbLoadURL, mbMoveToCenter, mbMoveWindow, mbRect, mbSetAudioMuted,
    mbSetContextMenuEnabled, mbSetContextMenuItemShow, mbSetCookie, mbSetCookieEnabled,
    mbSetCookieJarFullPath, mbSetCookieJarPath, mbSetCspCheckEnable, mbSetDebugConfig,
    mbSetDiskCacheEnabled, mbSetDiskCacheLevel, mbSetDiskCacheLimit, mbSetDiskCacheLimitDisk,
    mbSetDiskCachePath, mbSetDragDropEnable, mbSetDragEnable, mbSetHandle, mbSetHandleOffset,
    mbSetHeadlessEnabled, mbSetLocalStorageFullPath, mbSetMemoryCacheEnable, mbSetMouseEnabled,
    mbSetNavigationToNewWindowEnable, mbSetNpapiPluginsEnabled, mbSetProxy, mbSetResourceGc,
    mbSetSystemTouchEnabled, mbSetTouchEnabled, mbSetTransparent, mbSetUserAgent, mbSetWindowTitle,
    mbSetZoomFactor, mbShowWindow, mbWebView, HWND,
};
use std::ffi::c_void;
use std::{ffi::CStr, ptr::null_mut};

pub enum DebugConfig {
    ///开启开发者工具，此时param要填写开发者工具的资源路径，如file:///c:/miniblink-release/front_end/inspector.html。注意param此时必须是utf8编码
    ShowDevTools(String),
    ///设置帧率，默认值是10，值越大帧率越低
    WakeMinInterval(u32),
    ///设置帧率，默认值是3，值越大帧率越低
    DrawMinInterval(u32),
    ///设置抗锯齿渲染。param必须设置为"1"
    AntiAlias(bool),
    ///最小字体
    MinimumFontSize(u32),
    ///最小逻辑字体
    MinimumLogicalFontSize(u32),
    ///默认字体
    DefaultFontSize(u32),
    ///默认fixed字体
    DefaultFixedFontSize(u32),
}

impl DebugConfig {
    pub(crate) fn get_native_params(&self) -> (String, String) {
        match self {
            DebugConfig::ShowDevTools(path) => ("showDevTools".to_owned(), path.clone()),
            DebugConfig::WakeMinInterval(param) => {
                ("wakeMinInterval".to_owned(), param.to_string())
            }
            DebugConfig::DrawMinInterval(param) => {
                ("drawMinInterval".to_owned(), param.to_string())
            }
            DebugConfig::AntiAlias(param) => ("antiAlias".to_owned(), param.to_string()),
            DebugConfig::MinimumFontSize(param) => {
                ("minimumFontSize".to_owned(), param.to_string())
            }
            DebugConfig::MinimumLogicalFontSize(param) => {
                ("minimumLogicalFontSize".to_owned(), param.to_string())
            }
            DebugConfig::DefaultFontSize(param) => {
                ("defaultFontSize".to_owned(), param.to_string())
            }
            DebugConfig::DefaultFixedFontSize(param) => {
                ("defaultFixedFontSize".to_owned(), param.to_string())
            }
        }
    }
}

#[repr(i32)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum MenuItemId {
    MenuSelectedAllId = _mbMenuItemId_kMbMenuSelectedAllId,
    MenuSelectedTextId = _mbMenuItemId_kMbMenuSelectedTextId,
    MenuUndoId = _mbMenuItemId_kMbMenuUndoId,
    MenuCopyImageId = _mbMenuItemId_kMbMenuCopyImageId,
    MenuInspectElementAtId = _mbMenuItemId_kMbMenuInspectElementAtId,
    MenuCutId = _mbMenuItemId_kMbMenuCutId,
    MenuPasteId = _mbMenuItemId_kMbMenuPasteId,
    MenuPrintId = _mbMenuItemId_kMbMenuPrintId,
    MenuGoForwardId = _mbMenuItemId_kMbMenuGoForwardId,
    MenuGoBackId = _mbMenuItemId_kMbMenuGoBackId,
    MenuReloadId = _mbMenuItemId_kMbMenuReloadId,
}

pub struct WebView {
    webview: mbWebView,
}

impl WebView {
    pub fn popup(rc: Rect) -> Self {
        unsafe {
            let webview = mbCreateWebWindow.unwrap()(
                _mbWindowType_MB_WINDOW_TYPE_POPUP,
                null_mut(),
                rc.x,
                rc.y,
                rc.width,
                rc.height,
            );
            return WebView { webview };
        }
    }

    pub fn load_url(&self, url: &str) -> Result<()> {
        unsafe {
            mbLoadURL.unwrap()(self.webview, to_cstr_ptr(url)?);
            Ok(())
        }
    }

    pub fn load_html_with_base_url(&self, html: &str, base_url: &str) -> Result<()> {
        unsafe {
            mbLoadHtmlWithBaseUrl.unwrap()(
                self.webview,
                to_cstr_ptr(html)?,
                to_cstr_ptr(base_url)?,
            );
            Ok(())
        }
    }

    pub fn show(&self) {
        unsafe {
            mbShowWindow.unwrap()(self.webview, 1);
        }
    }

    pub fn move_to(&self, rc: Rect) {
        unsafe {
            mbMoveWindow.unwrap()(self.webview, rc.x, rc.y, rc.width, rc.height);
        }
    }

    pub fn move_to_center(&self) {
        unsafe {
            mbMoveToCenter.unwrap()(self.webview);
        }
    }

    pub fn get_caret_rect(&self) -> Rect {
        unsafe {
            let mut rc = mbRect {
                x: 0,
                y: 0,
                w: 0,
                h: 0,
            };
            mbGetCaretRect.unwrap()(self.webview, &mut rc);
            return Rect::from_mb(&rc);
        }
    }

    pub fn set_audio_muted(&self, mute: bool) {
        unsafe {
            mbSetAudioMuted.unwrap()(self.webview, to_bool_int(mute));
        }
    }

    pub fn is_audio_muted(&self) -> bool {
        unsafe {
            return from_bool_int(mbIsAudioMuted.unwrap()(self.webview));
        }
    }

    pub fn set_proxy(&self, proxy: Proxy) -> crate::error::Result<()> {
        unsafe {
            let mb_proxy = proxy.into_native()?;
            mbSetProxy.unwrap()(self.webview, &mb_proxy);
            Ok(())
        }
    }

    pub fn set_debug_config(&self, config: DebugConfig) -> Result<()> {
        unsafe {
            let (debug_str, param) = config.get_native_params();
            mbSetDebugConfig.unwrap()(self.webview, to_cstr_ptr(&debug_str)?, to_cstr_ptr(&param)?);
            Ok(())
        }
    }

    pub fn set_mouse_enabled(&self, enable: bool) {
        unsafe {
            mbSetMouseEnabled.unwrap()(self.webview, to_bool_int(enable));
        }
    }

    pub fn set_touch_enabled(&self, enable: bool) {
        unsafe {
            mbSetTouchEnabled.unwrap()(self.webview, to_bool_int(enable));
        }
    }

    pub fn set_system_touch_enabled(&self, enable: bool) {
        unsafe {
            mbSetSystemTouchEnabled.unwrap()(self.webview, to_bool_int(enable));
        }
    }

    pub fn set_context_menu_enabled(&self, enable: bool) {
        unsafe {
            mbSetContextMenuEnabled.unwrap()(self.webview, to_bool_int(enable));
        }
    }

    pub fn set_navigation_to_new_window_enabled(&self, enable: bool) {
        unsafe {
            mbSetNavigationToNewWindowEnable.unwrap()(self.webview, to_bool_int(enable));
        }
    }

    pub fn set_headless_enabled(&self, enable: bool) {
        unsafe {
            mbSetHeadlessEnabled.unwrap()(self.webview, to_bool_int(enable));
        }
    }

    pub fn set_drag_drop_enabled(&self, enable: bool) {
        unsafe {
            mbSetDragDropEnable.unwrap()(self.webview, to_bool_int(enable));
        }
    }

    pub fn set_drag_enabled(&self, enable: bool) {
        unsafe {
            mbSetDragEnable.unwrap()(self.webview, to_bool_int(enable));
        }
    }

    pub fn set_context_menu_item_show(&self, menu_item_id: MenuItemId, show: bool) {
        unsafe {
            mbSetContextMenuItemShow.unwrap()(self.webview, menu_item_id as i32, to_bool_int(show));
        }
    }

    pub fn set_handle(&self, hwnd: HWND) {
        unsafe {
            mbSetHandle.unwrap()(self.webview, hwnd);
        }
    }

    pub fn set_handle_offset(&self, x: i32, y: i32) {
        unsafe {
            mbSetHandleOffset.unwrap()(self.webview, x, y);
        }
    }

    pub fn get_platform_window_handle(&self) -> *mut c_void {
        unsafe {
            return mbGetPlatformWindowHandle.unwrap()(self.webview);
        }
    }

    pub fn get_host_hwnd(&self) -> HWND {
        unsafe {
            return mbGetHostHWND.unwrap()(self.webview);
        }
    }

    pub fn set_transparent(&self, transparent: bool) {
        unsafe {
            mbSetTransparent.unwrap()(self.webview, to_bool_int(transparent));
        }
    }

    pub fn set_csp_check_enabled(&self, enable: bool) {
        unsafe {
            mbSetCspCheckEnable.unwrap()(self.webview, to_bool_int(enable));
        }
    }

    pub fn set_npapi_plugins_enabled(&self, enable: bool) {
        unsafe {
            mbSetNpapiPluginsEnabled.unwrap()(self.webview, to_bool_int(enable));
        }
    }

    pub fn set_memory_cache_enabled(&self, enable: bool) {
        unsafe {
            mbSetMemoryCacheEnable.unwrap()(self.webview, to_bool_int(enable));
        }
    }

    pub fn set_cookie_enabled(&self, enable: bool) {
        unsafe {
            mbSetCookieEnabled.unwrap()(self.webview, to_bool_int(enable));
        }
    }

    pub fn set_cookie(&self, url: &str, cookie: &str) -> Result<()> {
        unsafe {
            mbSetCookie.unwrap()(self.webview, to_cstr_ptr(url)?, to_cstr_ptr(cookie)?);
            Ok(())
        }
    }

    pub fn set_cookie_jar_path(&self, path: &str) {
        unsafe {
            mbSetCookieJarPath.unwrap()(self.webview, (&to_cstr16_ptr(path)).as_ptr());
        }
    }

    pub fn set_cookie_jar_full_path(&self, path: &str) {
        unsafe {
            mbSetCookieJarFullPath.unwrap()(self.webview, (&to_cstr16_ptr(path)).as_ptr());
        }
    }

    pub fn set_local_storage_full_path(&self, path: &str) {
        unsafe {
            mbSetLocalStorageFullPath.unwrap()(self.webview, (&to_cstr16_ptr(path)).as_ptr());
        }
    }

    pub fn get_title(&self) -> Result<String> {
        unsafe {
            let title = CStr::from_ptr(mbGetTitle.unwrap()(self.webview))
                .to_str()
                .map_err(Error::other)?
                .to_owned();
            Ok(title)
        }
    }

    pub fn set_title(&self, title: &str) -> Result<()> {
        unsafe {
            mbSetWindowTitle.unwrap()(self.webview, to_cstr_ptr(title)?);
            Ok(())
        }
    }

    pub fn get_url(&self) -> Result<String> {
        unsafe {
            let title = CStr::from_ptr(mbGetUrl.unwrap()(self.webview))
                .to_str()
                .map_err(Error::other)?
                .to_owned();
            Ok(title)
        }
    }

    pub fn get_cursor_info_type(&self) -> i32 {
        unsafe {
            return mbGetCursorInfoType.unwrap()(self.webview);
        }
    }

    pub fn add_plugin_directory(&self, path: &str) {
        unsafe {
            mbAddPluginDirectory.unwrap()(self.webview, (&to_cstr16_ptr(path)).as_ptr());
        }
    }

    pub fn set_user_agent(&self, user_agent: &str) -> Result<()> {
        unsafe {
            mbSetUserAgent.unwrap()(self.webview, to_cstr_ptr(user_agent)?);
            Ok(())
        }
    }

    pub fn set_zoom_factor(&self, factor: f32) {
        unsafe {
            mbSetZoomFactor.unwrap()(self.webview, factor);
        }
    }

    pub fn get_zoom_factor(&self) -> f32 {
        unsafe { mbGetZoomFactor.unwrap()(self.webview) }
    }

    pub fn set_disk_cache_enabled(&self, enable: bool) {
        unsafe {
            mbSetDiskCacheEnabled.unwrap()(self.webview, to_bool_int(enable));
        }
    }

    pub fn set_disk_cache_path(&self, path: &str) {
        unsafe {
            mbSetDiskCachePath.unwrap()(self.webview, (&to_cstr16_ptr(path)).as_ptr());
        }
    }

    pub fn set_disk_cache_limit(&self, limit: usize) {
        unsafe {
            mbSetDiskCacheLimit.unwrap()(self.webview, limit);
        }
    }

    pub fn set_disk_cache_limit_disk(&self, limit: usize) {
        unsafe {
            mbSetDiskCacheLimitDisk.unwrap()(self.webview, limit);
        }
    }

    pub fn set_disk_cache_level(&self, level: i32) {
        unsafe {
            mbSetDiskCacheLevel.unwrap()(self.webview, level);
        }
    }

    pub fn set_resource_gc(&self, interval_seconds: i32) {
        unsafe {
            mbSetResourceGc.unwrap()(self.webview, interval_seconds);
        }
    }
}
