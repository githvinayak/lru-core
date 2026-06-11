use std::{collections::HashMap, hash::Hash};

pub trait Cache<K, V> {
    fn put(&mut self, key: K, value: V);
    fn get(&self, key: &K) -> Option<&V>;
    fn remove(&mut self, key: &K) -> Option<V>;
    fn len(&self) -> usize;
}

pub struct BasicCache<K, V>
where
    K: Eq + Hash,
{
    storage: HashMap<K, V>,
}

impl<K, V> BasicCache<K, V>
where
    K: Eq + Hash,
{
    pub fn new() -> Self {
        BasicCache {
            storage: HashMap::new(),
        }
    }
}