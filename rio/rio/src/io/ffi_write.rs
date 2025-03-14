use ffk::ffi_result::FFIResult;
use ffk::ffi_value::FFIValue;
use std::io::Write;

pub fn write<W: Write>(writer: &mut W, buf: &[u8]) -> FFIResult {
    match writer.write(buf) {
        Ok(size) => FFIResult::Ok(size.into()),
        Err(error) => error.into(),
    }
}

pub fn write_all<W: Write>(writer: &mut W, buf: &[u8]) -> FFIResult {
    match writer.write_all(buf) {
        Ok(()) => FFIResult::Ok(FFIValue::Unit),
        Err(error) => error.into(),
    }
}

pub fn flush<W: Write>(writer: &mut W) -> FFIResult {
    match writer.flush() {
        Ok(()) => FFIResult::Ok(FFIValue::Unit),
        Err(error) => error.into(),
    }
}
