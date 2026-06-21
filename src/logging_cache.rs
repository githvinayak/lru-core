use std::cell::Cell;
use std::collections::HashMap;
use std::hash::Hash;
use crate::cache::Cache;
use crate::errors::CacheError;

pub struct LoggingCache<K,V>
where
    K: Eq + Hash
{
    storage:HashMap<K,V>,
    access_count:Cell<u32>
}

impl<K,V> LoggingCache<K,V>
where
    K:Eq + Hash
{
    pub fn new() ->Self {
        LoggingCache{
            storage: HashMap::new(),
            access_count:Cell::new(0)
        }
    }
    pub fn get_count(&self)->u32{
        self.access_count.get()
    }
}

impl<K,V> Cache<K,V> for LoggingCache<K,V>
where
    K:Eq + Hash
{
    fn get(&self, key: &K) -> Option<&V> {
        self.access_count.set(self.access_count.get() + 1);
        self.storage.get(key)
    }

    fn put(&mut self, key: K, value: V)->Result<(),CacheError> {
        self.storage.insert(key, value);
        Ok(())
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