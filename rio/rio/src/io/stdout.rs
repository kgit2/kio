use crate::container::HandleContainer;
use crate::io::ffi_write::{flush, write, write_all};
use ffk::ffi_convertor::ffi_byte_array::FFIByteArray;
use ffk::ffi_handle::FFIHandle;
use ffk::ffi_result::FFIResult;
use std::io::Stdout;
use std::ops::DerefMut;
use std::sync::LazyLock;

static STDOUT_CONTAINER: LazyLock<HandleContainer<Stdout>> = LazyLock::new(HandleContainer::new);

#[no_mangle]
pub extern "C" fn stdout_init() -> FFIResult {
    let stdout = std::io::stdout();
    let handle = STDOUT_CONTAINER.create_handle(stdout, FFIHandle::stdout);
    FFIResult::Ok(handle.into())
}

#[no_mangle]
pub extern "C" fn stdout_write(stdout_handle: &FFIHandle, buffer: FFIByteArray) -> FFIResult {
    match STDOUT_CONTAINER.get_mut(stdout_handle) {
        Some(mut stdout) => write(stdout.deref_mut(), buffer),
        None => FFIResult::handle_error(),
    }
}

#[no_mangle]
pub extern "C" fn stdout_write_all(stdout_handle: &FFIHandle, buffer: FFIByteArray) -> FFIResult {
    match STDOUT_CONTAINER.get_mut(stdout_handle) {
        Some(mut stdout) => write_all(stdout.deref_mut(), buffer),
        None => FFIResult::handle_error(),
    }
}

#[no_mangle]
pub extern "C" fn stdout_flush(stdout_handle: &FFIHandle) -> FFIResult {
    match STDOUT_CONTAINER.get_mut(stdout_handle) {
        Some(mut stdout) => flush(stdout.deref_mut()),
        None => FFIResult::handle_error(),
    }
}

#[no_mangle]
pub extern "C" fn free_stdout(stdout_handle: &FFIHandle) {
    STDOUT_CONTAINER.free_handle(stdout_handle)
}
