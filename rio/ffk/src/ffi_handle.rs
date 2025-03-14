use crate::ffi_value::FFIValue;

#[repr(C)]
pub struct FFIHandle {
    pub handle: u64,
    pub handle_type: FFIHandleType,
}

impl FFIHandle {
    pub fn stdin(handle: u64) -> FFIHandle {
        FFIHandle {
            handle,
            handle_type: FFIHandleType::Stdin,
        }
    }

    pub fn stdout(handle: u64) -> FFIHandle {
        FFIHandle {
            handle,
            handle_type: FFIHandleType::Stdout,
        }
    }

    pub fn stderr(handle: u64) -> FFIHandle {
        FFIHandle {
            handle,
            handle_type: FFIHandleType::Stderr,
        }
    }

    pub fn file(handle: u64) -> FFIHandle {
        FFIHandle {
            handle,
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
