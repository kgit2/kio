use crate::ffi_value::{FFIValue, IntoFFIValue};

#[repr(C)]
#[derive(Debug, Clone)]
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

    pub fn metadata(index: u64) -> FFIHandle {
        FFIHandle {
            index,
            handle_type: FFIHandleType::Metadata,
        }
    }

    pub fn path(index: u64) -> FFIHandle {
        FFIHandle {
            index,
            handle_type: FFIHandleType::Path,
        }
    }

    pub fn read_dir(index: u64) -> FFIHandle {
        FFIHandle {
            index,
            handle_type: FFIHandleType::ReadDir,
        }
    }

    pub fn dir_entry(index: u64) -> FFIHandle {
        FFIHandle {
            index,
            handle_type: FFIHandleType::DirEntry,
        }
    }
}

impl IntoFFIValue for FFIHandle {
    fn into_ffi_value(self) -> FFIValue {
        FFIValue::Handle(self)
    }
}

#[repr(C)]
#[derive(Debug, Clone)]
pub enum FFIHandleType {
    Stdin,
    Stdout,
    Stderr,
    Path,
    File,
    Metadata,
    ReadDir,
    DirEntry,
}
