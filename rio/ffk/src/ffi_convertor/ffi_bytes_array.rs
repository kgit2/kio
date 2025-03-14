use crate::ffi_convertor::ffi_bytes::FFIByteArray;
use crate::ffi_convertor::FFIConvertor;

#[repr(C)]
pub struct FFIStringList {
    pub items: *mut *mut std::ffi::c_char,
    pub len: usize,
    pub capacity: usize,
}

impl FFIConvertor for Vec<String> {
    type FFIType = FFIStringList;

    fn into_ffi(self) -> Self::FFIType {
        let mut items = self.into_iter().map(|s| s.into_ffi()).collect::<Vec<_>>();
        let list = FFIStringList {
            items: items.as_mut_ptr(),
            len: items.len(),
            capacity: items.capacity(),
        };
        std::mem::forget(items);
        list
    }

    unsafe fn from_ffi_borrowed(ffi: Self::FFIType) -> Self {
        let items = std::slice::from_raw_parts(ffi.items, ffi.len);
        items
            .iter()
            .map(|&item| unsafe { String::from_ffi_borrowed(item) })
            .collect()
    }

    unsafe fn free(ffi: Self::FFIType) {
        todo!()
    }
}
