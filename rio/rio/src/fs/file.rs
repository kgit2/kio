use ffk::ffi_convertor::FFIConvertor;
use ffk::ffi_result::FFIResult;
use ffk::ffi_value::FFIValue;

pub fn return_file_ffi_result(file: Result<std::fs::File, std::io::Error>) -> FFIResult {
    match file {
        Ok(file) => FFIResult::Ok(FFIValue::COpaquePointer(
            Box::into_raw(Box::new(file)) as *mut std::ffi::c_void
        )),
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
