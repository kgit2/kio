use ffk::ffi_result::FFIResult;
use ffk::ffi_value::FFIValue;
use std::io::Read;

pub mod stderr;
pub mod stdin;
pub mod stdout;

pub fn mut_borrow_from_ptr<'a, T>(ptr: *mut std::ffi::c_void) -> &'a mut T {
    unsafe { &mut *(ptr as *mut T) }
}

pub fn io_ffi_result<T, F: FnOnce(T) -> FFIValue>(
    io_result: std::io::Result<T>,
    get_real_data: F,
) -> FFIResult {
    match io_result {
        Ok(value) => FFIResult::Ok(get_real_data(value)),
        Err(error) => error.into(),
    }
}

pub fn read<R: Read>(reader: &mut R, buf: &mut [u8]) -> std::io::Result<usize> {
    reader.read(buf)
}
