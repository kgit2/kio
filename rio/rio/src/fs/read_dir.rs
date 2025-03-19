use crate::container::HandleContainer;
use ffk::ffi_handle::FFIHandle;
use ffk::ffi_result::FFIResult;
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
pub extern "C" fn read_dir_nth(handle: &FFIHandle, n: u64) -> FFIResult {
    match READ_DIR_CONTAINER.get_mut(handle) {
        None => FFIResult::none(),
        Some(mut read_dir) => read_dir
            .nth(n as usize)
            .map_or(FFIResult::none(), |dir_entry| match dir_entry {
                Ok(dir_entry) => {
                    let handle = dir_entry_init(dir_entry);
                    FFIResult::Ok(handle.into())
                }
                Err(error) => error.into(),
            }),
    }
}
