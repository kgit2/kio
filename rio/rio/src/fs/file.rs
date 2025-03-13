use crossbeam_queue::SegQueue;
use ffk::ffi_convertor::FFIConvertor;
use ffk::ffi_handle::FFIHandle;
use ffk::ffi_result::FFIResult;
use ffk::ffi_value::FFIValue;
use lazy_static::lazy_static;
use std::fs::File;

lazy_static! {
    static ref FILE_CONTAINER: SegQueue<File> = SegQueue::new();
}

pub fn alloc_file_handle(file: File) -> FFIHandle {
    FILE_CONTAINER.push(file);
    FFIHandle::file((FILE_CONTAINER.len() - 1) as u64)
}

pub fn return_file_ffi_result(file: Result<std::fs::File, std::io::Error>) -> FFIResult {
    match file {
        Ok(file) => {
            let handle = alloc_file_handle(file);
            FFIResult::Ok(FFIValue::Handle(handle))
        }
        Err(error) => FFIResult::Err(error.to_string().into_ffi().into()),
    }
}

#[no_mangle]
pub extern "C" fn file_open(path: *mut std::ffi::c_char) -> FFIResult {
    let path = unsafe { String::from_ffi(path) };
    return_file_ffi_result(std::fs::File::open(path))
}

#[no_mangle]
pub extern "C" fn file_create(path: *mut std::ffi::c_char) -> FFIResult {
    let path = unsafe { String::from_ffi(path) };
    return_file_ffi_result(std::fs::File::create(path))
}

// #[no_mangle]
// pub extern "C" fn file_read()
