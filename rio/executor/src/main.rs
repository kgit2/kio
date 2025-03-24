use rio::ffk::ffi_convertor::{FromFFI, IntoFFI};
use rio::ffk::ffi_result::FFIResult;
use rio::ffk::ffi_value::ffi_vec::FFIVec;
use rio::path::{path_components, path_init, path_to_string};

fn main() {
    let ffi_string = "a/b/c".to_string().into_ffi();
    match path_init(&ffi_string) {
        FFIResult::Ok(path) => {
            let path = path.to_handle();
            let ffi_string = path_to_string(&path).result_unwrap().to_string();
            for _ in 0..10 {
                let components = path_components(&path).result_unwrap().to_vec();
                unsafe { FFIVec::free_ffi_vec(components) };
            }
        }
        FFIResult::Err(_) => {}
        FFIResult::None => {}
    }
}
