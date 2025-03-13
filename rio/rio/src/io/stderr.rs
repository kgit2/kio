use crate::container::HandleContainer;
use crate::io::io_ffi_result;
use ffk::ffi_convertor::ffi_byte_array::FFIByteArray;
use ffk::ffi_handle::FFIHandle;
use ffk::ffi_result::FFIResult;
use ffk::ffi_value::FFIValue;
use std::io::Write;
use std::sync::LazyLock;

static STDERR_CONTAINER: LazyLock<HandleContainer<std::io::Stderr>> =
    LazyLock::new(HandleContainer::new);

#[no_mangle]
pub extern "C" fn stderr_init() -> FFIResult {
    let stderr = std::io::stderr();
    let handle = STDERR_CONTAINER.create_handle(stderr, FFIHandle::stderr);
    FFIResult::Ok(FFIValue::Handle(handle))
}

#[no_mangle]
pub extern "C" fn stderr_write(stderr_handle: FFIHandle, array_buffer: FFIByteArray) -> FFIResult {
    match STDERR_CONTAINER.get_mut(&stderr_handle) {
        Some(mut stderr) => {
            let buf = unsafe { std::slice::from_raw_parts(array_buffer.buffer, array_buffer.len) };
            io_ffi_result(stderr.write(buf), FFIValue::from)
        }
        None => FFIResult::handle_error(),
    }
}

#[no_mangle]
pub extern "C" fn stderr_write_all(
    stderr_handle: FFIHandle,
    array_buffer: FFIByteArray,
) -> FFIResult {
    match STDERR_CONTAINER.get_mut(&stderr_handle) {
        Some(mut stderr) => {
            let buf = unsafe { std::slice::from_raw_parts(array_buffer.buffer, array_buffer.len) };
            io_ffi_result(stderr.write_all(buf), |_| FFIValue::Unit)
        }
        None => FFIResult::handle_error(),
    }
}

#[no_mangle]
pub extern "C" fn stderr_flush(stderr_handle: FFIHandle) -> FFIResult {
    match STDERR_CONTAINER.get_mut(&stderr_handle) {
        Some(mut stderr) => io_ffi_result(stderr.flush(), |_| FFIValue::Unit),
        None => FFIResult::handle_error(),
    }
}

#[no_mangle]
pub extern "C" fn free_stderr(stderr_handle: FFIHandle) {
    STDERR_CONTAINER.free_handle(stderr_handle)
}
