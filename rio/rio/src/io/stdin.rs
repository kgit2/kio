use crate::container::HandleContainer;
use crate::io::io_ffi_result;
use ffk::ffi_convertor::ffi_byte_array::FFIByteArray;
use ffk::ffi_convertor::FFIConvertor;
use ffk::ffi_handle::FFIHandle;
use ffk::ffi_result::FFIResult;
use ffk::ffi_value::FFIValue;
use std::io::{Read, Stdin};
use std::sync::LazyLock;

static STDIN_CONTAINER: LazyLock<HandleContainer<Stdin>> = LazyLock::new(HandleContainer::new);

#[no_mangle]
pub extern "C" fn stdin_init() -> FFIResult {
    let stdin = std::io::stdin();
    let handle = STDIN_CONTAINER.create_handle(stdin, FFIHandle::stdin);
    FFIResult::Ok(FFIValue::Handle(handle))
}

#[no_mangle]
pub extern "C" fn stdin_read(stdin_handle: FFIHandle, array_buffer: FFIByteArray) -> FFIResult {
    match STDIN_CONTAINER.get_mut(&stdin_handle) {
        Some(mut stdin_ref_mut) => {
            let buf =
                unsafe { std::slice::from_raw_parts_mut(array_buffer.buffer, array_buffer.len) };
            io_ffi_result(stdin_ref_mut.read(buf), FFIValue::from)
        }
        None => FFIResult::handle_error(),
    }
}

#[no_mangle]
pub extern "C" fn stdin_read_to_end(stdin_handle: FFIHandle) -> FFIResult {
    match STDIN_CONTAINER.get_mut(&stdin_handle) {
        Some(mut stdin_ref_mut) => {
            let mut buf = Vec::<u8>::new();
            io_ffi_result(stdin_ref_mut.read_to_end(&mut buf), |_| {
                buf.into_ffi().into()
            })
        }
        None => FFIResult::handle_error(),
    }
}

#[no_mangle]
pub extern "C" fn free_stdin(stdin_handle: FFIHandle) {
    STDIN_CONTAINER.free_handle(stdin_handle)
}
