use crate::container::HandlerContainer;
use ffk::ffi_handle::FFIHandler;
use ffk::ffi_io::ffi_read::{read, read_to_end};
use ffk::ffi_result::FFIResult;
use ffk::ffi_value::ffi_bytes::FFIBytes;
use std::io::Stdin;
use std::ops::DerefMut;
use std::sync::LazyLock;

static STDIN_CONTAINER: LazyLock<HandlerContainer<Stdin>> = LazyLock::new(HandlerContainer::new);

#[no_mangle]
pub extern "C" fn stdin_init() -> FFIResult {
    let stdin = std::io::stdin();
    let handle = STDIN_CONTAINER.create_handler(stdin, FFIHandler::stdin);
    FFIResult::Ok(handle.into())
}

#[no_mangle]
pub extern "C" fn stdin_read(stdin_handle: &FFIHandler, buffer: &mut FFIBytes) -> FFIResult {
    match STDIN_CONTAINER.get_mut(stdin_handle) {
        Some(mut stdin) => read(stdin.deref_mut(), buffer),
        None => FFIResult::handle_error(),
    }
}

#[no_mangle]
pub extern "C" fn stdin_read_to_end(stdin_handle: &FFIHandler) -> FFIResult {
    match STDIN_CONTAINER.get_mut(stdin_handle) {
        Some(mut stdin) => read_to_end(stdin.deref_mut()),
        None => FFIResult::handle_error(),
    }
}

#[no_mangle]
pub extern "C" fn free_stdin(stdin_handle: &FFIHandler) {
    STDIN_CONTAINER.free_handle(stdin_handle)
}
