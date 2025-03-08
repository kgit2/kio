use crate::byte_array::ArrayBuffer;
use crate::ffi::FFITransform;
use crate::ffi_result::FFIResult;
use crate::type_wrapper::TypeWrapper;
use std::io::Read;

#[no_mangle]
pub extern "C" fn stdin_init() -> FFIResult {
    let stdin_ptr = Box::into_raw(Box::new(std::io::stdin())) as *mut std::ffi::c_void;
    FFIResult::Ok(TypeWrapper::COpaquePointer(stdin_ptr))
}

/// # Safety
#[no_mangle]
pub unsafe extern "C" fn stdin_read(
    stdin_ptr: *mut std::ffi::c_void,
    array_buffer: ArrayBuffer,
) -> FFIResult {
    let stdin = &mut *(stdin_ptr as *mut std::io::Stdin);
    let buf = std::slice::from_raw_parts_mut(array_buffer.buffer, array_buffer.len);
    match stdin.read(buf) {
        Ok(size) => FFIResult::Ok(size.into()),
        Err(error) => FFIResult::Err(error.to_string().into_ffi().into()),
    }
}

/// # Safety
#[no_mangle]
pub unsafe extern "C" fn stdin_read_to_end(stdin_ptr: *mut std::ffi::c_void) -> FFIResult {
    let stdin = &mut *(stdin_ptr as *mut std::io::Stdin);
    let mut buf = Vec::<u8>::new();
    match stdin.read_to_end(&mut buf) {
        Ok(size) => FFIResult::Ok(TypeWrapper::Array(buf.into())),
        Err(error) => FFIResult::Err(error.to_string().into_ffi().into()),
    }
}

#[no_mangle]
pub extern "C" fn free_stdin(stdin_ptr: *mut std::ffi::c_void) {
    unsafe {
        drop(Box::from_raw(stdin_ptr as *mut std::io::Stdin));
    }
}
