use crate::ffi::ffi_convertor::FFIConvertor;
use crate::ffi::ffi_result::FFIResult;

impl From<std::io::Error> for FFIResult {
    fn from(error: std::io::Error) -> Self {
        FFIResult::Err(error.to_string().into_ffi().into())
    }
}
