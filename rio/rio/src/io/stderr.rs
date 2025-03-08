use crate::ffi::ffi_convertor::ffi_byte_array::FFIByteArray;
use crate::ffi::ffi_result::FFIResult;
use crate::ffi::ffi_value::FFIValue;
use crate::io::mut_borrow_from_ptr;
use std::io::Write;

#[no_mangle]
pub extern "C" fn stderr_init() -> FFIResult {
    let stderr_ptr = Box::into_raw(Box::new(std::io::stderr())) as *mut std::ffi::c_void;
    FFIResult::Ok(FFIValue::COpaquePointer(stderr_ptr))
}

#[no_mangle]
pub extern "C" fn stderr_write(
    stderr_ptr: *mut std::ffi::c_void,
    array_buffer: FFIByteArray,
) -> FFIResult {
    let stderr = mut_borrow_from_ptr::<std::io::Stderr>(stderr_ptr);
    let buf = unsafe { std::slice::from_raw_parts(array_buffer.buffer, array_buffer.len) };
    FFIResult::from_io_result(stderr.write(buf), FFIValue::from)
}

#[no_mangle]
pub extern "C" fn stderr_write_all(
    stderr_ptr: *mut std::ffi::c_void,
    array_buffer: FFIByteArray,
) -> FFIResult {
    let stderr = mut_borrow_from_ptr::<std::io::Stderr>(stderr_ptr);
    let buf = unsafe { std::slice::from_raw_parts(array_buffer.buffer, array_buffer.len) };
    FFIResult::from_io_result(stderr.write_all(buf), |_| FFIValue::Unit)
}

#[no_mangle]
pub extern "C" fn stderr_flush(stderr_ptr: *mut std::ffi::c_void) -> FFIResult {
    let stderr = mut_borrow_from_ptr::<std::io::Stderr>(stderr_ptr);
    FFIResult::from_io_result(stderr.flush(), |_| FFIValue::Unit)
}

#[no_mangle]
pub extern "C" fn free_stderr(stderr_ptr: *mut std::ffi::c_void) {
    unsafe {
        drop(Box::from_raw(stderr_ptr as *mut std::io::Stderr));
    }
}
