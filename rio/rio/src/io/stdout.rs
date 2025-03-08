use crate::byte_array::ArrayBuffer;
use crate::ffi::FFITransform;
use crate::ffi_result::FFIResult;
use crate::type_wrapper::TypeWrapper;
use std::io::Write;

#[no_mangle]
pub extern "C" fn stdout_init() -> FFIResult {
    let stdout_ptr = Box::into_raw(Box::new(std::io::stdout())) as *mut std::ffi::c_void;
    FFIResult::Ok(TypeWrapper::COpaquePointer(stdout_ptr))
}

/// # Safety
#[no_mangle]
pub unsafe extern "C" fn stdout_write(
    stdout_ptr: *mut std::ffi::c_void,
    array_buffer: ArrayBuffer,
) -> FFIResult {
    let stdout = &mut *(stdout_ptr as *mut std::io::Stdout);
    let buf = std::slice::from_raw_parts(array_buffer.buffer, array_buffer.len);
    match stdout.write(buf) {
        Ok(size) => FFIResult::Ok(size.into()),
        Err(error) => FFIResult::Err(error.to_string().into_ffi().into()),
    }
}

/// # Safety
#[no_mangle]
pub unsafe extern "C" fn stdout_write_all(
    stdout_ptr: *mut std::ffi::c_void,
    array_buffer: ArrayBuffer,
) -> FFIResult {
    let stdout = &mut *(stdout_ptr as *mut std::io::Stdout);
    let buf = std::slice::from_raw_parts(array_buffer.buffer, array_buffer.len);
    match stdout.write_all(buf) {
        Ok(_) => FFIResult::Ok(TypeWrapper::Unit),
        Err(error) => FFIResult::Err(error.to_string().into_ffi().into()),
    }
}

/// # Safety
#[no_mangle]
pub unsafe extern "C" fn stdout_flush(stdout_ptr: *mut std::ffi::c_void) -> FFIResult {
    let stdout = &mut *(stdout_ptr as *mut std::io::Stdout);
    match stdout.flush() {
        Ok(_) => FFIResult::Ok(TypeWrapper::Unit),
        Err(error) => FFIResult::Err(error.to_string().into_ffi().into()),
    }
}

/// # Safety
#[no_mangle]
pub unsafe extern "C" fn free_stdout(stdout_ptr: *mut std::ffi::c_void) {
    unsafe {
        drop(Box::from_raw(stdout_ptr as *mut std::io::Stdout));
    }
}
