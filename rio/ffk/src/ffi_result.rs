use crate::ffi_convertor::FFIConvertor;
use crate::ffi_value::FFIValue;

#[repr(C)]
pub enum FFIResult {
    Ok(FFIValue),
    Err(FFIValue),
}

impl FFIResult {
    #[unsafe(no_mangle)]
    pub extern "C" fn is_oK(&self) -> bool {
        matches!(self, Self::Ok(_))
    }

    #[unsafe(no_mangle)]
    pub extern "C" fn is_err(&self) -> bool {
        matches!(self, Self::Err(_))
    }

    #[unsafe(no_mangle)]
    pub extern "C" fn unwrap(self) -> FFIValue {
        match self {
            Self::Ok(value) => value,
            Self::Err(error) => match error {
                FFIValue::String(error) => {
                    let error = unsafe { String::from_ffi(error) };
                    panic!("called `FFIResult::unwrap()` on an `Err` value: {}", error)
                }
                _ => panic!("Unexpected error type"),
            },
        }
    }

    #[unsafe(no_mangle)]
    pub extern "C" fn unwrap_err(self) -> FFIValue {
        match self {
            Self::Ok(value) => value,
            Self::Err(error) => error,
        }
    }
}
