use std::ffi::{c_char, CString};

pub fn string_into_cstring(str: String) -> CString {
    CString::new(str).expect("convert to CString failed")
}

pub fn string_into_c_char(str: String) -> *mut c_char {
    CString::new(str)
        .expect("convert to CString failed")
        .into_raw()
}

pub fn c_char_restore_string(ptr: *mut c_char) -> String {
    assert!(!ptr.is_null(), "char ptr is null");
    unsafe {
        CString::from_raw(ptr)
            .into_string()
            .expect("convert char ptr to CString failed")
    }
}

/// # Safety
/// Will drop the string
#[no_mangle]
pub unsafe extern "C" fn drop_c_char(string: *mut c_char) {
    if string.is_null() {
        return;
    }
    c_char_restore_string(string);
}
