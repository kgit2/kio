use ffk::ffi_result::FFIResult;
use std::io::Read;

pub fn read<R: Read>(reader: &mut R, buf: &mut [u8]) -> FFIResult {
    match reader.read(buf) {
        Ok(size) => FFIResult::Ok(size.into()),
        Err(error) => error.into(),
    }
}

pub fn read_to_end<R: Read>(reader: &mut R, buf: &mut Vec<u8>) -> FFIResult {
    match reader.read_to_end(buf) {
        Ok(size) => FFIResult::Ok(size.into()),
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
