use crate::io::{io_ffi_result, mut_borrow_from_ptr};
use ffk::ffi_convertor::ffi_byte_array::FFIByteArray;
use ffk::ffi_handle::FFIHandle;
use ffk::ffi_result::FFIResult;
use ffk::ffi_value::FFIValue;
use lazy_static::lazy_static;
use std::io::{Stdin, Write};
use std::sync::{Arc, Mutex};

lazy_static! {
    static ref STDOUT_CONTAINER: Arc<Mutex<Vec<Stdin>>> = Arc::new(Mutex::new(vec![]));
}

pub fn stdin_ffi_result(stdin: Stdin) -> FFIResult {
    let mut guard = crate::io::stdin::STDIN_CONTAINER
        .lock()
        .expect("failed to lock STDIN_CONTAINER");
    guard.push(stdin);
    let handle = FFIHandle::stdin((guard.len() - 1) as u64);
    FFIResult::Ok(FFIValue::Handle(handle))
}

#[no_mangle]
pub extern "C" fn stdout_init() -> FFIResult {
    let stdout_ptr = Box::into_raw(Box::new(std::io::stdout())) as *mut std::ffi::c_void;
    FFIResult::Ok(FFIValue::COpaquePointer(stdout_ptr))
}

#[no_mangle]
pub extern "C" fn stdout_write(
    stdout_ptr: *mut std::ffi::c_void,
    array_buffer: FFIByteArray,
) -> FFIResult {
    let stdout = mut_borrow_from_ptr::<std::io::Stdout>(stdout_ptr);
    let buf = unsafe { std::slice::from_raw_parts(array_buffer.buffer, array_buffer.len) };
    io_ffi_result(stdout.write(buf), FFIValue::from)
}

#[no_mangle]
pub extern "C" fn stdout_write_all(
    stdout_ptr: *mut std::ffi::c_void,
    array_buffer: FFIByteArray,
) -> FFIResult {
    let stdout = mut_borrow_from_ptr::<std::io::Stdout>(stdout_ptr);
    let buf = unsafe { std::slice::from_raw_parts(array_buffer.buffer, array_buffer.len) };
    io_ffi_result(stdout.write_all(buf), |_| FFIValue::Unit)
}

#[no_mangle]
pub extern "C" fn stdout_flush(stdout_ptr: *mut std::ffi::c_void) -> FFIResult {
    let stdout = mut_borrow_from_ptr::<std::io::Stdout>(stdout_ptr);
    io_ffi_result(stdout.flush(), |_| FFIValue::Unit)
}

#[no_mangle]
pub extern "C" fn free_stdout(stdout_ptr: *mut std::ffi::c_void) {
    unsafe {
        drop(Box::from_raw(stdout_ptr as *mut std::io::Stdout));
    }
}
