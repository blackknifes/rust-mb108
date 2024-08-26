use super::common::{Size, UserValue};
use super::webframe::WebFrame;
use super::{common::Rect, Proxy};
use crate::error::{Error, Result};
use crate::utils::{from_bool_int, to_bool_int, to_cstr16_ptr, to_cstr_ptr};
use mb108_sys::{
    _mbMenuItemId_kMbMenuCopyImageId, _mbMenuItemId_kMbMenuCutId, _mbMenuItemId_kMbMenuGoBackId,
    _mbMenuItemId_kMbMenuGoForwardId, _mbMenuItemId_kMbMenuInspectElementAtId,
    _mbMenuItemId_kMbMenuPasteId, _mbMenuItemId_kMbMenuPrintId, _mbMenuItemId_kMbMenuReloadId,
    _mbMenuItemId_kMbMenuSelectedAllId, _mbMenuItemId_kMbMenuSelectedTextId,
    _mbMenuItemId_kMbMenuUndoId, _mbWindowType_MB_WINDOW_TYPE_POPUP, mbAddPluginDirectory,
    mbClearCookie, mbCookieCommand_mbCookieCommandClearAllCookies,
    mbCookieCommand_mbCookieCommandClearSessionCookies,
    mbCookieCommand_mbCookieCommandFlushCookiesToFile,
    mbCookieCommand_mbCookieCommandReloadCookiesFromFile, mbCreateWebWindow,
    mbDefaultPrinterSettings, mbEditorCopy, mbEditorCut, mbEditorDelete, mbEditorPaste,
    mbEditorRedo, mbEditorSelectAll, mbEditorUnSelect, mbEditorUndo, mbGetCaretRect,
    mbGetContentHeight, mbGetContentWidth, mbGetCursorInfoType, mbGetHostHWND, mbGetLockedViewDC,
    mbGetNavigateIndex, mbGetPlatformWindowHandle, mbGetSize, mbGetTitle, mbGetUrl,
    mbGetUserKeyValue, mbGetWebViewForCurrentContext, mbGetZoomFactor, mbGoBack, mbGoForward,
    mbGoToIndex, mbGoToOffset, mbIsAudioMuted, mbIsMainFrame, mbKillFocus, mbLoadHtmlWithBaseUrl,
    mbLoadURL, mbMoveToCenter, mbMoveWindow, mbNavigateAtIndex, mbPerformCookieCommand, mbPostURL,
    mbRect, mbReload, mbResize, mbSetAudioMuted, mbSetContextMenuEnabled, mbSetContextMenuItemShow,
    mbSetCookie, mbSetCookieEnabled, mbSetCookieJarFullPath, mbSetCookieJarPath,
    mbSetCspCheckEnable, mbSetDebugConfig, mbSetDiskCacheEnabled, mbSetDiskCacheLevel,
    mbSetDiskCacheLimit, mbSetDiskCacheLimitDisk, mbSetDiskCachePath, mbSetDragDropEnable,
    mbSetDragEnable, mbSetEditable, mbSetFocus, mbSetHandle, mbSetHandleOffset,
    mbSetHeadlessEnabled, mbSetLocalStorageFullPath, mbSetMemoryCacheEnable, mbSetMouseEnabled,
    mbSetNavigationToNewWindowEnable, mbSetNodeJsEnable, mbSetNpapiPluginsEnabled, mbSetProxy,
    mbSetResourceGc, mbSetSystemTouchEnabled, mbSetTouchEnabled, mbSetTransparent, mbSetUserAgent,
    mbSetUserKeyValue, mbSetWindowTitle, mbSetZoomFactor, mbShowWindow, mbStopLoading,
    mbUnlockViewDC, mbUtilSetDefaultPrinterSettings, mbWake, mbWebFrameGetMainFrame, mbWebView,
    HDC, HWND,
};
use std::ffi::c_void;
use std::sync::Arc;
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

    pub fn from_current_context() -> Option<Self> {
        unsafe {
            let webview = mbGetWebViewForCurrentContext.unwrap()();
            if webview != 0 {
                Some(Self { webview })
            } else {
                None
            }
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

    pub fn can_go_forward(&self) -> bool {
        todo!()
    }

    pub fn can_go_back(&self) -> bool {
        todo!()
    }

    pub fn get_cookie(&self) {
        todo!()
    }

    pub fn clear_cookie(&self) {
        unsafe {
            mbClearCookie.unwrap()(self.webview);
        }
    }

    pub fn resize(&self, width: u32, height: u32) {
        unsafe {
            mbResize.unwrap()(self.webview, width as i32, height as i32);
        }
    }

    pub fn get_size(&self) -> Rect {
        unsafe {
            let mut rc = mbRect {
                x: 0,
                y: 0,
                w: 0,
                h: 0,
            };
            mbGetSize.unwrap()(self.webview, &mut rc);
            Rect::from_mb(&rc)
        }
    }

    pub fn go_back(&self) {
        unsafe {
            mbGoBack.unwrap()(self.webview);
        }
    }

    pub fn go_forward(&self) {
        unsafe {
            mbGoForward.unwrap()(self.webview);
        }
    }

    pub fn navigate_at_index(&self, index: i32) {
        unsafe {
            mbNavigateAtIndex.unwrap()(self.webview, index);
        }
    }

    pub fn get_navigate_index(&self) -> i32 {
        unsafe { mbGetNavigateIndex.unwrap()(self.webview) }
    }

    pub fn stop_loading(&self) {
        unsafe {
            mbStopLoading.unwrap()(self.webview);
        }
    }

    pub fn reload(&self) {
        unsafe {
            mbReload.unwrap()(self.webview);
        }
    }

    pub fn perform_cookie_command(&self, command: CookieCommand) {
        unsafe {
            mbPerformCookieCommand.unwrap()(self.webview, command as i32);
        }
    }

    pub fn editor_select_all(&self) {
        unsafe {
            mbEditorSelectAll.unwrap()(self.webview);
        }
    }

    pub fn editor_unselect(&self) {
        unsafe {
            mbEditorUnSelect.unwrap()(self.webview);
        }
    }

    pub fn editor_copy(&self) {
        unsafe {
            mbEditorCopy.unwrap()(self.webview);
        }
    }

    pub fn editor_cut(&self) {
        unsafe {
            mbEditorCut.unwrap()(self.webview);
        }
    }
    pub fn editor_paste(&self) {
        unsafe {
            mbEditorPaste.unwrap()(self.webview);
        }
    }
    pub fn editor_delete(&self) {
        unsafe {
            mbEditorDelete.unwrap()(self.webview);
        }
    }
    pub fn editor_undo(&self) {
        unsafe {
            mbEditorUndo.unwrap()(self.webview);
        }
    }

    pub fn editor_redo(&self) {
        unsafe {
            mbEditorRedo.unwrap()(self.webview);
        }
    }

    pub fn set_focus(&self) {
        unsafe {
            mbSetFocus.unwrap()(self.webview);
        }
    }

    pub fn kill_focus(&self) {
        unsafe {
            mbKillFocus.unwrap()(self.webview);
        }
    }

    pub fn show(&self) {
        unsafe {
            mbShowWindow.unwrap()(self.webview, 1);
        }
    }

    pub fn hide(&self) {
        unsafe {
            mbShowWindow.unwrap()(self.webview, 0);
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

    pub fn post_url(&self, url: &str, post_data: &str) -> Result<()> {
        unsafe {
            let url_c = to_cstr_ptr(url)?;
            let post_data_c = to_cstr_ptr(post_data)?;
            mbPostURL.unwrap()(self.webview, url_c, post_data_c, post_data.len() as i32);
            Ok(())
        }
    }

    pub fn get_locked_view_dc(&self) -> HDC {
        unsafe { mbGetLockedViewDC.unwrap()(self.webview) }
    }

    pub fn unlock_view_dc(&self) {
        unsafe {
            mbUnlockViewDC.unwrap()(self.webview);
        }
    }

    pub fn wake(&self) {
        unsafe {
            mbWake.unwrap()(self.webview);
        }
    }

    pub fn get_main_frame(&self) -> WebFrame {
        unsafe {
            let frame = mbWebFrameGetMainFrame.unwrap()(self.webview);
            WebFrame::from_mb(self.webview, frame)
        }
    }

    pub fn is_main(&self, frame: &WebFrame) -> bool {
        unsafe { from_bool_int(mbIsMainFrame.unwrap()(self.webview, frame.frame)) }
    }

    pub fn set_node_js_enabled(&self, enable: bool) {
        unsafe {
            mbSetNodeJsEnable.unwrap()(self.webview, to_bool_int(enable));
        }
    }

    pub async fn get_content_as_markup(&self) -> String {
        todo!()
    }

    pub async fn get_source(&self) -> String {
        todo!()
    }

    pub async fn util_serialize_to_mhtml(&self) -> String {
        todo!()
    }

    pub fn set_default_printer_settings(&self, settings: DefaultPrinterSettings) {
        unsafe {
            let settings = settings.into_mb();
            mbUtilSetDefaultPrinterSettings.unwrap()(self.webview, &settings);
        }
    }

    pub fn get_content_size(&self) -> Size {
        unsafe {
            Size {
                width: mbGetContentWidth.unwrap()(self.webview),
                height: mbGetContentHeight.unwrap()(self.webview),
            }
        }
    }

    pub fn set_user_value<T: 'static>(&self, key: &str, value: T) -> Result<()> {
        unsafe {
            let ptr = UserValue::into_raw(UserValue::new(value));
            mbSetUserKeyValue.unwrap()(self.webview, to_cstr_ptr(key)?, ptr as *mut c_void);
            Ok(())
        }
    }

    pub fn get_user_value<T: 'static>(&self, key: &str) -> Result<Arc<UserValue<T>>> {
        unsafe {
            let ptr = mbGetUserKeyValue.unwrap()(self.webview, to_cstr_ptr(key)?);
            UserValue::from_raw(ptr as *const UserValue<T>)
        }
    }

    pub fn go_to_offset(&self, offset: i32) {
        unsafe {
            mbGoToOffset.unwrap()(self.webview, offset);
        }
    }

    pub fn go_to_index(&self, index: i32) {
        unsafe {
            mbGoToIndex.unwrap()(self.webview, index);
        }
    }

    pub fn set_editable(&self, editable: bool) {
        unsafe {
            mbSetEditable.unwrap()(self.webview, editable);
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DefaultPrinterSettings {
    pub is_landscape: bool,
    pub is_print_head_footer: bool,
    pub is_print_backgroud: bool,
    pub edge_distance_left: i32,
    pub edge_distance_top: i32,
    pub edge_distance_right: i32,
    pub edge_distance_bottom: i32,
    pub copies: i32,
    pub paper_type: i32,
}

impl DefaultPrinterSettings {
    pub(crate) fn into_mb(&self) -> mbDefaultPrinterSettings {
        mbDefaultPrinterSettings {
            structSize: std::mem::size_of::<mbDefaultPrinterSettings>() as i32,
            isLandscape: to_bool_int(self.is_landscape),
            isPrintHeadFooter: to_bool_int(self.is_print_head_footer),
            isPrintBackgroud: to_bool_int(self.is_print_backgroud),
            edgeDistanceLeft: self.edge_distance_left,
            edgeDistanceTop: self.edge_distance_top,
            edgeDistanceRight: self.edge_distance_right,
            edgeDistanceBottom: self.edge_distance_bottom,
            copies: self.copies,
            paperType: self.paper_type,
        }
    }
}

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CookieCommand {
    ClearAllCookies = mbCookieCommand_mbCookieCommandClearAllCookies,
    ClearSessionCookies = mbCookieCommand_mbCookieCommandClearSessionCookies,
    FlushCookiesToFile = mbCookieCommand_mbCookieCommandFlushCookiesToFile,
    ReloadCookiesFromFile = mbCookieCommand_mbCookieCommandReloadCookiesFromFile,
}
