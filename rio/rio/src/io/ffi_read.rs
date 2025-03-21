use ffk::ffi_result::FFIResult;
use ffk::ffi_value::ffi_bytes::FFIBytes;
use ffk::ffi_value::IntoFFIValue;
use std::io::Read;

pub fn read<R: Read>(reader: &mut R, buffer: &mut FFIBytes) -> FFIResult {
    let buf = unsafe { std::slice::from_raw_parts_mut(buffer.buffer, buffer.len) };
    match reader.read(buf) {
        Ok(size) => FFIResult::Ok(size.into()),
        Err(error) => error.into(),
    }
}

pub fn read_to_end<R: Read>(reader: &mut R) -> FFIResult {
    let mut buf = Vec::<u8>::new();
    match reader.read_to_end(&mut buf) {
        Ok(size) => {
            buf.truncate(size);
            FFIResult::Ok(buf.into_ffi_value())
        }
        Err(error) => error.into(),
    }
}

pub fn read_test<R>(reader: &mut R, buf: &mut Vec<u8>) -> FFIResult
where
    R: Read,
{
    match reader.read_to_end(buf) {
        Ok(size) => FFIResult::Ok(size.into()),
        Err(error) => error.into(),
    }
}
