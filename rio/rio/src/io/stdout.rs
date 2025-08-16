use crate::container::HandlerContainer;
use ffk::ffi_handle::FFIHandler;
use ffk::ffi_io::ffi_write::{flush, write, write_all};
use ffk::ffi_result::FFIResult;
use ffk::ffi_value::ffi_bytes::FFIBytes;
use std::io::Stdout;
use std::ops::DerefMut;
use std::sync::LazyLock;

static STDOUT_CONTAINER: LazyLock<HandlerContainer<Stdout>> = LazyLock::new(HandlerContainer::new);

#[no_mangle]
pub extern "C" fn stdout_init() -> FFIResult {
    let stdout = std::io::stdout();
    let handle = STDOUT_CONTAINER.create_handler(stdout, FFIHandler::stdout);
    FFIResult::Ok(handle.into())
}

#[no_mangle]
pub extern "C" fn stdout_write(stdout_handle: &FFIHandler, buffer: &FFIBytes) -> FFIResult {
    match STDOUT_CONTAINER.get_mut(stdout_handle) {
        Some(mut stdout) => write(stdout.deref_mut(), buffer),
        None => FFIResult::handle_error(),
    }
}

#[no_mangle]
pub extern "C" fn stdout_write_all(stdout_handle: &FFIHandler, buffer: &FFIBytes) -> FFIResult {
    match STDOUT_CONTAINER.get_mut(stdout_handle) {
        Some(mut stdout) => write_all(stdout.deref_mut(), buffer),
        None => FFIResult::handle_error(),
    }
}

#[no_mangle]
pub extern "C" fn stdout_flush(stdout_handle: &FFIHandler) -> FFIResult {
    match STDOUT_CONTAINER.get_mut(stdout_handle) {
        Some(mut stdout) => flush(stdout.deref_mut()),
        None => FFIResult::handle_error(),
    }
}

#[no_mangle]
pub extern "C" fn free_stdout(stdout_handle: &FFIHandler) {
    STDOUT_CONTAINER.free_handle(stdout_handle)
}
