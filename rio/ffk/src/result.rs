use crate::ffi_result::FFIResult;
use crate::ffi_value::IntoFFIValue;

impl From<std::io::Error> for FFIResult {
    fn from(error: std::io::Error) -> Self {
        FFIResult::Err(error.to_string().into_ffi_value())
    }
}

// impl From<std::io::Error> for FFIOption {
//     fn from(value: Error) -> Self {
//         FFIOption::Some(value.to_string().into_ffi_value())
//     }
// }
