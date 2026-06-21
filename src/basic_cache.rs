use std::collections::HashMap;
use std::hash::Hash;
use crate::cache::Cache;
use crate::errors::CacheError;

pub struct BasicCache<K, V>
where
    K: Eq + Hash
{
    storage: HashMap<K, V>,
}

impl<K, V> BasicCache<K, V>
where
    K: Eq + Hash
{
    pub fn new() -> Self {
        BasicCache {
            storage: HashMap::new(),
        }
    }

    pub fn contains_value(&self,value:&V)->bool
    where
        V : PartialEq
    {
        self.storage.values().any(|v| v == value)
    }
    pub fn count_where<F>(&self,predicate:F) -> usize
    where
        F: Fn(&V) -> bool
    {

        self.storage.values().filter(|v| predicate(v)).count()
    }

    pub fn is_empty(&self) -> bool {
        // your code
        self.storage.is_empty()
    }
    pub fn keys(&self) -> Vec<&K> {
        self.storage.keys().collect()
    }
    pub fn values(&self) -> Vec<&V> {
        self.storage.values().collect()
    }
    pub fn update_all<F>(&mut self, f: F)
    where
        F: Fn(&mut V),
    {
        self.storage.iter_mut().for_each(|(k,v)| f(v));
    }
}

impl<K, V> Cache<K, V> for BasicCache<K, V>
where
    K: Eq + Hash
{
    fn put(&mut self, key: K, value: V)->Result<(),CacheError> {
        self.storage.insert(key, value);
        Ok(())
    }
    fn get(&self, key: &K) -> Option<&V> {
        self.storage.get(key)
    }
    fn remove(&mut self, key: &K) -> Result<V, CacheError> {
        match self.storage.remove(key){
            Some(v)=>Ok(v),
            None=>Err(CacheError::NotFound)
        }
    }
    fn len(&self) -> usize {
        self.storage.len()
    }
}