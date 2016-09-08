#![allow(non_upper_case_globals)]
extern crate dero;
extern crate libc;

use libc::{c_char};
use std::ffi::{CStr, CString};
use std::str;
use std::io::{self, Write};
use std::panic;
use std::process;

pub const dero_OK: i32 = 0;
pub const dero_ERR_NULL: i32 = -1;
pub const dero_ERR_UTF8: i32 = -2;
pub const dero_ERR_CONVERT: i32 = -3;
pub const dero_ERR_PANIC: i32 = -128;

#[no_mangle]
pub unsafe extern "C" fn dero_error_message(error_code: i32) -> *const c_char {
    let bytes: &'static [u8] = match error_code {
        dero_OK => &b"no error\0"[..],
        dero_ERR_NULL => &b"the text pointer is null\0"[..],
        dero_ERR_UTF8 => &b"invalid UTF-8 in text\0"[..],
        dero_ERR_CONVERT => &b"the full text could not be converted\0"[..],
        dero_ERR_PANIC => &b"the converter panicked\0"[..],
        _ => &b"unknown error\0"[..],
    };
    bytes.as_ptr() as *const c_char
}

#[no_mangle]
pub unsafe extern "C" fn dero_explain_error(text: *const c_char) {
    let res = panic::catch_unwind(move || {
        if text.is_null() {
            write!(io::stderr(), "The text pointer is null").unwrap();
            return;
        }
        let res = str::from_utf8(CStr::from_ptr(text).to_bytes());
        let text = if let Ok(text) = res {
            text
        } else {
            write!(io::stderr(), "Could not read text").unwrap();
            return;
        };
        match dero::convert(text) {
            Ok(_) => {
                write!(io::stderr(), "No error found").unwrap();
            }
            Err(err) => {
                let _ = err.print_explanation(text);
            }
        }
    });
    if res.is_err() {
        process::exit(-1);
    }
}

#[no_mangle]
pub unsafe extern "C" fn dero_convert(text: *const c_char, output: *mut *const c_char) -> i32 {
    let res = panic::catch_unwind(move || {
        if text.is_null() {
            return dero_ERR_NULL;
        }    
        let res = str::from_utf8(CStr::from_ptr(text).to_bytes());
        let text = if let Ok(text) = res {
            text
        } else {
            return dero_ERR_UTF8;
        };
        match dero::convert(text) {
            Ok(converted) => {
                let cstring = if let Ok(cstring) = CString::new(converted) {
                    cstring
                } else {
                    return dero_ERR_UTF8;
                };
                *output = cstring.into_raw();
                dero_OK
            }
            Err(_) => dero_ERR_CONVERT,
        }
    });
    res.unwrap_or(dero_ERR_PANIC)
}

#[no_mangle]
pub unsafe extern "C" fn dero_free_converted(text: *const c_char) {
    if text.is_null() {
        return;
    }
    CString::from_raw(text as *mut c_char);
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
