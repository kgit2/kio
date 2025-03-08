use crate::ffi::ffi_convertor::FFIConvertor;
use crate::ffi::ffi_value::FFIValue;

#[repr(C)]
pub enum FFIResult {
    Ok(FFIValue),
    Err(FFIValue),
}

impl FFIResult {
    #[no_mangle]
    pub extern "C" fn is_oK(&self) -> bool {
        matches!(self, Self::Ok(_))
    }

    #[no_mangle]
    pub extern "C" fn is_err(&self) -> bool {
        matches!(self, Self::Err(_))
    }

    #[no_mangle]
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

    #[no_mangle]
    pub extern "C" fn unwrap_err(self) -> FFIValue {
        match self {
            Self::Ok(value) => value,
            Self::Err(error) => error,
        }
    }

    pub fn from_io_result<T, F: FnOnce(T) -> FFIValue>(
        io_result: std::io::Result<T>,
        get_real_data: F,
    ) -> Self {
        match io_result {
            Ok(value) => Self::Ok(get_real_data(value)),
            Err(error) => error.into(),
        }
    }
}
