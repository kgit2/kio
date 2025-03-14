use crate::ffi_value::FFIValue;

#[repr(C)]
pub struct FFIHandle {
    pub index: u64,
    pub handle_type: FFIHandleType,
}

impl FFIHandle {
    pub fn stdin(index: u64) -> FFIHandle {
        FFIHandle {
            index,
            handle_type: FFIHandleType::Stdin,
        }
    }

    pub fn stdout(index: u64) -> FFIHandle {
        FFIHandle {
            index,
            handle_type: FFIHandleType::Stdout,
        }
    }

    pub fn stderr(index: u64) -> FFIHandle {
        FFIHandle {
            index,
            handle_type: FFIHandleType::Stderr,
        }
    }

    pub fn file(index: u64) -> FFIHandle {
        FFIHandle {
            index,
            handle_type: FFIHandleType::File,
        }
    }
}

#[repr(C)]
pub enum FFIHandleType {
    File,
    Stdin,
    Stdout,
    Stderr,
}

impl From<FFIHandle> for FFIValue {
    fn from(value: FFIHandle) -> Self {
        FFIValue::Handle(value)
    }
}
