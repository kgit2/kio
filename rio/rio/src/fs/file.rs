use crate::container::HandleContainer;
use crate::io::io_ffi_result;
use ffk::ffi_convertor::ffi_byte_array::FFIByteArray;
use ffk::ffi_convertor::FFIConvertor;
use ffk::ffi_handle::FFIHandle;
use ffk::ffi_result::FFIResult;
use ffk::ffi_value::FFIValue;
use std::fs::File;
use std::io::{Read, Write};
use std::sync::LazyLock;

static FILE_CONTAINER: LazyLock<HandleContainer<File>> = LazyLock::new(HandleContainer::new);

pub fn return_file_ffi_result(file: Result<File, std::io::Error>) -> FFIResult {
    match file {
        Ok(file) => {
            let handle = FILE_CONTAINER.create_handle(file, FFIHandle::file);
            FFIResult::Ok(FFIValue::Handle(handle))
        }
        Err(error) => FFIResult::Err(error.to_string().into_ffi().into()),
    }
}

#[no_mangle]
pub extern "C" fn file_open(path: *mut std::ffi::c_char) -> FFIResult {
    let path = unsafe { String::from_ffi(path) };
    return_file_ffi_result(File::open(path))
}

#[no_mangle]
pub extern "C" fn file_create(path: *mut std::ffi::c_char) -> FFIResult {
    let path = unsafe { String::from_ffi(path) };
    return_file_ffi_result(File::create(path))
}

#[no_mangle]
pub extern "C" fn file_read(file_handle: FFIHandle, array_buffer: FFIByteArray) -> FFIResult {
    match FILE_CONTAINER.get_mut(&file_handle) {
        Some(mut file) => {
            let mut buf =
                unsafe { std::slice::from_raw_parts_mut(array_buffer.buffer, array_buffer.len) };
            io_ffi_result(file.read(&mut buf), FFIValue::from)
        }
        None => FFIResult::handle_error(),
    }
}

#[no_mangle]
pub extern "C" fn file_write(file_handle: FFIHandle, buffer: FFIByteArray) -> FFIResult {
    match FILE_CONTAINER.get_mut(&file_handle) {
        Some(mut file) => {
            let buf = unsafe { std::slice::from_raw_parts(buffer.buffer, buffer.len) };
            io_ffi_result(file.write(buf), FFIValue::from)
        }
        None => FFIResult::handle_error(),
    }
}
