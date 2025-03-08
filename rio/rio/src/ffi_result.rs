use crate::type_wrapper::TypeWrapper;

#[repr(C)]
pub enum FFIResult {
    Ok(TypeWrapper),
    Err(TypeWrapper),
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
    pub extern "C" fn unwrap(self) -> TypeWrapper {
        match self {
            Self::Ok(value) => value,
            Self::Err(error) => {
                panic!("called `FFIResult::unwrap()` on an `Err` value")
            }
        }
    }

    #[no_mangle]
    pub extern "C" fn unwrap_err(self) -> TypeWrapper {
        match self {
            Self::Ok(value) => value,
            Self::Err(error) => error,
        }
    }
}
