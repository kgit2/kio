use crate::ffi_convertor::{FromFFI, IntoFFI};
use crate::ffi_value::FFIValue;

#[repr(C)]
#[derive(Debug)]
pub enum FFIResult {
    Ok(FFIValue),
    Err(FFIValue),
    None,
}

impl FFIResult {
    pub fn none() -> Self {
        Self::None
    }

    pub fn handle_error() -> Self {
        Self::Err(FFIValue::String(
            "Cannot find raw pointer from handle".to_string().into_ffi(),
        ))
    }

    #[no_mangle]
    pub extern "C" fn is_ok(&self) -> bool {
        matches!(self, Self::Ok(_))
    }

    #[no_mangle]
    pub extern "C" fn is_err(&self) -> bool {
        matches!(self, Self::Err(_))
    }

    #[no_mangle]
    pub extern "C" fn result_unwrap(self) -> FFIValue {
        match self {
            Self::Ok(value) => value,
            Self::Err(error) => match &error {
                FFIValue::String(error) => {
                    let error = error.as_origin();
                    panic!("called `FFIResult::unwrap()` on an `Err` value: {}", error)
                }
                _ => panic!("Unexpected error type"),
            },
            Self::None => panic!("called `FFIResult::unwrap()` on a `None` value"),
        }
    }

    #[no_mangle]
    pub extern "C" fn unwrap_err(self) -> FFIValue {
        match self {
            Self::Ok(value) => panic!(
                "called `FFIResult::unwrap_err()` on an `Ok` value: {:?}",
                value
            ),
            Self::Err(error) => error,
            Self::None => panic!("called `FFIResult::unwrap_err()` on an `None` value"),
        }
    }
}
