use crate::byte_array::ArrayBuffer;
use crate::ffi::FFITransform;
use crate::ffi_result::FFIResult;
use crate::type_wrapper::TypeWrapper;
use std::io::Write;

#[no_mangle]
pub extern "C" fn stderr_init() -> FFIResult {
    let stderr_ptr = Box::into_raw(Box::new(std::io::stderr())) as *mut std::ffi::c_void;
    FFIResult::Ok(TypeWrapper::COpaquePointer(stderr_ptr))
}

#[no_mangle]
pub unsafe extern "C" fn stderr_write(
    stderr_ptr: *mut std::ffi::c_void,
    array_buffer: ArrayBuffer,
) -> FFIResult {
    let stderr = &mut *(stderr_ptr as *mut std::io::Stderr);
    let buf = std::slice::from_raw_parts(array_buffer.buffer, array_buffer.len);
    match stderr.write(buf) {
        Ok(size) => FFIResult::Ok(size.into()),
        Err(error) => FFIResult::Err(error.to_string().into_ffi().into()),
    }
}

#[no_mangle]
pub unsafe extern "C" fn stderr_write_all(
    stderr_ptr: *mut std::ffi::c_void,
    array_buffer: ArrayBuffer,
) -> FFIResult {
    let stderr = &mut *(stderr_ptr as *mut std::io::Stderr);
    let buf = std::slice::from_raw_parts(array_buffer.buffer, array_buffer.len);
    match stderr.write_all(buf) {
        Ok(_) => FFIResult::Ok(TypeWrapper::Unit),
        Err(error) => FFIResult::Err(error.to_string().into_ffi().into()),
    }
}

#[no_mangle]
pub unsafe extern "C" fn stderr_flush(stderr_ptr: *mut std::ffi::c_void) -> FFIResult {
    let stderr = &mut *(stderr_ptr as *mut std::io::Stderr);
    match stderr.flush() {
        Ok(_) => FFIResult::Ok(TypeWrapper::Unit),
        Err(error) => FFIResult::Err(error.to_string().into_ffi().into()),
    }
}

#[no_mangle]
pub unsafe extern "C" fn free_stderr(stderr_ptr: *mut std::ffi::c_void) {
    unsafe {
        drop(Box::from_raw(stderr_ptr as *mut std::io::Stderr));
    }
}
