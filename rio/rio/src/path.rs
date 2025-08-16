use crate::container::HandlerContainer;
use crate::fs::metadata::metadata_init;
use crate::fs::read_dir::read_dir_init;
use ffk::ffi_convertor::FromFFI;
use ffk::ffi_handle::FFIHandler;
use ffk::ffi_result::FFIResult;
use ffk::ffi_value::ffi_string::FFIString;
use ffk::ffi_value::{FFIValue, IntoFFIValue};
use path_clean::PathClean;
use std::cmp::Ordering;
use std::hash::{DefaultHasher, Hash, Hasher};
use std::ops::Deref;
use std::path::PathBuf;
use std::ptr;
use std::sync::LazyLock;

static PATH_CONTAINER: LazyLock<HandlerContainer<PathBuf>> = LazyLock::new(HandlerContainer::new);

pub fn path_create(path: PathBuf) -> FFIHandler {
    PATH_CONTAINER.create_handler(path, FFIHandler::path)
}

#[no_mangle]
pub extern "C" fn path_init(buffer: &FFIString) -> FFIResult {
    let path = PathBuf::from(buffer.as_origin());
    let handle = PATH_CONTAINER.create_handler(path, FFIHandler::path);
    FFIResult::Ok(handle.into())
}

#[no_mangle]
pub extern "C" fn path_cwd() -> FFIResult {
    match std::env::current_dir() {
        Ok(path) => {
            let handle = PATH_CONTAINER.create_handler(path, FFIHandler::path);
            FFIResult::Ok(handle.into())
        }
        Err(e) => e.into(),
    }
}

#[no_mangle]
pub extern "C" fn path_push(handle: &FFIHandler, buffer: &FFIString) -> FFIResult {
    match PATH_CONTAINER.get_mut(handle) {
        None => FFIResult::handle_error(),
        Some(mut path) => {
            path.push(buffer.as_origin());
            FFIResult::Ok(FFIValue::Unit)
        }
    }
}

#[no_mangle]
pub extern "C" fn path_pop(handle: &FFIHandler) -> FFIResult {
    match PATH_CONTAINER.get_mut(handle) {
        None => FFIResult::handle_error(),
        Some(mut path) => {
            let result = path.pop();
            FFIResult::Ok(FFIValue::Boolean(result))
        }
    }
}

#[no_mangle]
pub extern "C" fn path_set_file_name(handle: &FFIHandler, buffer: &FFIString) -> FFIResult {
    match PATH_CONTAINER.get_mut(handle) {
        None => FFIResult::handle_error(),
        Some(mut path) => {
            path.set_file_name(buffer.as_origin());
            FFIResult::Ok(FFIValue::Unit)
        }
    }
}

#[no_mangle]
pub extern "C" fn path_set_extension(handle: &FFIHandler, buffer: &FFIString) -> FFIResult {
    match PATH_CONTAINER.get_mut(handle) {
        None => FFIResult::handle_error(),
        Some(mut path) => {
            path.set_extension(buffer.as_origin());
            FFIResult::Ok(FFIValue::Unit)
        }
    }
}

#[no_mangle]
pub extern "C" fn path_clear(handle: &FFIHandler) -> FFIResult {
    match PATH_CONTAINER.get_mut(handle) {
        None => FFIResult::handle_error(),
        Some(mut path) => {
            path.clear();
            FFIResult::Ok(FFIValue::Unit)
        }
    }
}

#[no_mangle]
pub extern "C" fn path_clone(handle: &FFIHandler) -> FFIResult {
    match PATH_CONTAINER.get(handle) {
        None => FFIResult::handle_error(),
        Some(path) => {
            let clone = path.clone();
            let handle = PATH_CONTAINER.create_handler(clone, FFIHandler::path);
            FFIResult::Ok(handle.into())
        }
    }
}

#[no_mangle]
pub extern "C" fn path_eq(handle: &FFIHandler, other: &FFIHandler) -> FFIResult {
    match (PATH_CONTAINER.get(handle), PATH_CONTAINER.get(other)) {
        (Some(path), Some(other)) => {
            FFIResult::Ok(FFIValue::Boolean(path.deref() == other.deref()))
        }
        _ => FFIResult::handle_error(),
    }
}

#[no_mangle]
pub extern "C" fn path_hash(handle: &FFIHandler) -> FFIResult {
    match PATH_CONTAINER.get(handle) {
        Some(path) => {
            let mut hasher = DefaultHasher::default();
            path.hash(&mut hasher);
            FFIResult::Ok(FFIValue::UInt64(hasher.finish()))
        }
        None => FFIResult::handle_error(),
    }
}

#[no_mangle]
pub extern "C" fn path_compare(handle: &FFIHandler, other: &FFIHandler) -> FFIResult {
    match (PATH_CONTAINER.get(handle), PATH_CONTAINER.get(other)) {
        (Some(path), Some(other)) => match path.cmp(other.deref()) {
            Ordering::Less => FFIResult::Ok(FFIValue::Int32(-1)),
            Ordering::Equal => FFIResult::Ok(FFIValue::Int32(0)),
            Ordering::Greater => FFIResult::Ok(FFIValue::Int32(1)),
        },
        _ => FFIResult::handle_error(),
    }
}

#[no_mangle]
pub extern "C" fn path_to_string(handle: &FFIHandler) -> FFIResult {
    match PATH_CONTAINER.get(handle) {
        Some(path) => match path.to_str() {
            Some(s) => FFIResult::Ok(s.to_string().into_ffi_value()),
            None => FFIResult::Err("Path contains invalid UTF-8".to_string().into_ffi_value()),
        },
        None => FFIResult::handle_error(),
    }
}

#[no_mangle]
pub extern "C" fn path_to_string_lossy(handle: &FFIHandler) -> FFIResult {
    match PATH_CONTAINER.get(handle) {
        Some(path) => FFIResult::Ok(path.to_string_lossy().to_string().into_ffi_value()),
        None => FFIResult::handle_error(),
    }
}

#[no_mangle]
pub extern "C" fn path_file_name(handle: &FFIHandler) -> FFIResult {
    match PATH_CONTAINER.get(handle) {
        Some(path) => match path.file_name() {
            Some(os_str) => {
                let file_name = os_str.to_string_lossy().to_string();
                FFIResult::Ok(file_name.into_ffi_value())
            }
            None => FFIResult::Ok("".to_string().into_ffi_value()),
        },
        None => FFIResult::handle_error(),
    }
}

#[no_mangle]
pub extern "C" fn path_extension(handle: &FFIHandler) -> FFIResult {
    match PATH_CONTAINER.get(handle) {
        Some(path) => match path.extension() {
            Some(os_str) => {
                let extension = os_str.to_string_lossy().to_string();
                FFIResult::Ok(extension.into_ffi_value())
            }
            None => FFIResult::Ok("".to_string().into_ffi_value()),
        },
        None => FFIResult::handle_error(),
    }
}

#[no_mangle]
pub extern "C" fn path_parent(handle: &FFIHandler) -> FFIResult {
    match PATH_CONTAINER.get(handle) {
        Some(path) => match path.parent() {
            Some(parent) => {
                let handle = PATH_CONTAINER.create_handler(parent.to_path_buf(), FFIHandler::path);
                FFIResult::Ok(handle.into())
            }
            None => FFIResult::Ok(FFIValue::Unit),
        },
        None => FFIResult::handle_error(),
    }
}

#[no_mangle]
pub extern "C" fn path_exists(handle: &FFIHandler) -> FFIResult {
    match PATH_CONTAINER.get(handle) {
        Some(path) => FFIResult::Ok(FFIValue::Boolean(path.exists())),
        None => FFIResult::handle_error(),
    }
}

#[no_mangle]
pub extern "C" fn path_is_file(handle: &FFIHandler) -> FFIResult {
    match PATH_CONTAINER.get(handle) {
        Some(path) => FFIResult::Ok(FFIValue::Boolean(path.is_file())),
        None => FFIResult::handle_error(),
    }
}

#[no_mangle]
pub extern "C" fn path_is_dir(handle: &FFIHandler) -> FFIResult {
    match PATH_CONTAINER.get(handle) {
        Some(path) => FFIResult::Ok(FFIValue::Boolean(path.is_dir())),
        None => FFIResult::handle_error(),
    }
}

#[no_mangle]
pub extern "C" fn path_is_absolute(handle: &FFIHandler) -> FFIResult {
    match PATH_CONTAINER.get(handle) {
        Some(path) => FFIResult::Ok(FFIValue::Boolean(path.is_absolute())),
        None => FFIResult::handle_error(),
    }
}

#[no_mangle]
pub extern "C" fn path_is_relative(handle: &FFIHandler) -> FFIResult {
    match PATH_CONTAINER.get(handle) {
        Some(path) => FFIResult::Ok(FFIValue::Boolean(path.is_relative())),
        None => FFIResult::handle_error(),
    }
}

#[no_mangle]
pub extern "C" fn path_normalize(handle: &FFIHandler) -> FFIResult {
    match PATH_CONTAINER.get(handle) {
        Some(path) => {
            let normalized = path.clean();
            let handle = PATH_CONTAINER.create_handler(normalized, FFIHandler::path);
            FFIResult::Ok(handle.into())
        }
        None => FFIResult::handle_error(),
    }
}

#[no_mangle]
pub extern "C" fn path_canonicalize(handle: &FFIHandler) -> FFIResult {
    match PATH_CONTAINER.get(handle) {
        Some(path) => match path.canonicalize() {
            Ok(canonicalized) => {
                let handle = PATH_CONTAINER.create_handler(canonicalized, FFIHandler::path);
                FFIResult::Ok(handle.into())
            }
            Err(e) => e.into(),
        },
        None => FFIResult::handle_error(),
    }
}

#[no_mangle]
pub extern "C" fn path_metadata(handle: &FFIHandler) -> FFIResult {
    match PATH_CONTAINER.get(handle) {
        Some(path) => match path.metadata() {
            Ok(metadata) => {
                let handle = metadata_init(metadata);
                FFIResult::Ok(handle.into())
            }
            Err(e) => e.into(),
        },
        None => FFIResult::handle_error(),
    }
}

#[no_mangle]
pub extern "C" fn path_read_dir(handle: &FFIHandler) -> FFIResult {
    match PATH_CONTAINER.get(handle) {
        Some(path) => match path.read_dir() {
            Ok(dir) => {
                let handle = read_dir_init(dir);
                FFIResult::Ok(handle.into())
            }
            Err(e) => e.into(),
        },
        None => FFIResult::handle_error(),
    }
}

#[no_mangle]
pub extern "C" fn path_components(handle: &FFIHandler) -> FFIResult {
    match PATH_CONTAINER.get(handle) {
        Some(path) => {
            let components = path
                .components()
                .map(|c| c.as_os_str().to_string_lossy().to_string());
            let components = components.collect::<Vec<_>>();
            FFIResult::Ok(components.into_ffi_value())
        }
        None => FFIResult::handle_error(),
    }
}

#[no_mangle]
pub extern "C" fn free_path(handle: &FFIHandler) {
    PATH_CONTAINER.free_handle(handle);
}
