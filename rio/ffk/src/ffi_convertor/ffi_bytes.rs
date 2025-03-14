use crate::ffi_convertor::FFIConvertor;

#[repr(C)]
pub struct FFIByteArray {
    pub buffer: *mut u8,
    pub len: usize,
    pub capacity: usize,
}

impl From<FFIByteArray> for String {
    fn from(value: FFIByteArray) -> Self {
        unsafe {
            let slice = std::slice::from_raw_parts(value.buffer, value.len);
            String::from_utf8_lossy(slice).to_string()
        }
    }
}

impl FFIConvertor for Vec<u8> {
    type FFIType = FFIByteArray;

    fn into_ffi(mut self) -> Self::FFIType {
        let byte_array = FFIByteArray {
            buffer: self.as_mut_ptr(),
            len: self.len(),
            capacity: self.capacity(),
        };
        std::mem::forget(self);
        byte_array
    }

    unsafe fn from_ffi_borrowed(ffi: Self::FFIType) -> Self {
        unsafe { Vec::from_raw_parts(ffi.buffer, ffi.len, ffi.capacity) }
    }

    unsafe fn free(ffi: Self::FFIType) {
        unsafe { drop(Self::from_ffi_borrowed(ffi)) }
    }
}

impl From<Vec<u8>> for FFIByteArray {
    fn from(value: Vec<u8>) -> Self {
        value.into_ffi()
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn free_byte_array(buffer: FFIByteArray) {
    unsafe { Vec::<u8>::free(buffer) }
}
