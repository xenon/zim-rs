use std::ffi::CStr;

use cxx::{CxxString, CxxVector, UniquePtr};

pub(crate) fn string_from_ptr(ptr: UniquePtr<CxxString>) -> Result<String, ()> {
    match ptr.is_null() {
        true => Err(()),
        false => Ok(ptr.as_ref().unwrap().to_string()),
    }
}

pub(crate) fn vec_string_from_ptr(ptr: UniquePtr<CxxVector<CxxString>>) -> Result<Vec<String>, ()> {
    match ptr.is_null() {
        true => Err(()),
        false => Ok(ptr
            .as_ref()
            .unwrap()
            .iter()
            .map(|s| s.to_string())
            .collect::<Vec<String>>()),
    }
}

pub(crate) unsafe fn string_from_char_array(chars: *const i8) -> Result<String, ()> {
    if chars.is_null() {
        return Err(());
    }
    CStr::from_ptr(chars)
        .to_str()
        .map_err(|_| ())
        .map(|str| str.to_string())
}
