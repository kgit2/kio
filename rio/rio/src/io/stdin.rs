use crate::ffi::ffi_convertor::ffi_byte_array::FFIByteArray;
use crate::ffi::ffi_convertor::FFIConvertor;
use crate::ffi::ffi_result::FFIResult;
use crate::ffi::ffi_value::FFIValue;
use crate::io::mut_borrow_from_ptr;
use std::io::Read;

#[no_mangle]
pub extern "C" fn stdin_init() -> FFIResult {
    let stdin_ptr = Box::into_raw(Box::new(std::io::stdin())) as *mut std::ffi::c_void;
    FFIResult::Ok(FFIValue::COpaquePointer(stdin_ptr))
}

#[no_mangle]
pub extern "C" fn stdin_read(
    stdin_ptr: *mut std::ffi::c_void,
    array_buffer: FFIByteArray,
) -> FFIResult {
    let stdin = mut_borrow_from_ptr::<std::io::Stdin>(stdin_ptr);
    let buf = unsafe { std::slice::from_raw_parts_mut(array_buffer.buffer, array_buffer.len) };
    FFIResult::from_io_result(stdin.read(buf), FFIValue::from)
}

#[no_mangle]
pub extern "C" fn stdin_read_to_end(stdin_ptr: *mut std::ffi::c_void) -> FFIResult {
    let stdin = mut_borrow_from_ptr::<std::io::Stdin>(stdin_ptr);
    let mut buf = Vec::<u8>::new();
    FFIResult::from_io_result(stdin.read_to_end(&mut buf), |_size| buf.into_ffi().into())
}

#[no_mangle]
pub extern "C" fn free_stdin(stdin_ptr: *mut std::ffi::c_void) {
    unsafe {
        drop(Box::from_raw(stdin_ptr as *mut std::io::Stdin));
    }
}
