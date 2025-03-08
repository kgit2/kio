use std::ffi::{CStr, CString};

pub trait FFITransform {
    type Output;
    fn into_ffi(self) -> *mut Self::Output;

    /// # Safety
    unsafe fn from_ffi_owned(ffi: *mut Self::Output) -> Self;

    /// # Safety
    unsafe fn from_ffi_borrowed(ffi: *const Self::Output) -> Self;

    /// # Safety
    unsafe fn free(ffi: *mut Self::Output);
}

impl FFITransform for String {
    type Output = std::ffi::c_char;

    fn into_ffi(self) -> *mut Self::Output {
        CString::new(self).unwrap().into_raw()
    }

    unsafe fn from_ffi_owned(ffi: *mut Self::Output) -> Self {
        CString::from_raw(ffi).to_str().unwrap().to_string()
    }

    unsafe fn from_ffi_borrowed(ffi: *const Self::Output) -> Self {
        CStr::from_ptr(ffi).to_str().unwrap().to_string()
    }

    unsafe fn free(ffi: *mut Self::Output) {
        if ffi.is_null() {
            return;
        }
        drop(CString::from_raw(ffi));
    }
}

/// # Safety
#[no_mangle]
pub unsafe extern "C" fn free_string(string_ptr: *mut std::ffi::c_char) {
    String::free(string_ptr);
}
