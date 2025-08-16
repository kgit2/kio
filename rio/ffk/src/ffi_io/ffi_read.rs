use crate::ffi_result::FFIResult;
use crate::ffi_value::ffi_bytes::FFIBytes;
use crate::ffi_value::IntoFFIValue;
use std::io::{ErrorKind, Read};

pub fn read<R: Read>(reader: &mut R, buffer: &mut FFIBytes) -> FFIResult {
    let buf = unsafe { std::slice::from_raw_parts_mut(buffer.buffer, buffer.len) };
    loop {
        match reader.read(buf) {
            Ok(0) => return FFIResult::Ok((-1).into_ffi_value()),
            Ok(size) => return FFIResult::Ok(size.into_ffi_value()),
            Err(error) => match error.kind() {
                ErrorKind::WouldBlock => return FFIResult::Ok(0.into_ffi_value()),
                ErrorKind::Interrupted => continue,
                _ => return error.into(),
            },
        }
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
