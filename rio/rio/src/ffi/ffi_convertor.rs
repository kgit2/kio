pub mod ffi_byte_array;
pub mod ffi_string;

pub trait FFIConvertor {
    type FFIType;
    fn into_ffi(self) -> Self::FFIType;

    /// # Safety
    unsafe fn from_ffi(ffi: Self::FFIType) -> Self;

    /// # Safety
    unsafe fn free(ffi: Self::FFIType);
}
