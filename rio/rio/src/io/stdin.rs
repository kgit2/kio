use crate::container::HandleContainer;
use crate::io::ffi_read::{read, read_to_end};
use ffk::ffi_convertor::ffi_byte_array::FFIByteArray;
use ffk::ffi_handle::FFIHandle;
use ffk::ffi_result::FFIResult;
use std::io::Stdin;
use std::ops::DerefMut;
use std::sync::LazyLock;

static STDIN_CONTAINER: LazyLock<HandleContainer<Stdin>> = LazyLock::new(HandleContainer::new);

#[no_mangle]
pub extern "C" fn stdin_init() -> FFIResult {
    let stdin = std::io::stdin();
    let handle = STDIN_CONTAINER.create_handle(stdin, FFIHandle::stdin);
    FFIResult::Ok(handle.into())
}

#[no_mangle]
pub extern "C" fn stdin_read(stdin_handle: FFIHandle, array_buffer: FFIByteArray) -> FFIResult {
    match STDIN_CONTAINER.get_mut(&stdin_handle) {
        Some(mut stdin) => {
            let buf =
                unsafe { std::slice::from_raw_parts_mut(array_buffer.buffer, array_buffer.len) };
            read(stdin.deref_mut(), buf)
        }
        None => FFIResult::handle_error(),
    }
}

#[no_mangle]
pub extern "C" fn stdin_read_to_end(stdin_handle: FFIHandle) -> FFIResult {
    match STDIN_CONTAINER.get_mut(&stdin_handle) {
        Some(mut stdin) => {
            let mut buf = Vec::<u8>::new();
            read_to_end(stdin.deref_mut(), &mut buf)
        }
        None => FFIResult::handle_error(),
    }
}

#[no_mangle]
pub extern "C" fn free_stdin(stdin_handle: FFIHandle) {
    STDIN_CONTAINER.free_handle(stdin_handle)
}
