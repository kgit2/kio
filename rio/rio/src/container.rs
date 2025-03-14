use dashmap::DashMap;
use ffk::ffi_handle::FFIHandle;
use std::hash::Hash;
use std::ops::{Deref, DerefMut};
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;

type DashRef<'a, K, V> = dashmap::mapref::one::Ref<'a, K, V>;
type DashRefMut<'a, K, V> = dashmap::mapref::one::RefMut<'a, K, V>;

pub struct HandleContainer<T> {
    container: DashMap<u64, T>,
    index: Arc<AtomicU64>,
}

impl<T> HandleContainer<T> {
    pub fn new() -> Self {
        Self {
            container: DashMap::new(),
            index: Arc::new(AtomicU64::new(0)),
        }
    }

    pub fn create_handle<F>(&self, value: T, handle_type: F) -> FFIHandle
    where
        F: FnOnce(u64) -> FFIHandle,
    {
        let index = self.index.fetch_add(1, Ordering::SeqCst);
        self.container.insert(index, value);
        handle_type(index)
    }

    pub fn free_handle(&self, handle: &FFIHandle) {
        self.container.remove(&handle.index);
    }

    #[allow(unused)]
    pub fn get(&self, handle: &FFIHandle) -> Option<FFIRef<u64, T>> {
        self.container.get(&handle.index).map(|v| FFIRef(v))
    }

    pub fn get_mut(&self, handle: &FFIHandle) -> Option<FFIRefMut<'_, u64, T>> {
        self.container.get_mut(&handle.index).map(|v| FFIRefMut(v))
    }
}

pub struct FFIRef<'a, K: Eq + Hash, V>(DashRef<'a, K, V>);

impl<'a, K: Eq + Hash, V> Deref for FFIRef<'a, K, V> {
    type Target = V;

    fn deref(&self) -> &Self::Target {
        self.0.deref()
    }
}

pub struct FFIRefMut<'a, K: Eq + Hash, V>(DashRefMut<'a, K, V>);

impl<'a, K: Eq + Hash, V> Deref for FFIRefMut<'a, K, V> {
    type Target = V;

    fn deref(&self) -> &Self::Target {
        self.0.deref()
    }
}

impl<'a, K: Eq + Hash, V> DerefMut for FFIRefMut<'a, K, V> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.0.deref_mut()
    }
}
