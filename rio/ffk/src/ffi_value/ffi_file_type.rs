use crate::ffi_convertor::IntoFFI;
use crate::ffi_value::{FFIValue, IntoFFIValue};
use std::fs::FileType;

#[repr(C)]
#[derive(Debug)]
pub enum FFIFileType {
    IsFile,
    IsDirectory,
    IsSymlink,
    Other,
}

impl IntoFFI for FileType {
    type FFIType = FFIFileType;

    fn into_ffi(self) -> Self::FFIType {
        if self.is_file() {
            FFIFileType::IsFile
        } else if self.is_dir() {
            FFIFileType::IsDirectory
        } else if self.is_symlink() {
            FFIFileType::IsSymlink
        } else {
            FFIFileType::Other
        }
    }
}

impl IntoFFIValue for FileType {
    fn into_ffi_value(self) -> FFIValue {
        FFIValue::FileType(self.into_ffi())
    }
}
