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

#[repr(C)]
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

impl From<FFIHandle> for FFIValue {
    fn from(value: FFIHandle) -> Self {
        FFIValue::Handle(value)
    }
}
