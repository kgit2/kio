use rio::ffi::ffi_convertor::ffi_byte_array::FFIByteArray;
use rio::ffi::ffi_convertor::FFIConvertor;
use rio::ffi::ffi_result::FFIResult;
use rio::ffi::ffi_value::FFIValue;
use rio::io::stdin::{stdin_init, stdin_read};

fn main() {
    let mut buf = [0u8; 1024];
    let array_buf = FFIByteArray {
        buffer: buf.as_mut_ptr(),
        len: buf.len(),
        capacity: buf.len(),
    };
    if let FFIResult::Ok(FFIValue::COpaquePointer(stdin_ptr)) = stdin_init() {
        unsafe {
            match stdin_read(stdin_ptr, array_buf) {
                FFIResult::Ok(FFIValue::ULong(size)) => {
                    println!("[rust]: stdin_read.size: {}", size);
                }
                FFIResult::Err(FFIValue::String(str)) => {
                    println!("{}", String::from_ffi(str))
                }
                _ => {}
            }
        }
    }
    // let size = stdin().read(&mut buf).unwrap();
    // println!("{}", String::from_utf8(buf[..size].to_vec()).unwrap());
}
