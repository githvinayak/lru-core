pub mod errors;
use errors::CacheError;
use std::{collections::HashMap, hash::Hash};
use std::cell::Cell;

pub trait Cache<K, V> {
    fn put(&mut self, key: K, value: V)->Result<(),CacheError>;
    fn get(&self, key: &K) -> Option<&V>;
    fn remove(&mut self, key: &K) -> Result<V, CacheError>;
    fn len(&self) -> usize;
}

pub struct BasicCache<K, V>
where
    K: Eq + Hash
{
    storage: HashMap<K, V>,
}

pub struct LoggingCache<K,V>
where
    K: Eq + Hash
{
    storage:HashMap<K,V>,
    access_count:Cell<u32>
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
    fn keys(&self) -> Vec<&K> {
        self.storage.keys().collect()
    }
    fn values(&self) -> Vec<&V> {
        self.storage.values().collect()
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_put_and_get() {
        let mut cache: BasicCache<String, i32> = BasicCache::new();
        cache.put("vini".to_string(), 5).unwrap();
        assert_eq!(cache.len(), 1);
        assert_eq!(cache.get(&"vini".to_string()), Some(&5));
    }

    #[test]
    fn test_get_missing_key() {
        let mut cache: BasicCache<String, i32> = BasicCache::new();
        let value = cache.get(&"vini".to_string());
        assert!(value.is_none())
    }

    #[test]
    fn test_remove() {
        let mut cache: BasicCache<String, i32> = BasicCache::new();
        cache.put("vini".to_string(), 5).unwrap();
        cache.remove(&"vini".to_string());
        assert_eq!(cache.len(), 0);
    }

    #[test]
    fn test_len() {
        let mut cache: BasicCache<String, i32> = BasicCache::new();
        cache.put("vini".to_string(), 5).unwrap();
        cache.put("vinii".to_string(), 4).unwrap();
        cache.put("viniii".to_string(), 3).unwrap();
        assert_eq!(cache.len(), 3);
    }

    #[test]
    fn test_overwrite() {
        let mut cache: BasicCache<String, i32> = BasicCache::new();
        cache.put("vini".to_string(), 5).unwrap();
        cache.put("vini".to_string(), 1).unwrap();
        let value = cache.get(&"vini".to_string()).unwrap();
        assert_eq!(value, &1);
    }

    #[test]
    fn test_empty_cache() {
        let mut cache: BasicCache<String, i32> = BasicCache::new();
        assert_eq!(cache.len(), 0);
        let value = cache.get(&"vini".to_string());
        assert!(value.is_none())
    }

    #[test]
    fn test_keys() {
        let mut cache: BasicCache<String, i32> = BasicCache::new();
        cache.put("a".to_string(), 5).unwrap();
        cache.put("b".to_string(), 1).unwrap();

        let mut keys = cache.keys();
        keys.sort();
        assert_eq!(keys,vec!["a","b"])
    }
    #[test]
     fn test_values() {
        let mut cache: BasicCache<String, i32> = BasicCache::new();
        cache.put("a".to_string(), 5).unwrap();
        cache.put("b".to_string(), 1).unwrap();
        let mut values = cache.values();
        values.sort();
        assert_eq!(values,vec![&1,&5])
    }

    #[test]
    fn test_contains_value() {
        let mut cache: BasicCache<String, i32> = BasicCache::new();
        cache.put("a".to_string(), 5).unwrap();
        cache.put("b".to_string(), 1).unwrap();
        let is_contain =  cache.contains_value(&5);
        assert!(is_contain);
    }
    #[test]
    fn test_count_where() {
        let mut cache: BasicCache<String, i32> = BasicCache::new();
        cache.put("a".to_string(), 5).unwrap();
        cache.put("b".to_string(), 1).unwrap();
        let count =  cache.count_where(|&x| x < 5);
        assert_eq!(count,1)
    }
    #[test]
    fn test_is_empty() {
        let cache: BasicCache<String, i32> = BasicCache::new();
        let is_empty =  cache.is_empty();
        assert!(is_empty)
    }
    #[test]
    fn test_logging_cache_access_count() {
        let mut cache: LoggingCache<String, i32> = LoggingCache::new();
        cache.put(String::from("a"),4).unwrap();
        cache.get(&"a".to_string());
        cache.get(&"a".to_string());
        cache.get(&"a".to_string());
        assert_eq!(cache.get_count(), 3);
    }
}