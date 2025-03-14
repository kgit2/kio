pub mod ffi_bytes;
mod ffi_bytes_array;
pub mod ffi_string;

pub trait FFIConvertor {
    type FFIType;
    fn into_ffi(self) -> Self::FFIType;

    /// # Safety
    unsafe fn from_ffi_borrowed(ffi: Self::FFIType) -> Self;

    unsafe fn from_ffi_owned(ffi: Self::FFIType) -> Self;

    /// # Safety
    unsafe fn free(ffi: Self::FFIType);
}
