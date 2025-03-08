use rio::byte_array::ArrayBuffer;
use rio::ffi::FFITransform;
use rio::ffi_result::FFIResult;
use rio::io::stdin::{stdin_init, stdin_read};
use rio::type_wrapper::TypeWrapper;

fn main() {
    let mut buf = [0u8; 1024];
    let array_buf = ArrayBuffer {
        buffer: buf.as_mut_ptr(),
        len: buf.len(),
    };
    if let FFIResult::Ok(TypeWrapper::COpaquePointer(stdin_ptr)) = stdin_init() {
        unsafe {
            match stdin_read(stdin_ptr, array_buf) {
                FFIResult::Ok(TypeWrapper::ULong(size)) => {
                    println!("[rust]: stdin_read.size: {}", size);
                }
                FFIResult::Err(TypeWrapper::String(str)) => {
                    println!("{}", String::from_ffi_owned(str))
                }
                _ => {}
            }
        }
    }
    // let size = stdin().read(&mut buf).unwrap();
    // println!("{}", String::from_utf8(buf[..size].to_vec()).unwrap());
}
