use crate::ffi_convertor::{FromFFI, IntoFFI};
use crate::ffi_value::{FFIValue, IntoFFIValue};

#[repr(C)]
#[derive(Debug)]
pub struct FFIVec {
    pub items: *mut FFIValue,
    pub len: usize,
    pub capacity: usize,
}

impl<T: IntoFFIValue> IntoFFI for Vec<T> {
    type FFIType = FFIVec;

    fn into_ffi(self) -> Self::FFIType {
        let len = self.len();
        let capacity = self.capacity();
        let items = self
            .into_iter()
            .map(|s| s.into_ffi_value())
            .collect::<Vec<_>>()
            .into_boxed_slice();
        let items = Box::into_raw(items) as *mut FFIValue;
        FFIVec {
            items,
            len,
            capacity,
        }
    }
}

impl FromFFI for FFIVec {
    type OriginRef = [FFIValue];
    type OriginOwned = Vec<FFIValue>;

    fn as_origin(&self) -> &Self::OriginRef {
        if self.items.is_null() {
            panic!("FFI buffer is null");
        }
        unsafe { std::slice::from_raw_parts(self.items, self.len) }
    }

    fn as_origin_mut(&mut self) -> &mut Self::OriginRef {
        if self.items.is_null() {
            panic!("FFI buffer is null");
        }
        unsafe { std::slice::from_raw_parts_mut(self.items, self.len) }
    }

    fn into_origin(mut self) -> Self::OriginOwned {
        if self.items.is_null() {
            panic!("FFI buffer is null");
        }
        let items = unsafe { Vec::from_raw_parts(self.items, self.len, self.capacity) };
        self.items = std::ptr::null_mut();
        self.len = 0;
        self.capacity = 0;
        items
    }
}

impl<T: IntoFFIValue> IntoFFIValue for Vec<T> {
    fn into_ffi_value(self) -> FFIValue {
        FFIValue::Vec(self.into_ffi())
    }
}

impl FFIVec {
    /// # Safety
    #[no_mangle]
    pub unsafe extern "C" fn free_ffi_vec(self) {
        if self.items.is_null() {
            return;
        }
        drop(self);
    }
}

impl Drop for FFIVec {
    fn drop(&mut self) {
        if self.items.is_null() {
            return;
        }
        drop(unsafe { Vec::from_raw_parts(self.items, self.len, self.capacity) });
        self.items = std::ptr::null_mut();
        self.len = 0;
        self.capacity = 0;
    }
}
