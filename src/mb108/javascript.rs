use crate::error::Result;
use crate::utils::{from_bool_int, from_cstr_ptr};
use mb108_sys::{
    mbGetJsValueType, mbJsExecState, mbJsToBoolean, mbJsToDouble, mbJsToString,
    mbJsType_kMbJsTypeBool, mbJsType_kMbJsTypeNull, mbJsType_kMbJsTypeNumber,
    mbJsType_kMbJsTypeString, mbJsType_kMbJsTypeUndefined, mbJsType_kMbJsTypeV8Value, mbJsValue,
};

pub struct ExecState {
    state: mbJsExecState,
}

pub struct JsValue {
    value: mbJsValue,
}

#[repr(i32)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum JsType {
    Number = mbJsType_kMbJsTypeNumber,
    String = mbJsType_kMbJsTypeString,
    Bool = mbJsType_kMbJsTypeBool,
    Undefined = mbJsType_kMbJsTypeUndefined,
    Null = mbJsType_kMbJsTypeNull,
    V8Value = mbJsType_kMbJsTypeV8Value,
}

impl ExecState {
    pub(crate) fn from_mb(state: mbJsExecState) -> Self {
        Self { state }
    }
}

impl JsValue {
    pub(crate) fn from_mb(value: mbJsValue) -> Self {
        Self { value }
    }

    #[allow(non_upper_case_globals)]
    pub fn get_type(&self, state: &ExecState) -> JsType {
        unsafe {
            match mbGetJsValueType.unwrap()(state.state, self.value) {
                mbJsType_kMbJsTypeNumber => JsType::Number,
                mbJsType_kMbJsTypeString => JsType::String,
                mbJsType_kMbJsTypeBool => JsType::Bool,
                mbJsType_kMbJsTypeUndefined => JsType::Undefined,
                mbJsType_kMbJsTypeNull => JsType::Null,
                _ => JsType::V8Value,
            }
        }
    }

    pub fn to_double(&self, state: &ExecState) -> f64 {
        unsafe { mbJsToDouble.unwrap()(state.state, self.value) }
    }

    pub fn to_boolean(&self, state: &ExecState) -> bool {
        unsafe { from_bool_int(mbJsToBoolean.unwrap()(state.state, self.value)) }
    }

    pub fn to_string(&self, state: &ExecState) -> Result<String> {
        unsafe { from_cstr_ptr(mbJsToString.unwrap()(state.state, self.value)) }
    }
}
