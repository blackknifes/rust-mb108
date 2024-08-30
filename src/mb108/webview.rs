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
    mbGetContentAsMarkup, mbGetContentHeight, mbGetContentWidth, mbGetCursorInfoType,
    mbGetHostHWND, mbGetLockedViewDC, mbGetNavigateIndex, mbGetPlatformWindowHandle, mbGetSize,
    mbGetSource, mbGetTitle, mbGetUrl, mbGetUserKeyValue, mbGetWebViewForCurrentContext,
    mbGetZoomFactor, mbGoBack, mbGoForward, mbGoToIndex, mbGoToOffset, mbInsertCSSByFrame,
    mbIsAudioMuted, mbIsMainFrame, mbKillFocus, mbLoadHtmlWithBaseUrl, mbLoadURL, mbMoveToCenter,
    mbMoveWindow, mbNavigateAtIndex, mbPerformCookieCommand, mbPostURL, mbRect, mbReload, mbResize,
    mbSetAudioMuted, mbSetContextMenuEnabled, mbSetContextMenuItemShow, mbSetCookie,
    mbSetCookieEnabled, mbSetCookieJarFullPath, mbSetCookieJarPath, mbSetCspCheckEnable,
    mbSetDebugConfig, mbSetDiskCacheEnabled, mbSetDiskCacheLevel, mbSetDiskCacheLimit,
    mbSetDiskCacheLimitDisk, mbSetDiskCachePath, mbSetDragDropEnable, mbSetDragEnable,
    mbSetEditable, mbSetFocus, mbSetHandle, mbSetHandleOffset, mbSetHeadlessEnabled,
    mbSetLocalStorageFullPath, mbSetMemoryCacheEnable, mbSetMouseEnabled,
    mbSetNavigationToNewWindowEnable, mbSetNodeJsEnable, mbSetNpapiPluginsEnabled, mbSetProxy,
    mbSetResourceGc, mbSetSystemTouchEnabled, mbSetTouchEnabled, mbSetTransparent, mbSetUserAgent,
    mbSetUserKeyValue, mbSetWindowTitle, mbSetZoomFactor, mbShowWindow, mbStopLoading,
    mbUnlockViewDC, mbUtilSerializeToMHTML, mbUtilSetDefaultPrinterSettings, mbWake,
    mbWebFrameGetMainFrame, mbWebView, HDC, HWND,
};
use std::any::Any;
use std::cell::RefCell;
use std::collections::HashMap;
use std::ffi::c_void;
use std::rc::Rc;
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

pub struct WebViewInner {
    webview: mbWebView,
    values: HashMap<String, Box<dyn Any>>,
}

pub struct WebView {
    inner: Rc<RefCell<WebViewInner>>,
}

impl WebView {
    pub(crate) fn get_webview_handle(&self) -> mbWebView {
        self.inner.borrow().webview
    }

    /// 创建弹出窗口
    /// - 依赖接口 mbCreateWebWindow
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

            let inner = Rc::new(RefCell::new(WebViewInner {
                webview,
                values: Default::default(),
            }));

            mbSetUserKeyValue.unwrap()(
                webview,
                to_cstr_ptr("rust").unwrap().to_utf8(),
                Rc::into_raw(inner.clone()) as *mut c_void,
            );

            mbOnClose

            return WebView { inner };
        }
    }

    /// 从当前上下文获取WebView
    /// - 依赖接口 mbGetWebViewForCurrentContext
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

    /// 移动窗口
    /// - 依赖接口 mbMoveWindow
    pub fn move_to(&self, rc: Rect) {
        unsafe {
            mbMoveWindow.unwrap()(self.get_webview_handle(), rc.x, rc.y, rc.width, rc.height);
        }
    }

    /// 移动窗口到屏幕中间
    /// - 依赖接口 mbMoveToCenter
    pub fn move_to_center(&self) {
        unsafe {
            mbMoveToCenter.unwrap()(self.get_webview_handle());
        }
    }

    /// 获取光标矩形
    /// - 依赖接口 mbGetCaretRect
    pub fn get_caret_rect(&self) -> Rect {
        unsafe {
            let mut rc = mbRect {
                x: 0,
                y: 0,
                w: 0,
                h: 0,
            };
            mbGetCaretRect.unwrap()(self.get_webview_handle(), &mut rc);
            return Rect::from_mb(&rc);
        }
    }

    /// 获取是否禁音
    /// - 依赖接口 mbSetAudioMuted
    pub fn set_audio_muted(&self, mute: bool) {
        unsafe {
            mbSetAudioMuted.unwrap()(self.get_webview_handle(), to_bool_int(mute));
        }
    }

    /// 是否禁音
    /// - 依赖接口 mbIsAudioMuted
    pub fn is_audio_muted(&self) -> bool {
        unsafe {
            return from_bool_int(mbIsAudioMuted.unwrap()(self.get_webview_handle()));
        }
    }

    /// 设置代理
    /// - 依赖接口 mbSetProxy
    pub fn set_proxy(&self, proxy: Proxy) -> crate::error::Result<()> {
        unsafe {
            let mb_proxy = proxy.into_native()?;
            mbSetProxy.unwrap()(self.get_webview_handle(), &mb_proxy);
            Ok(())
        }
    }

    /// 设置调试配置
    /// - 依赖接口 mbSetDebugConfig
    pub fn set_debug_config(&self, config: DebugConfig) -> Result<()> {
        unsafe {
            let (debug_str, param) = config.get_native_params();
            mbSetDebugConfig.unwrap()(
                self.get_webview_handle(),
                to_cstr_ptr(&debug_str)?.to_utf8(),
                to_cstr_ptr(&param)?.to_utf8(),
            );
            Ok(())
        }
    }

    /// 设置鼠标是否启用
    /// - 依赖接口 mbSetMouseEnabled
    pub fn set_mouse_enabled(&self, enable: bool) {
        unsafe {
            mbSetMouseEnabled.unwrap()(self.get_webview_handle(), to_bool_int(enable));
        }
    }

    /// 设置触摸是否启用
    /// - 依赖接口 mbSetTouchEnabled
    pub fn set_touch_enabled(&self, enable: bool) {
        unsafe {
            mbSetTouchEnabled.unwrap()(self.get_webview_handle(), to_bool_int(enable));
        }
    }

    /// 设置系统触摸是否启用
    /// - 依赖接口 mbSetSystemTouchEnabled
    pub fn set_system_touch_enabled(&self, enable: bool) {
        unsafe {
            mbSetSystemTouchEnabled.unwrap()(self.get_webview_handle(), to_bool_int(enable));
        }
    }

    /// 设置上下文菜单是否启用
    /// - 依赖接口 mbSetContextMenuEnabled
    pub fn set_context_menu_enabled(&self, enable: bool) {
        unsafe {
            mbSetContextMenuEnabled.unwrap()(self.get_webview_handle(), to_bool_int(enable));
        }
    }

    /// 设置导航到新窗口是否启用
    /// - 依赖接口 mbSetNavigationToNewWindowEnable
    pub fn set_navigation_to_new_window_enabled(&self, enable: bool) {
        unsafe {
            mbSetNavigationToNewWindowEnable.unwrap()(
                self.get_webview_handle(),
                to_bool_int(enable),
            );
        }
    }

    /// 设置无头模式是否启用
    /// - 依赖接口 mbSetHeadlessEnabled
    pub fn set_headless_enabled(&self, enable: bool) {
        unsafe {
            mbSetHeadlessEnabled.unwrap()(self.get_webview_handle(), to_bool_int(enable));
        }
    }

    /// 设置拖拽放置是否启用
    /// - 依赖接口 mbSetDragDropEnable
    pub fn set_drag_drop_enabled(&self, enable: bool) {
        unsafe {
            mbSetDragDropEnable.unwrap()(self.get_webview_handle(), to_bool_int(enable));
        }
    }

    /// 设置拖拽是否启用
    /// - 依赖接口 mbSetDragEnable
    pub fn set_drag_enabled(&self, enable: bool) {
        unsafe {
            mbSetDragEnable.unwrap()(self.get_webview_handle(), to_bool_int(enable));
        }
    }

    /// 设置上下文菜单子项是否显示
    /// - 依赖接口 mbSetContextMenuItemShow
    pub fn set_context_menu_item_show(&self, menu_item_id: MenuItemId, show: bool) {
        unsafe {
            mbSetContextMenuItemShow.unwrap()(
                self.get_webview_handle(),
                menu_item_id as i32,
                to_bool_int(show),
            );
        }
    }

    /// 设置无头模式窗口句柄
    /// - 依赖接口 mbSetHandle
    pub fn set_handle(&self, hwnd: HWND) {
        unsafe {
            mbSetHandle.unwrap()(self.get_webview_handle(), hwnd);
        }
    }

    /// 设置无头模式窗口偏移
    /// - 依赖接口 mbSetHandleOffset
    pub fn set_handle_offset(&self, x: i32, y: i32) {
        unsafe {
            mbSetHandleOffset.unwrap()(self.get_webview_handle(), x, y);
        }
    }

    /// 获取平台窗口句柄
    /// - 依赖接口 mbGetPlatformWindowHandle
    pub fn get_platform_window_handle(&self) -> *mut c_void {
        unsafe {
            return mbGetPlatformWindowHandle.unwrap()(self.get_webview_handle());
        }
    }

    /// 获取窗口句柄
    /// - 依赖接口 mbGetHostHWND
    pub fn get_host_hwnd(&self) -> HWND {
        unsafe {
            return mbGetHostHWND.unwrap()(self.get_webview_handle());
        }
    }

    /// 设置是否半透明
    /// - 依赖接口 mbSetTransparent
    pub fn set_transparent(&self, transparent: bool) {
        unsafe {
            mbSetTransparent.unwrap()(self.get_webview_handle(), to_bool_int(transparent));
        }
    }

    /// 设置CSP检查是否启用
    /// - 依赖接口 mbSetCspCheckEnable
    pub fn set_csp_check_enabled(&self, enable: bool) {
        unsafe {
            mbSetCspCheckEnable.unwrap()(self.get_webview_handle(), to_bool_int(enable));
        }
    }

    /// 设置NAPI插件是否启用
    /// - 依赖接口 mbSetNpapiPluginsEnabled
    pub fn set_npapi_plugins_enabled(&self, enable: bool) {
        unsafe {
            mbSetNpapiPluginsEnabled.unwrap()(self.get_webview_handle(), to_bool_int(enable));
        }
    }

    /// 设置内存缓存是否启用
    /// - 依赖接口 mbSetMemoryCacheEnable
    pub fn set_memory_cache_enabled(&self, enable: bool) {
        unsafe {
            mbSetMemoryCacheEnable.unwrap()(self.get_webview_handle(), to_bool_int(enable));
        }
    }

    /// 设置cookie是否启用
    /// - 依赖接口 mbSetCookieEnabled
    pub fn set_cookie_enabled(&self, enable: bool) {
        unsafe {
            mbSetCookieEnabled.unwrap()(self.get_webview_handle(), to_bool_int(enable));
        }
    }

    /// 设置cookie
    /// - 依赖接口 mbSetCookie
    pub fn set_cookie(&self, url: &str, cookie: &str) -> Result<()> {
        unsafe {
            mbSetCookie.unwrap()(
                self.get_webview_handle(),
                to_cstr_ptr(url)?.to_utf8(),
                to_cstr_ptr(cookie)?.to_utf8(),
            );
            Ok(())
        }
    }

    /// 设置cookie jar路径
    /// - 依赖接口 mbSetCookieJarPath
    pub fn set_cookie_jar_path(&self, path: &str) {
        unsafe {
            mbSetCookieJarPath.unwrap()(self.get_webview_handle(), (&to_cstr16_ptr(path)).as_ptr());
        }
    }

    /// 设置cookie jar 绝对路径
    /// - 依赖接口 mbSetCookieJarFullPath
    pub fn set_cookie_jar_full_path(&self, path: &str) {
        unsafe {
            mbSetCookieJarFullPath.unwrap()(
                self.get_webview_handle(),
                (&to_cstr16_ptr(path)).as_ptr(),
            );
        }
    }

    /// 设置local storage 绝对路径
    /// - 依赖接口 mbSetLocalStorageFullPath
    pub fn set_local_storage_full_path(&self, path: &str) {
        unsafe {
            mbSetLocalStorageFullPath.unwrap()(
                self.get_webview_handle(),
                (&to_cstr16_ptr(path)).as_ptr(),
            );
        }
    }

    /// 获取标题
    /// - 依赖接口 mbGetTitle
    pub fn get_title(&self) -> Result<String> {
        unsafe {
            let title = CStr::from_ptr(mbGetTitle.unwrap()(self.get_webview_handle()))
                .to_str()
                .map_err(Error::other)?
                .to_owned();
            Ok(title)
        }
    }

    /// 设置标题
    /// - 依赖接口 mbSetWindowTitle
    pub fn set_title(&self, title: &str) -> Result<()> {
        unsafe {
            mbSetWindowTitle.unwrap()(self.get_webview_handle(), to_cstr_ptr(title)?.to_utf8());
            Ok(())
        }
    }

    /// 获取url
    /// - 依赖接口 mbGetUrl
    pub fn get_url(&self) -> Result<String> {
        unsafe {
            let title = CStr::from_ptr(mbGetUrl.unwrap()(self.get_webview_handle()))
                .to_str()
                .map_err(Error::other)?
                .to_owned();
            Ok(title)
        }
    }

    /// 获取光标类型
    /// - 依赖接口 mbGetCursorInfoType
    pub fn get_cursor_info_type(&self) -> i32 {
        unsafe {
            return mbGetCursorInfoType.unwrap()(self.get_webview_handle());
        }
    }

    /// 添加插件目录
    /// - 依赖接口 mbAddPluginDirectory
    pub fn add_plugin_directory(&self, path: &str) {
        unsafe {
            mbAddPluginDirectory.unwrap()(
                self.get_webview_handle(),
                (&to_cstr16_ptr(path)).as_ptr(),
            );
        }
    }

    /// 设置user-agent
    /// - 依赖接口 mbSetUserAgent
    pub fn set_user_agent(&self, user_agent: &str) -> Result<()> {
        unsafe {
            mbSetUserAgent.unwrap()(
                self.get_webview_handle(),
                to_cstr_ptr(user_agent)?.to_utf8(),
            );
            Ok(())
        }
    }

    /// 设置缩放因子
    /// - 依赖接口 mbSetZoomFactor
    pub fn set_zoom_factor(&self, factor: f32) {
        unsafe {
            mbSetZoomFactor.unwrap()(self.get_webview_handle(), factor);
        }
    }

    /// 获取缩放因子
    /// - 依赖接口 mbGetZoomFactor
    pub fn get_zoom_factor(&self) -> f32 {
        unsafe { mbGetZoomFactor.unwrap()(self.get_webview_handle()) }
    }

    /// 设置硬盘缓存是否启用
    /// - 依赖接口 mbSetDiskCacheEnabled
    pub fn set_disk_cache_enabled(&self, enable: bool) {
        unsafe {
            mbSetDiskCacheEnabled.unwrap()(self.get_webview_handle(), to_bool_int(enable));
        }
    }

    /// 设置硬盘缓存路径
    /// - 依赖接口 mbSetDiskCachePath
    pub fn set_disk_cache_path(&self, path: &str) {
        unsafe {
            mbSetDiskCachePath.unwrap()(self.get_webview_handle(), (&to_cstr16_ptr(path)).as_ptr());
        }
    }

    /// 设置硬盘缓存大小限制
    /// - 依赖接口 mbSetDiskCacheLimit
    pub fn set_disk_cache_limit(&self, limit: usize) {
        unsafe {
            mbSetDiskCacheLimit.unwrap()(self.get_webview_handle(), limit);
        }
    }

    /// 设置硬盘缓存大小限制
    /// - 依赖接口 mbSetDiskCacheLimitDisk
    pub fn set_disk_cache_limit_disk(&self, limit: usize) {
        unsafe {
            mbSetDiskCacheLimitDisk.unwrap()(self.get_webview_handle(), limit);
        }
    }

    /// 设置硬盘缓存级别
    /// - 依赖接口 mbSetDiskCacheLevel
    pub fn set_disk_cache_level(&self, level: i32) {
        unsafe {
            mbSetDiskCacheLevel.unwrap()(self.get_webview_handle(), level);
        }
    }

    /// 设置资源gc间隔时间（秒）
    /// - 依赖接口 mbSetResourceGc
    pub fn set_resource_gc(&self, interval_seconds: i32) {
        unsafe {
            mbSetResourceGc.unwrap()(self.get_webview_handle(), interval_seconds);
        }
    }

    /// 是否能前进
    /// - 依赖接口 can_go_forward
    pub fn can_go_forward(&self) -> bool {
        todo!()
    }

    /// 是否能后退
    /// - 依赖接口 can_go_back
    pub fn can_go_back(&self) -> bool {
        todo!()
    }

    /// 获取cookie
    /// - 依赖接口 get_cookie
    pub fn get_cookie(&self) {
        todo!()
    }

    /// 清除cookie
    /// - 依赖接口 mbClearCookie
    pub fn clear_cookie(&self) {
        unsafe {
            mbClearCookie.unwrap()(self.get_webview_handle());
        }
    }

    /// 调整大小
    /// - 依赖接口 mbResize
    pub fn resize(&self, width: u32, height: u32) {
        unsafe {
            mbResize.unwrap()(self.get_webview_handle(), width as i32, height as i32);
        }
    }

    /// 获取大小
    /// - 依赖接口 mbGetSize
    pub fn get_size(&self) -> Rect {
        unsafe {
            let mut rc = mbRect {
                x: 0,
                y: 0,
                w: 0,
                h: 0,
            };
            mbGetSize.unwrap()(self.get_webview_handle(), &mut rc);
            Rect::from_mb(&rc)
        }
    }

    /// 后退
    /// - 依赖接口 mbGoBack
    pub fn go_back(&self) {
        unsafe {
            mbGoBack.unwrap()(self.get_webview_handle());
        }
    }

    /// 前进
    /// - 依赖接口 mbGoForward
    pub fn go_forward(&self) {
        unsafe {
            mbGoForward.unwrap()(self.get_webview_handle());
        }
    }

    /// 导航到第index个记录
    /// - 依赖接口 mbNavigateAtIndex
    pub fn navigate_at_index(&self, index: i32) {
        unsafe {
            mbNavigateAtIndex.unwrap()(self.get_webview_handle(), index);
        }
    }

    /// 获取导航第index个记录
    /// - 依赖接口 mbGetNavigateIndex
    pub fn get_navigate_index(&self) -> i32 {
        unsafe { mbGetNavigateIndex.unwrap()(self.get_webview_handle()) }
    }

    /// 停止加载
    /// - 依赖接口 mbStopLoading
    pub fn stop_loading(&self) {
        unsafe {
            mbStopLoading.unwrap()(self.get_webview_handle());
        }
    }

    /// 重载刷新
    /// - 依赖接口 mbReload
    pub fn reload(&self) {
        unsafe {
            mbReload.unwrap()(self.get_webview_handle());
        }
    }

    /// 执行cookie命令
    /// - 依赖接口 mbPerformCookieCommand
    pub fn perform_cookie_command(&self, command: CookieCommand) {
        unsafe {
            mbPerformCookieCommand.unwrap()(self.get_webview_handle(), command as i32);
        }
    }

    /// 全选
    /// - 依赖接口 mbEditorSelectAll
    pub fn select_all(&self) {
        unsafe {
            mbEditorSelectAll.unwrap()(self.get_webview_handle());
        }
    }

    /// 取消全选
    /// - 依赖接口 mbEditorUnSelect
    pub fn unselect(&self) {
        unsafe {
            mbEditorUnSelect.unwrap()(self.get_webview_handle());
        }
    }

    /// 复制
    /// - 依赖接口 mbEditorCopy
    pub fn copy(&self) {
        unsafe {
            mbEditorCopy.unwrap()(self.get_webview_handle());
        }
    }

    /// 剪切
    /// - 依赖接口 mbEditorCut
    pub fn cut(&self) {
        unsafe {
            mbEditorCut.unwrap()(self.get_webview_handle());
        }
    }

    /// 粘贴
    /// - 依赖接口 mbEditorPaste
    pub fn paste(&self) {
        unsafe {
            mbEditorPaste.unwrap()(self.get_webview_handle());
        }
    }

    /// 删除
    /// - 依赖接口 mbEditorDelete
    pub fn delete(&self) {
        unsafe {
            mbEditorDelete.unwrap()(self.get_webview_handle());
        }
    }

    /// 撤销
    /// - 依赖接口 mbEditorUndo
    pub fn undo(&self) {
        unsafe {
            mbEditorUndo.unwrap()(self.get_webview_handle());
        }
    }

    /// 重做
    /// - 依赖接口 mbEditorRedo
    pub fn redo(&self) {
        unsafe {
            mbEditorRedo.unwrap()(self.get_webview_handle());
        }
    }

    /// 设置聚焦
    /// - 依赖接口 mbSetFocus
    pub fn set_focus(&self) {
        unsafe {
            mbSetFocus.unwrap()(self.get_webview_handle());
        }
    }

    /// 失去焦点
    /// - 依赖接口 mbKillFocus
    pub fn kill_focus(&self) {
        unsafe {
            mbKillFocus.unwrap()(self.get_webview_handle());
        }
    }

    /// 显示窗口
    /// - 依赖接口 mbShowWindow
    pub fn show(&self) {
        unsafe {
            mbShowWindow.unwrap()(self.get_webview_handle(), 1);
        }
    }

    /// 隐藏窗口
    /// - 依赖接口 mbShowWindow
    pub fn hide(&self) {
        unsafe {
            mbShowWindow.unwrap()(self.get_webview_handle(), 0);
        }
    }

    /// 从url加载
    /// - 依赖接口 mbLoadURL
    pub fn load_url(&self, url: &str) -> Result<()> {
        unsafe {
            mbLoadURL.unwrap()(self.get_webview_handle(), to_cstr_ptr(url)?.to_utf8());
            Ok(())
        }
    }

    /// 从html加载
    /// - 依赖接口 mbLoadHtmlWithBaseUrl
    pub fn load_html_with_base_url(&self, html: &str, base_url: &str) -> Result<()> {
        unsafe {
            mbLoadHtmlWithBaseUrl.unwrap()(
                self.get_webview_handle(),
                to_cstr_ptr(html)?.to_utf8(),
                to_cstr_ptr(base_url)?.to_utf8(),
            );
            Ok(())
        }
    }

    /// 从post url加载
    /// - 依赖接口 mbPostURL
    pub fn post_url(&self, url: &str, post_data: &str) -> Result<()> {
        unsafe {
            let url_c = to_cstr_ptr(url)?;
            let post_data_c = to_cstr_ptr(post_data)?;
            mbPostURL.unwrap()(
                self.get_webview_handle(),
                url_c.to_utf8(),
                post_data_c.to_utf8(),
                post_data.len() as i32,
            );
            Ok(())
        }
    }

    /// 获取主frame
    /// - 依赖接口 mbWebFrameGetMainFrame
    pub fn get_main_frame(&self) -> WebFrame {
        unsafe {
            let frame = mbWebFrameGetMainFrame.unwrap()(self.get_webview_handle());
            WebFrame::from_mb(self.get_webview_handle(), frame)
        }
    }

    /// 设置nodejs是否启用
    /// - 依赖接口 mbSetNodeJsEnable
    pub fn set_node_js_enabled(&self, enable: bool) {
        unsafe {
            mbSetNodeJsEnable.unwrap()(self.get_webview_handle(), to_bool_int(enable));
        }
    }

    /// 获取content
    /// - 依赖接口 mbGetContentAsMarkup
    pub async fn get_content_as_markup(&self) -> String {
        unsafe {
            // mbGetContentAsMarkup.unwrap()
            todo!()
        }
    }

    /// 获取源码
    /// - 依赖接口 mbGetSource
    pub async fn get_source(&self) -> String {
        unsafe {
            // mbGetSource.unwrap()
            todo!()
        }
    }

    /// 序列化为mhtml
    /// - 依赖接口 mbUtilSerializeToMHTML
    pub async fn serialize_to_mhtml(&self) -> String {
        unsafe {
            // mbUtilSerializeToMHTML.unwrap()()
            todo!()
        }
    }

    /// 设置默认打印设置
    /// - 依赖接口 mbUtilSetDefaultPrinterSettings
    pub fn set_default_printer_settings(&self, settings: DefaultPrinterSettings) {
        unsafe {
            let settings = settings.into_mb();
            mbUtilSetDefaultPrinterSettings.unwrap()(self.get_webview_handle(), &settings);
        }
    }

    /// 获取content大小
    /// - 依赖接口 mbGetContentWidth, mbGetContentHeight
    pub fn get_content_size(&self) -> Size {
        unsafe {
            Size {
                width: mbGetContentWidth.unwrap()(self.get_webview_handle()),
                height: mbGetContentHeight.unwrap()(self.get_webview_handle()),
            }
        }
    }

    /// 设置绑定的用户值
    /// - 依赖接口 mbSetUserKeyValue
    pub fn set_user_value<T: 'static>(&self, key: &str, value: T) -> Result<()> {
        unsafe {
            let ptr = UserValue::into_raw(UserValue::new(value));
            mbSetUserKeyValue.unwrap()(
                self.get_webview_handle(),
                to_cstr_ptr(key)?.to_utf8(),
                ptr as *mut c_void,
            );
            Ok(())
        }
    }

    /// 获取绑定的用户值
    /// - 依赖接口 mbGetUserKeyValue
    pub fn get_user_value<T: 'static>(&self, key: &str) -> Result<Arc<UserValue<T>>> {
        unsafe {
            let ptr =
                mbGetUserKeyValue.unwrap()(self.get_webview_handle(), to_cstr_ptr(key)?.to_utf8());
            UserValue::from_raw(ptr as *const UserValue<T>)
        }
    }

    /// 导航至offset偏移历史记录
    /// - 依赖接口 mbGoToOffset
    pub fn go_to_offset(&self, offset: i32) {
        unsafe {
            mbGoToOffset.unwrap()(self.get_webview_handle(), offset);
        }
    }

    /// 导航至第index个历史记录
    /// - 依赖接口 mbGoToIndex
    pub fn go_to_index(&self, index: i32) {
        unsafe {
            mbGoToIndex.unwrap()(self.get_webview_handle(), index);
        }
    }

    /// 设置是否可编辑
    /// - 依赖接口 mbSetEditable
    pub fn set_editable(&self, editable: bool) {
        unsafe {
            mbSetEditable.unwrap()(self.get_webview_handle(), editable);
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
