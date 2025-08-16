#[repr(C)]
#[derive(Debug, Clone)]
pub struct FFIHandler {
    pub index: u64,
    pub handle_type: FFIHandlerType,
}

impl FFIHandler {
    pub fn stdin(index: u64) -> FFIHandler {
        FFIHandler {
            index,
            handle_type: FFIHandlerType::Stdin,
        }
    }

    pub fn stdout(index: u64) -> FFIHandler {
        FFIHandler {
            index,
            handle_type: FFIHandlerType::Stdout,
        }
    }

    pub fn stderr(index: u64) -> FFIHandler {
        FFIHandler {
            index,
            handle_type: FFIHandlerType::Stderr,
        }
    }

    pub fn file(index: u64) -> FFIHandler {
        FFIHandler {
            index,
            handle_type: FFIHandlerType::File,
        }
    }

    pub fn metadata(index: u64) -> FFIHandler {
        FFIHandler {
            index,
            handle_type: FFIHandlerType::Metadata,
        }
    }

    pub fn path(index: u64) -> FFIHandler {
        FFIHandler {
            index,
            handle_type: FFIHandlerType::Path,
        }
    }

    pub fn read_dir(index: u64) -> FFIHandler {
        FFIHandler {
            index,
            handle_type: FFIHandlerType::ReadDir,
        }
    }

    pub fn dir_entry(index: u64) -> FFIHandler {
        FFIHandler {
            index,
            handle_type: FFIHandlerType::DirEntry,
        }
    }
}

#[repr(C)]
#[derive(Debug, Clone)]
pub enum FFIHandlerType {
    Stdin,
    Stdout,
    Stderr,
    Path,
    File,
    Metadata,
    ReadDir,
    DirEntry,
}
