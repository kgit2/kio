use crate::container::HandleContainer;
use crate::io::io_ffi_result;
use ffk::ffi_convertor::ffi_byte_array::FFIByteArray;
use ffk::ffi_handle::FFIHandle;
use ffk::ffi_result::FFIResult;
use ffk::ffi_value::FFIValue;
use std::io::{Stdout, Write};
use std::sync::LazyLock;

static STDOUT_CONTAINER: LazyLock<HandleContainer<Stdout>> = LazyLock::new(HandleContainer::new);

#[no_mangle]
pub extern "C" fn stdout_init() -> FFIResult {
    let stdout = std::io::stdout();
    let handle = STDOUT_CONTAINER.create_handle(stdout, FFIHandle::stdout);
    FFIResult::Ok(FFIValue::Handle(handle))
}

#[no_mangle]
pub extern "C" fn stdout_write(stdout_handle: FFIHandle, array_buffer: FFIByteArray) -> FFIResult {
    match STDOUT_CONTAINER.get_mut(&stdout_handle) {
        Some(mut stdout) => {
            let buf = unsafe { std::slice::from_raw_parts(array_buffer.buffer, array_buffer.len) };
            io_ffi_result(stdout.write(buf), FFIValue::from)
        }
        None => FFIResult::handle_error(),
    }
}

#[no_mangle]
pub extern "C" fn stdout_write_all(
    stdout_handle: FFIHandle,
    array_buffer: FFIByteArray,
) -> FFIResult {
    match STDOUT_CONTAINER.get_mut(&stdout_handle) {
        Some(mut stdout) => {
            let buf = unsafe { std::slice::from_raw_parts(array_buffer.buffer, array_buffer.len) };
            io_ffi_result(stdout.write_all(buf), |_| FFIValue::Unit)
        }
        None => FFIResult::handle_error(),
    }
}

#[no_mangle]
pub extern "C" fn stdout_flush(stdout_handle: FFIHandle) -> FFIResult {
    match STDOUT_CONTAINER.get_mut(&stdout_handle) {
        Some(mut stdout) => io_ffi_result(stdout.flush(), |_| FFIValue::Unit),
        None => FFIResult::handle_error(),
    }
}

#[no_mangle]
pub extern "C" fn free_stdout(stdout_handle: FFIHandle) {
    STDOUT_CONTAINER.free_handle(stdout_handle)
}
