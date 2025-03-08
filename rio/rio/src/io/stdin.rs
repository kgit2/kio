use std::io::Read;

#[no_mangle]
pub extern "C" fn stdin_init() -> *mut std::ffi::c_void {
    Box::into_raw(Box::new(std::io::stdin())) as *mut std::ffi::c_void
}

#[no_mangle]
pub unsafe extern "C" fn stdin_read(
    stdin_ptr: *mut std::ffi::c_void,
    buffer: *mut u8,
    len: usize,
    read_size: *mut usize,
    read_error: *mut *mut std::ffi::c_char,
) {
    let stdin = &mut *(stdin_ptr as *mut std::io::Stdin);
    let buf = std::slice::from_raw_parts_mut(buffer, len);
    match stdin.read(buf) {
        Ok(size) => {
            *read_size = size;
            *read_error = std::ptr::null_mut();
        }
        Err(error) => {
            *read_size = 0;
            *read_error = std::ffi::CString::new(error.to_string())
                .unwrap()
                .into_raw();
        }
    }
}

#[no_mangle]
pub extern "C" fn stdin_destroy(stdin_ptr: *mut std::ffi::c_void) {
    unsafe {
        drop(Box::from_raw(stdin_ptr as *mut std::io::Stdin));
    }
}
