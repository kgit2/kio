use crate::container::HandleContainer;
use ffk::ffi_handle::FFIHandle;
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
