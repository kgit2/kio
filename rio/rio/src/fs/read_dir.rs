use crate::container::HandleContainer;
use crate::fs::metadata::metadata_init;
use crate::path::path_create;
use ffk::ffi_convertor::IntoFFI;
use ffk::ffi_handle::FFIHandle;
use ffk::ffi_result::FFIResult;
use ffk::ffi_value::ffi_string::FFIString;
use ffk::ffi_value::IntoFFIValue;
use std::fs::{DirEntry, ReadDir};
use std::sync::LazyLock;

static READ_DIR_CONTAINER: LazyLock<HandleContainer<ReadDir>> = LazyLock::new(HandleContainer::new);
static DIR_ENTRY_CONTAINER: LazyLock<HandleContainer<DirEntry>> =
    LazyLock::new(HandleContainer::new);

pub fn read_dir_init(read_dir: ReadDir) -> FFIHandle {
    READ_DIR_CONTAINER.create_handle(read_dir, FFIHandle::read_dir)
}

pub fn dir_entry_init(dir_entry: DirEntry) -> FFIHandle {
    DIR_ENTRY_CONTAINER.create_handle(dir_entry, FFIHandle::dir_entry)
}

#[no_mangle]
pub extern "C" fn read_dir_next(handle: &FFIHandle) -> FFIResult {
    match READ_DIR_CONTAINER.get_mut(handle) {
        None => FFIResult::none(),
        Some(mut read_dir) => {
            read_dir
                .next()
                .map_or(FFIResult::none(), |dir_entry| match dir_entry {
                    Ok(dir_entry) => FFIResult::Ok(dir_entry_init(dir_entry).into()),
                    Err(error) => error.into(),
                })
        }
    }
}

#[no_mangle]
pub extern "C" fn read_dir_into_list(
    handle: &FFIHandle,
    error_callback: extern "C" fn(FFIString, *mut std::ffi::c_void),
    payload: *mut std::ffi::c_void,
) -> FFIResult {
    match READ_DIR_CONTAINER.remove_handle(handle) {
        None => FFIResult::none(),
        Some((_, read_dir)) => {
            let collection: Vec<FFIHandle> = read_dir
                .filter_map(|dir_entry| match dir_entry {
                    Ok(dir_entry) => dir_entry_init(dir_entry).into(),
                    Err(error) => {
                        let error_string = error.to_string().into_ffi();
                        error_callback(error_string, payload);
                        None
                    }
                })
                .collect();
            FFIResult::Ok(collection.into_ffi_value())
        }
    }
}

#[no_mangle]
pub extern "C" fn dir_entry_path(handle: &FFIHandle) -> FFIResult {
    match DIR_ENTRY_CONTAINER.get(handle) {
        None => FFIResult::none(),
        Some(dir_entry) => {
            let path = dir_entry.path();
            let handle = path_create(path);
            FFIResult::Ok(handle.into_ffi_value())
        }
    }
}

#[no_mangle]
pub extern "C" fn dir_entry_metadata(handle: &FFIHandle) -> FFIResult {
    match DIR_ENTRY_CONTAINER.get(handle) {
        None => FFIResult::none(),
        Some(dir_entry) => match dir_entry.metadata() {
            Ok(metadata) => {
                let handle = metadata_init(metadata);
                FFIResult::Ok(handle.into_ffi_value())
            }
            Err(error) => error.into(),
        },
    }
}

#[no_mangle]
pub extern "C" fn dir_entry_file_type(handle: &FFIHandle) -> FFIResult {
    match DIR_ENTRY_CONTAINER.get(handle) {
        None => FFIResult::none(),
        Some(dir_entry) => match dir_entry.file_type() {
            Ok(file_type) => FFIResult::Ok(file_type.into_ffi_value()),
            Err(error) => error.into(),
        },
    }
}

#[no_mangle]
pub extern "C" fn dir_entry_file_name(handle: &FFIHandle) -> FFIResult {
    match DIR_ENTRY_CONTAINER.get(handle) {
        None => FFIResult::none(),
        Some(dir_entry) => {
            let file_name = dir_entry.file_name().to_string_lossy().to_string();
            FFIResult::Ok(file_name.into_ffi_value())
        }
    }
}

#[no_mangle]
pub extern "C" fn read_dir_size() -> u64 {
    READ_DIR_CONTAINER.size() as u64
}

#[no_mangle]
pub extern "C" fn dir_entry_size() -> u64 {
    DIR_ENTRY_CONTAINER.size() as u64
}

#[no_mangle]
pub extern "C" fn free_read_dir(handle: &FFIHandle) {
    READ_DIR_CONTAINER.free_handle(handle);
}

#[no_mangle]
pub extern "C" fn free_dir_entry(handle: &FFIHandle) {
    DIR_ENTRY_CONTAINER.free_handle(handle);
}
