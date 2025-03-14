use crate::container::HandleContainer;
use ffk::ffi_handle::FFIHandle;
use std::fs::Metadata;
use std::sync::LazyLock;

static METADATA_CONTAINER: LazyLock<HandleContainer<Metadata>> =
    LazyLock::new(HandleContainer::new);

pub fn metadata_init(metadata: Metadata) -> FFIHandle {
    METADATA_CONTAINER.create_handle(metadata, FFIHandle::metadata)
}
