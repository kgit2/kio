use crate::container::HandleContainer;
use ffk::ffi_handle::FFIHandle;
use ffk::ffi_result::FFIResult;
use ffk::ffi_value::{FFIValue, IntoFFIValue};
use std::fs::Metadata;
use std::sync::LazyLock;

static METADATA_CONTAINER: LazyLock<HandleContainer<Metadata>> =
    LazyLock::new(HandleContainer::new);

pub fn metadata_init(metadata: Metadata) -> FFIHandle {
    METADATA_CONTAINER.create_handle(metadata, FFIHandle::metadata)
}

#[no_mangle]
pub extern "C" fn metadata_file_type(handle: &FFIHandle) -> FFIResult {
    match METADATA_CONTAINER.get(handle) {
        None => FFIResult::handle_error(),
        Some(metadata) => FFIResult::Ok(metadata.file_type().into_ffi_value()),
    }
}

#[no_mangle]
pub extern "C" fn metadata_is_dir(handle: &FFIHandle) -> FFIResult {
    match METADATA_CONTAINER.get(handle) {
        None => FFIResult::handle_error(),
        Some(metadata) => FFIResult::Ok(FFIValue::Boolean(metadata.is_dir())),
    }
}

#[no_mangle]
pub extern "C" fn metadata_is_file(handle: &FFIHandle) -> FFIResult {
    match METADATA_CONTAINER.get(handle) {
        None => FFIResult::handle_error(),
        Some(metadata) => FFIResult::Ok(FFIValue::Boolean(metadata.is_file())),
    }
}

#[no_mangle]
pub extern "C" fn metadata_is_symlink(handle: &FFIHandle) -> FFIResult {
    match METADATA_CONTAINER.get(handle) {
        None => FFIResult::handle_error(),
        Some(metadata) => FFIResult::Ok(FFIValue::Boolean(metadata.is_symlink())),
    }
}

#[no_mangle]
pub extern "C" fn metadata_len(handle: &FFIHandle) -> FFIResult {
    match METADATA_CONTAINER.get(handle) {
        None => FFIResult::handle_error(),
        Some(metadata) => FFIResult::Ok(FFIValue::UInt64(metadata.len())),
    }
}

#[no_mangle]
pub extern "C" fn metadata_read_only(handle: &FFIHandle) -> FFIResult {
    match METADATA_CONTAINER.get(handle) {
        None => FFIResult::handle_error(),
        Some(metadata) => FFIResult::Ok(FFIValue::Boolean(metadata.permissions().readonly())),
    }
}

#[no_mangle]
pub extern "C" fn metadata_set_read_only(handle: &FFIHandle, readonly: bool) -> FFIResult {
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
pub extern "C" fn metadata_mode(handle: &FFIHandle) -> FFIResult {
    use std::os::unix::fs::PermissionsExt;
    match METADATA_CONTAINER.get(handle) {
        None => FFIResult::handle_error(),
        Some(metadata) => FFIResult::Ok(FFIValue::UInt32(metadata.permissions().mode())),
    }
}

#[no_mangle]
#[cfg(unix)]
pub extern "C" fn metadata_set_mode(handle: &FFIHandle, mode: u32) -> FFIResult {
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
