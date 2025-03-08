#[repr(C)]
pub struct ArrayBuffer {
    len: usize,
    buffer: *mut u8,
}

impl From<Vec<u8>> for ArrayBuffer {
    fn from(mut value: Vec<u8>) -> Self {
        let len = value.len();
        let buffer = value.as_mut_ptr();
        std::mem::forget(value);
        ArrayBuffer { len, buffer }
    }
}

#[no_mangle]
pub unsafe extern "C" fn free_array_buffer(array_buffer: ArrayBuffer) {
    drop(Vec::from_raw_parts(
        array_buffer.buffer,
        array_buffer.len,
        array_buffer.len,
    ));
}
