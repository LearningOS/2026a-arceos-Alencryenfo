pub use alloc::collections::*;

use core::hash::{BuildHasherDefault, Hasher};

pub struct SimpleHasher(u64);

impl Default for SimpleHasher {
    fn default() -> Self {
        Self(0)
    }
}

impl Hasher for SimpleHasher {
    fn finish(&self) -> u64 {
        self.0
    }

    fn write(&mut self, bytes: &[u8]) {
        for b in bytes {
            self.0 = self.0.wrapping_mul(31).wrapping_add(*b as u64);
        }
    }
}

pub type SimpleHasherBuilder = BuildHasherDefault<SimpleHasher>;

pub struct HashMap<K, V> {
    inner: hashbrown::HashMap<K, V, SimpleHasherBuilder>,
}

impl<K, V> HashMap<K, V> {
    pub fn new() -> Self {
        Self {
            inner: hashbrown::HashMap::with_hasher(SimpleHasherBuilder::default()),
        }
    }
}

impl<K: Eq + core::hash::Hash, V> HashMap<K, V> {
    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        self.inner.insert(key, value)
    }

    pub fn get(&self, key: &K) -> Option<&V> {
        self.inner.get(key)
    }
}

impl<K, V> HashMap<K, V> {
    pub fn iter(&self) -> hashbrown::hash_map::Iter<'_, K, V> {
        self.inner.iter()
    }

    pub fn len(&self) -> usize {
        self.inner.len()
    }

    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }
}

impl<K, V> Default for HashMap<K, V> {
    fn default() -> Self {
        Self::new()
    }
}
