use crate::ffi_convertor::FFIConvertor;
use std::ffi::CString;

impl FFIConvertor for String {
    type FFIType = *mut std::ffi::c_char;

    fn into_ffi(self) -> Self::FFIType {
        CString::new(self).unwrap().into_raw()
    }

    unsafe fn from_ffi_borrowed(ffi: Self::FFIType) -> Self {
        if ffi.is_null() {
            return String::new();
        }
        unsafe { CString::from_raw(ffi).to_str().unwrap().to_string() }
    }

    unsafe fn free(ffi: Self::FFIType) {
        if ffi.is_null() {
            return;
        }
        drop(unsafe { CString::from_raw(ffi) });
    }
}

/// # Safety
#[unsafe(no_mangle)]
pub unsafe extern "C" fn free_string(string_ptr: *mut std::ffi::c_char) {
    unsafe {
        String::free(string_ptr);
    }
}
