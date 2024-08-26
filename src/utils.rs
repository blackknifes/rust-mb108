use crate::error::{Error, Result};
use mb108_sys::BOOL;
use std::ffi::CString;

pub(crate) unsafe fn to_cstr16_ptr(str: &str) -> Vec<u16> {
    let mut str_u16 = str.encode_utf16().collect::<Vec<u16>>();
    str_u16.push(0);
    return str_u16;
}

pub(crate) unsafe fn to_cstr_ptr(str: &str) -> Result<*const i8> {
    return Ok(CString::new(str).map_err(|err| Error::other(err))?.as_ptr());
}

pub(crate) fn to_bool_int(value: bool) -> BOOL {
    if value {
        1
    } else {
        0
    }
}

pub(crate) fn from_bool_int(value: BOOL) -> bool {
    if value == 0 {
        false
    } else {
        true
    }
}
