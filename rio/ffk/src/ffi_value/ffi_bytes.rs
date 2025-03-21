use crate::ffi_convertor::{FromFFI, IntoFFI};
use crate::ffi_value::{FFIValue, IntoFFIValue};

#[repr(C)]
#[derive(Debug)]
pub struct FFIBytes {
    pub buffer: *mut u8,
    pub len: usize,
    pub capacity: usize,
}

impl From<FFIBytes> for String {
    fn from(value: FFIBytes) -> Self {
        unsafe {
            let slice = std::slice::from_raw_parts(value.buffer, value.len);
            String::from_utf8_lossy(slice).to_string()
        }
    }
}

impl IntoFFI for Vec<u8> {
    type FFIType = FFIBytes;

    fn into_ffi(mut self) -> Self::FFIType {
        let byte_array = FFIBytes {
            buffer: self.as_mut_ptr(),
            len: self.len(),
            capacity: self.capacity(),
        };
        std::mem::forget(self);
        byte_array
    }
}

impl FromFFI for FFIBytes {
    type OriginRef = [u8];
    type OriginOwned = Vec<u8>;

    fn as_origin(&self) -> &Self::OriginRef {
        if self.buffer.is_null() {
            panic!("FFI buffer is null");
        }
        unsafe { std::slice::from_raw_parts(self.buffer, self.len) }
    }

    fn as_origin_mut(&mut self) -> &mut Self::OriginRef {
        if self.buffer.is_null() {
            panic!("FFI buffer is null");
        }
        unsafe { std::slice::from_raw_parts_mut(self.buffer, self.len) }
    }

    fn into_origin(&mut self) -> Self::OriginOwned {
        if self.buffer.is_null() {
            panic!("FFI buffer is null");
        }
        unsafe { Vec::from_raw_parts(self.buffer, self.len, self.capacity) }
    }
}

impl FFIBytes {
    #[no_mangle]
    pub extern "C" fn free_ffi_bytes(&mut self) {
        if self.buffer.is_null() {
            return;
        }
        drop(self.into_origin());
    }
}

impl IntoFFIValue for FFIBytes {
    fn into_ffi_value(self) -> FFIValue {
        FFIValue::Bytes(self)
    }
}

impl IntoFFIValue for Vec<u8> {
    fn into_ffi_value(self) -> FFIValue {
        self.into_ffi().into_ffi_value()
    }
}

impl Drop for FFIBytes {
    fn drop(&mut self) {
        if !self.buffer.is_null() {
            unsafe { drop(Vec::from_raw_parts(self.buffer, self.len, self.capacity)) }
        }
    }
}
