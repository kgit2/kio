use crate::container::HandlerContainer;
use ffk::ffi_handle::FFIHandler;
use ffk::ffi_io::ffi_write::{flush, write, write_all};
use ffk::ffi_result::FFIResult;
use ffk::ffi_value::ffi_bytes::FFIBytes;
use std::io::Stderr;
use std::ops::DerefMut;
use std::sync::LazyLock;

static STDERR_CONTAINER: LazyLock<HandlerContainer<Stderr>> = LazyLock::new(HandlerContainer::new);

#[no_mangle]
pub extern "C" fn stderr_init() -> FFIResult {
    let stderr = std::io::stderr();
    let handle = STDERR_CONTAINER.create_handler(stderr, FFIHandler::stderr);
    FFIResult::Ok(handle.into())
}

#[no_mangle]
pub extern "C" fn stderr_write(stderr_handle: &FFIHandler, buffer: &FFIBytes) -> FFIResult {
    match STDERR_CONTAINER.get_mut(stderr_handle) {
        Some(mut stderr) => write(stderr.deref_mut(), buffer),
        None => FFIResult::handle_error(),
    }
}

#[no_mangle]
pub extern "C" fn stderr_write_all(stderr_handle: &FFIHandler, buffer: &FFIBytes) -> FFIResult {
    match STDERR_CONTAINER.get_mut(stderr_handle) {
        Some(mut stderr) => write_all(stderr.deref_mut(), buffer),
        None => FFIResult::handle_error(),
    }
}

#[no_mangle]
pub extern "C" fn stderr_flush(stderr_handle: &FFIHandler) -> FFIResult {
    match STDERR_CONTAINER.get_mut(stderr_handle) {
        Some(mut stderr) => flush(stderr.deref_mut()),
        None => FFIResult::handle_error(),
    }
}

#[no_mangle]
pub extern "C" fn free_stderr(stderr_handle: &FFIHandler) {
    STDERR_CONTAINER.free_handle(stderr_handle)
}
