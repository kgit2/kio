use crate::container::HandlerContainer;
use ffk::ffi_handle::FFIHandler;
use ffk::ffi_result::FFIResult;
use ffk::ffi_value::{FFIValue, IntoFFIValue};
use std::fs::Metadata;
use std::sync::LazyLock;

static METADATA_CONTAINER: LazyLock<HandlerContainer<Metadata>> =
    LazyLock::new(HandlerContainer::new);

pub fn metadata_init(metadata: Metadata) -> FFIHandler {
    METADATA_CONTAINER.create_handler(metadata, FFIHandler::metadata)
}

#[no_mangle]
pub extern "C" fn metadata_file_type(handle: &FFIHandler) -> FFIResult {
    match METADATA_CONTAINER.get(handle) {
        None => FFIResult::handle_error(),
        Some(metadata) => FFIResult::Ok(metadata.file_type().into_ffi_value()),
    }
}

#[no_mangle]
pub extern "C" fn metadata_is_dir(handle: &FFIHandler) -> FFIResult {
    match METADATA_CONTAINER.get(handle) {
        None => FFIResult::handle_error(),
        Some(metadata) => FFIResult::Ok(FFIValue::Boolean(metadata.is_dir())),
    }
}

#[no_mangle]
pub extern "C" fn metadata_is_file(handle: &FFIHandler) -> FFIResult {
    match METADATA_CONTAINER.get(handle) {
        None => FFIResult::handle_error(),
        Some(metadata) => FFIResult::Ok(FFIValue::Boolean(metadata.is_file())),
    }
}

#[no_mangle]
pub extern "C" fn metadata_is_symlink(handle: &FFIHandler) -> FFIResult {
    match METADATA_CONTAINER.get(handle) {
        None => FFIResult::handle_error(),
        Some(metadata) => FFIResult::Ok(FFIValue::Boolean(metadata.is_symlink())),
    }
}

#[no_mangle]
pub extern "C" fn metadata_len(handle: &FFIHandler) -> FFIResult {
    match METADATA_CONTAINER.get(handle) {
        None => FFIResult::handle_error(),
        Some(metadata) => FFIResult::Ok(FFIValue::UInt64(metadata.len())),
    }
}

#[no_mangle]
pub extern "C" fn metadata_readonly(handle: &FFIHandler) -> FFIResult {
    match METADATA_CONTAINER.get(handle) {
        None => FFIResult::handle_error(),
        Some(metadata) => FFIResult::Ok(FFIValue::Boolean(metadata.permissions().readonly())),
    }
}

#[no_mangle]
pub extern "C" fn metadata_set_readonly(handle: &FFIHandler, readonly: bool) -> FFIResult {
    match METADATA_CONTAINER.get_mut(handle) {
        None => FFIResult::handle_error(),
        Some(metadata) => {
            let mut permissions = metadata.permissions();
            permissions.set_readonly(readonly);
            FFIResult::Ok(FFIValue::Unit)
        }
    }
}

#[no_mangle]
#[cfg(unix)]
pub extern "C" fn metadata_mode(handle: &FFIHandler) -> FFIResult {
    use std::os::unix::fs::PermissionsExt;
    match METADATA_CONTAINER.get(handle) {
        None => FFIResult::handle_error(),
        Some(metadata) => FFIResult::Ok(FFIValue::UInt32(metadata.permissions().mode())),
    }
}

#[no_mangle]
#[cfg(unix)]
pub extern "C" fn metadata_set_mode(handle: &FFIHandler, mode: u32) -> FFIResult {
    use std::os::unix::fs::PermissionsExt;
    match METADATA_CONTAINER.get_mut(handle) {
        None => FFIResult::handle_error(),
        Some(metadata) => {
            let mut permissions = metadata.permissions();
            permissions.set_mode(mode);
            FFIResult::Ok(FFIValue::Unit)
        }
    }
}

#[no_mangle]
pub extern "C" fn free_metadata(handle: &FFIHandler) {
    METADATA_CONTAINER.free_handle(handle);
}
