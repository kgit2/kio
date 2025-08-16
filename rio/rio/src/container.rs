use dashmap::DashMap;
use ffk::ffi_handle::FFIHandler;
use std::hash::Hash;
use std::ops::{Deref, DerefMut};
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;

type DashRef<'a, K, V> = dashmap::mapref::one::Ref<'a, K, V>;
type DashRefMut<'a, K, V> = dashmap::mapref::one::RefMut<'a, K, V>;

pub struct HandlerContainer<T> {
    container: DashMap<u64, T>,
    index: Arc<AtomicU64>,
}

impl<T> Default for HandlerContainer<T> {
    fn default() -> Self {
        Self {
            container: DashMap::new(),
            index: Arc::new(AtomicU64::new(0)),
        }
    }
}

impl<T> HandlerContainer<T> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn size(&self) -> usize {
        self.container.len()
    }

    pub fn create_handler<F>(&self, value: T, handle_type: F) -> FFIHandler
    where
        F: FnOnce(u64) -> FFIHandler,
    {
        let index = self.index.fetch_add(1, Ordering::SeqCst);
        self.container.insert(index, value);
        handle_type(index)
    }

    pub fn remove_handler(&self, handle: &FFIHandler) -> Option<(u64, T)> {
        self.container.remove(&handle.index)
    }

    pub fn free_handle(&self, handle: &FFIHandler) {
        self.remove_handler(handle);
    }

    #[allow(unused)]
    pub fn get(&self, handle: &FFIHandler) -> Option<FFIRef<u64, T>> {
        self.container.get(&handle.index).map(|v| FFIRef(v))
    }

    pub fn get_mut(&self, handle: &FFIHandler) -> Option<FFIRefMut<'_, u64, T>> {
        self.container.get_mut(&handle.index).map(|v| FFIRefMut(v))
    }
}

impl<T> AsRef<DashMap<u64, T>> for HandlerContainer<T> {
    fn as_ref(&self) -> &DashMap<u64, T> {
        &self.container
    }
}

pub struct FFIRef<'a, K: Eq + Hash, V>(DashRef<'a, K, V>);

impl<K: Eq + Hash, V> Deref for FFIRef<'_, K, V> {
    type Target = V;

    fn deref(&self) -> &Self::Target {
        self.0.deref()
    }
}

pub struct FFIRefMut<'a, K: Eq + Hash, V>(DashRefMut<'a, K, V>);

impl<K: Eq + Hash, V> Deref for FFIRefMut<'_, K, V> {
    type Target = V;

    fn deref(&self) -> &Self::Target {
        self.0.deref()
    }
}

impl<K: Eq + Hash, V> DerefMut for FFIRefMut<'_, K, V> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.0.deref_mut()
    }
}
