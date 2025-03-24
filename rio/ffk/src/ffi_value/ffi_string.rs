use crate::ffi_convertor::{FromFFI, IntoFFI};
use crate::ffi_value::{FFIValue, IntoFFIValue};
use std::ffi::{CStr, CString};

#[repr(C)]
#[derive(Debug)]
pub struct FFIString {
    pub buffer: *mut std::ffi::c_char,
    pub len: usize,
}

impl IntoFFI for String {
    type FFIType = FFIString;

    fn into_ffi(self) -> Self::FFIType {
        let len = self.len();
        let buffer = CString::new(self).unwrap().into_raw();
        FFIString { buffer, len }
    }
}

impl FromFFI for FFIString {
    type OriginRef = str;
    type OriginOwned = String;

    fn as_origin(&self) -> &Self::OriginRef {
        if self.buffer.is_null() {
            panic!("FFI buffer is null");
        }
        unsafe {
            CStr::from_ptr(self.buffer)
                .to_str()
                .expect("Invalid UTF-8 sequence")
        }
    }

    fn as_origin_mut(&mut self) -> &mut Self::OriginRef {
        if self.buffer.is_null() {
            panic!("FFI buffer is null");
        }
        unsafe {
            let bytes = std::slice::from_raw_parts_mut(self.buffer as *mut u8, self.len);
            std::str::from_utf8_mut(bytes).expect("Invalid UTF-8 sequence")
        }
    }

    fn into_origin(mut self) -> Self::OriginOwned {
        if self.buffer.is_null() {
            panic!("FFI buffer is null");
        }
        let buffer = unsafe { CString::from_raw(self.buffer).to_str().unwrap().to_string() };
        self.buffer = std::ptr::null_mut();
        self.len = 0;
        buffer
    }
}

impl IntoFFIValue for FFIString {
    fn into_ffi_value(self) -> FFIValue {
        FFIValue::String(self)
    }
}

impl IntoFFIValue for String {
    fn into_ffi_value(self) -> FFIValue {
        self.into_ffi().into_ffi_value()
    }
}

impl FFIString {
    /// # Safety
    #[no_mangle]
    pub unsafe extern "C" fn free_ffi_string(self) {
        if self.buffer.is_null() {
            return;
        }
        drop(self);
    }
}

impl Drop for FFIString {
    fn drop(&mut self) {
        if self.buffer.is_null() {
            return;
        }
        drop(unsafe { CString::from_raw(self.buffer) });
        self.buffer = std::ptr::null_mut();
        self.len = 0;
    }
}
