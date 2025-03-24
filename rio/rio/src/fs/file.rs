use crate::container::HandleContainer;
use ffk::ffi_io::ffi_read::{read, read_to_end};
use ffk::ffi_io::ffi_write::{flush, write, write_all};
use ffk::ffi_convertor::FromFFI;
use ffk::ffi_handle::FFIHandle;
use ffk::ffi_result::FFIResult;
use ffk::ffi_value::ffi_bytes::FFIBytes;
use ffk::ffi_value::ffi_string::FFIString;
use ffk::ffi_value::{FFIValue, IntoFFIValue};
use std::fs::File;
use std::ops::DerefMut;
use std::sync::LazyLock;

static FILE_CONTAINER: LazyLock<HandleContainer<File>> = LazyLock::new(HandleContainer::new);

pub fn return_file_ffi_result(file: Result<File, std::io::Error>) -> FFIResult {
    match file {
        Ok(file) => {
            let handle = FILE_CONTAINER.create_handle(file, FFIHandle::file);
            FFIResult::Ok(FFIValue::Handle(handle))
        }
        Err(error) => FFIResult::Err(error.to_string().into_ffi_value()),
    }
}

#[no_mangle]
pub extern "C" fn file_open(path: &FFIString) -> FFIResult {
    let path = path.as_origin();
    return_file_ffi_result(File::open(path))
}

#[no_mangle]
pub extern "C" fn file_create(path: &FFIString) -> FFIResult {
    let path = path.as_origin();
    return_file_ffi_result(File::create(path))
}

#[no_mangle]
pub extern "C" fn file_read(file_handle: &FFIHandle, buffer: &mut FFIBytes) -> FFIResult {
    match FILE_CONTAINER.get_mut(file_handle) {
        Some(mut file) => read(file.deref_mut(), buffer),
        None => FFIResult::handle_error(),
    }
}

#[no_mangle]
pub extern "C" fn file_read_to_end(file_handle: &FFIHandle) -> FFIResult {
    match FILE_CONTAINER.get_mut(file_handle) {
        Some(mut file) => read_to_end(file.deref_mut()),
        None => FFIResult::handle_error(),
    }
}

#[no_mangle]
pub extern "C" fn file_write(file_handle: &FFIHandle, buffer: &FFIBytes) -> FFIResult {
    match FILE_CONTAINER.get_mut(file_handle) {
        Some(mut file) => write(file.deref_mut(), buffer),
        None => FFIResult::handle_error(),
    }
}

#[no_mangle]
pub extern "C" fn file_write_all(file_handle: &FFIHandle, buffer: &FFIBytes) -> FFIResult {
    match FILE_CONTAINER.get_mut(file_handle) {
        Some(mut file) => write_all(file.deref_mut(), buffer),
        None => FFIResult::handle_error(),
    }
}

#[no_mangle]
pub extern "C" fn file_flush(file_handle: &FFIHandle) -> FFIResult {
    match FILE_CONTAINER.get_mut(file_handle) {
        Some(mut file) => flush(file.deref_mut()),
        None => FFIResult::handle_error(),
    }
}

#[no_mangle]
pub extern "C" fn free_file(file_handle: &FFIHandle) {
    FILE_CONTAINER.free_handle(file_handle)
}
