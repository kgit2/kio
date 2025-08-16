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

    fn into_origin(self) -> Self::OriginOwned {
        if self.buffer.is_null() {
            panic!("FFI buffer is null");
        }
        unsafe { Vec::from_raw_parts(self.buffer, self.len, self.capacity) }
    }
}

impl FFIBytes {
    /// # Safety
    #[no_mangle]
    pub unsafe extern "C" fn free_ffi_bytes(self) {
        if self.buffer.is_null() {
            return;
        }
        drop(self);
    }
}

impl Drop for FFIBytes {
    fn drop(&mut self) {
        if self.buffer.is_null() {
            return;
        }
        drop(unsafe { Vec::from_raw_parts(self.buffer, self.len, self.capacity) });
        self.buffer = std::ptr::null_mut();
        self.len = 0;
        self.capacity = 0;
    }
}
