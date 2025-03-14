use crate::container::HandleContainer;
use crate::io::ffi_write::{flush, write, write_all};
use ffk::ffi_convertor::ffi_bytes::FFIByteArray;
use ffk::ffi_handle::FFIHandle;
use ffk::ffi_result::FFIResult;
use std::io::Stderr;
use std::ops::DerefMut;
use std::sync::LazyLock;

static STDERR_CONTAINER: LazyLock<HandleContainer<Stderr>> = LazyLock::new(HandleContainer::new);

#[no_mangle]
pub extern "C" fn stderr_init() -> FFIResult {
    let stderr = std::io::stderr();
    let handle = STDERR_CONTAINER.create_handle(stderr, FFIHandle::stderr);
    FFIResult::Ok(handle.into())
}

#[no_mangle]
pub extern "C" fn stderr_write(stderr_handle: &FFIHandle, buffer: FFIByteArray) -> FFIResult {
    match STDERR_CONTAINER.get_mut(stderr_handle) {
        Some(mut stderr) => write(stderr.deref_mut(), buffer),
        None => FFIResult::handle_error(),
    }
}

#[no_mangle]
pub extern "C" fn stderr_write_all(stderr_handle: &FFIHandle, buffer: FFIByteArray) -> FFIResult {
    match STDERR_CONTAINER.get_mut(stderr_handle) {
        Some(mut stderr) => write_all(stderr.deref_mut(), buffer),
        None => FFIResult::handle_error(),
    }
}

#[no_mangle]
pub extern "C" fn stderr_flush(stderr_handle: &FFIHandle) -> FFIResult {
    match STDERR_CONTAINER.get_mut(stderr_handle) {
        Some(mut stderr) => flush(stderr.deref_mut()),
        None => FFIResult::handle_error(),
    }
}

#[no_mangle]
pub extern "C" fn free_stderr(stderr_handle: &FFIHandle) {
    STDERR_CONTAINER.free_handle(stderr_handle)
}
