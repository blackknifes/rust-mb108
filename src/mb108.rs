pub mod common;
pub mod javascript;
pub mod webview;

use crate::error::{Error, Result};
use mb108_sys::{
    _mbSettingMask_MB_ENABLE_DISABLE_CC, _mbSettingMask_MB_ENABLE_DISABLE_H5VIDEO,
    _mbSettingMask_MB_ENABLE_DISABLE_PDFVIEW, _mbSettingMask_MB_ENABLE_ENABLE_EGLGLES2,
    _mbSettingMask_MB_ENABLE_ENABLE_SWIFTSHAER, _mbSettingMask_MB_ENABLE_NODEJS,
    _mbSettingMask_MB_SETTING_PROXY, kMbVersion, mbInit, mbProxy, mbProxyType,
    mbProxyType_MB_PROXY_HTTP, mbProxyType_MB_PROXY_NONE, mbProxyType_MB_PROXY_SOCKS4,
    mbProxyType_MB_PROXY_SOCKS4A, mbProxyType_MB_PROXY_SOCKS5, mbProxyType_MB_PROXY_SOCKS5HOSTNAME,
    mbRunMessageLoop, mbSetMbMainDllPath, mbSettings, mbUninit,
};
use std::ptr::{null, null_mut};

///代理设置
pub struct ProxyOptions {
    ///代理域名或ip地址
    hostname: String,
    ///代理端口
    port: u16,
    ///用户名
    username: Option<String>,
    ///密码
    password: Option<String>,
}

fn encode_to_buf(str: &str, buf: &mut [i8]) -> Result<()> {
    let mut index = 0;
    let mut ch_buf = [0u8; 8];

    for ch in str.chars() {
        let ch_size = ch.encode_utf8(&mut ch_buf).len();
        for i in 0..ch_size {
            if index >= buf.len() - 1 {
                return Err(Error::new(""));
            }

            buf[index] = ch_buf[i] as i8;
            index = index + 1;
        }
    }

    return Ok(());
}

impl ProxyOptions {
    fn into_native(&self, proxy_type: mbProxyType) -> Result<mbProxy> {
        let mut hostname = [0i8; 100];
        let mut username = [0i8; 50];
        let mut password = [0i8; 50];

        encode_to_buf(&self.hostname, &mut hostname)?;
        if let Some(str) = &self.username {
            encode_to_buf(&str, &mut username)?;
        }

        if let Some(str) = &self.password {
            encode_to_buf(&str, &mut password)?;
        }

        Ok(mbProxy {
            type_: proxy_type,
            hostname,
            port: self.port,
            username,
            password,
        })
    }
}

///代理
pub enum Proxy {
    ///禁用代理
    None,
    ///HTTP代理
    Http(ProxyOptions),
    ///Sock4代理
    Sock4(ProxyOptions),
    ///Sock4A代理
    Sock4A(ProxyOptions),
    ///Sock5代理
    Sock5(ProxyOptions),
    ///Sock5Hostname代理
    Sock5Hostname(ProxyOptions),
}

impl PartialEq for Proxy {
    fn eq(&self, other: &Self) -> bool {
        core::mem::discriminant(self) == core::mem::discriminant(other)
    }
}
impl Eq for Proxy {}

impl Proxy {
    fn into_native(&self) -> Result<mbProxy> {
        match self {
            Proxy::None => Ok(mbProxy {
                type_: mbProxyType_MB_PROXY_NONE,
                hostname: [0; 100],
                port: 0,
                username: [0; 50],
                password: [0; 50],
            }),
            Proxy::Http(option) => option.into_native(mbProxyType_MB_PROXY_HTTP),
            Proxy::Sock4(option) => option.into_native(mbProxyType_MB_PROXY_SOCKS4),
            Proxy::Sock4A(option) => option.into_native(mbProxyType_MB_PROXY_SOCKS4A),
            Proxy::Sock5(option) => option.into_native(mbProxyType_MB_PROXY_SOCKS5),
            Proxy::Sock5Hostname(option) => option.into_native(mbProxyType_MB_PROXY_SOCKS5HOSTNAME),
        }
    }
}

impl std::default::Default for Proxy {
    fn default() -> Self {
        Proxy::None
    }
}

///初始化设置
#[derive(Default)]
pub struct Settings {
    ///代理配置
    pub proxy: Proxy,
    pub enable_nodejs: bool,
    pub enable_disable_h5video: bool,
    pub enable_disable_pdfview: bool,
    pub enable_disable_cc: bool,
    pub enable_enable_eglgles2: bool,
    pub enable_enable_swiftshaer: bool,
}

///初始化miniblink
pub fn init(dll: &str, settings: Option<Settings>) -> Result<()> {
    unsafe {
        let mut mask = 0u32;
        let proxy = if let Some(settings) = settings {
            if settings.proxy != Proxy::None {
                mask = mask | _mbSettingMask_MB_SETTING_PROXY as u32;
            }
            if settings.enable_nodejs {
                mask = mask | _mbSettingMask_MB_ENABLE_NODEJS as u32;
            }
            if settings.enable_disable_h5video {
                mask = mask | _mbSettingMask_MB_ENABLE_DISABLE_H5VIDEO as u32;
            }
            if settings.enable_disable_pdfview {
                mask = mask | _mbSettingMask_MB_ENABLE_DISABLE_PDFVIEW as u32;
            }
            if settings.enable_disable_cc {
                mask = mask | _mbSettingMask_MB_ENABLE_DISABLE_CC as u32;
            }
            if settings.enable_enable_eglgles2 {
                mask = mask | _mbSettingMask_MB_ENABLE_ENABLE_EGLGLES2 as u32;
            }
            if settings.enable_enable_swiftshaer {
                mask = mask | _mbSettingMask_MB_ENABLE_ENABLE_SWIFTSHAER as u32;
            }

            settings.proxy.into_native()?
        } else {
            Proxy::None.into_native()?
        };
        let mut dll_u16 = dll.encode_utf16().collect::<Vec<u16>>();
        dll_u16.push(0);
        mbSetMbMainDllPath((&dll_u16).as_ptr());

        let settings_struct = mbSettings {
            proxy,
            mask,
            blinkThreadInitCallback: None,
            blinkThreadInitCallbackParam: null_mut(),
            version: kMbVersion as isize,
            mainDllPath: null(),
            mainDllHandle: null_mut(),
            config: null(),
        };
        mbInit(&settings_struct);
        return Ok(());
    }
}

pub fn uninit() {
    unsafe {
        mbUninit.unwrap()();
    }
}

pub fn run() {
    unsafe {
        mbRunMessageLoop.unwrap()();
    }
}
