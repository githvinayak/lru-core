use std::{collections::HashMap, hash::Hash};
pub trait Cache<K, V> {
    fn put(&mut self, key: K, value: V);
    fn get(&self, key: &K) -> Option<&V>;
    fn remove(&mut self, key: &K) -> Option<V>;
    fn len(&self) -> usize;
    fn keys(&self) -> Vec<&K>;
    fn values(&self) -> Vec<&V>;
}


#[derive(PartialEq)]
pub struct BasicCache<K, V>
where
    K: Eq + Hash,V: std::cmp::PartialEq
{
    storage: HashMap<K, V>,
}

impl<K, V> BasicCache<K, V>
where
    K: Eq + Hash, V: std::cmp::PartialEq
{
    pub fn new() -> Self {
        BasicCache {
            storage: HashMap::new(),
        }
    }

    pub fn contains_value(&self,value:&V)->bool
    where
     K : PartialEq
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
}

impl<K, V> Cache<K, V> for BasicCache<K, V>
where
    K: Eq + Hash ,V: std::cmp::PartialEq
{
    fn put(&mut self, key: K, value: V) {
        self.storage.insert(key, value);
    }
    fn get(&self, key: &K) -> Option<&V> {
        self.storage.get(key)
    }
    fn remove(&mut self, key: &K) -> Option<V> {
        self.storage.remove(key)
    }
    fn len(&self) -> usize {
        self.storage.len()
    }
    fn keys(&self) -> Vec<&K> {
        self.storage.keys().collect()
    }
    fn values(&self) -> Vec<&V> {
        self.storage.values().collect()
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_put_and_get() {
        let mut cache: BasicCache<String, i32> = BasicCache::new();
        cache.put("vini".to_string(), 5);
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
        cache.put("vini".to_string(), 5);
        cache.remove(&"vini".to_string());
        assert_eq!(cache.len(), 0);
    }

    #[test]
    fn test_len() {
        let mut cache: BasicCache<String, i32> = BasicCache::new();
        cache.put("vini".to_string(), 5);
        cache.put("vinii".to_string(), 4);
        cache.put("viniii".to_string(), 3);
        assert_eq!(cache.len(), 3);
    }

    #[test]
    fn test_overwrite() {
        let mut cache: BasicCache<String, i32> = BasicCache::new();
        cache.put("vini".to_string(), 5);
        cache.put("vini".to_string(), 1);
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
        cache.put("a".to_string(), 5);
        cache.put("b".to_string(), 1);

        let mut keys = cache.keys();
        keys.sort();
        assert_eq!(keys,vec!["a","b"])
    }
    #[test]
     fn test_values() {
        let mut cache: BasicCache<String, i32> = BasicCache::new();
        cache.put("a".to_string(), 5);
        cache.put("b".to_string(), 1);

        let values = cache.values();
        assert_eq!(values,vec![&5,&1])
    }

    #[test]
    fn test_contains_value() {
        let mut cache: BasicCache<String, i32> = BasicCache::new();
        cache.put("a".to_string(), 5);
        cache.put("b".to_string(), 1);
        let is_contain =  cache.contains_value(&5);
        assert!(is_contain);
    }
    #[test]
    fn test_count_where() {
        let mut cache: BasicCache<String, i32> = BasicCache::new();
        cache.put("a".to_string(), 5);
        cache.put("b".to_string(), 1);
        let count =  cache.count_where(|&x| x < 5);
        assert_eq!(count,1)
    }
    #[test]
    fn test_is_empty() {
        let mut cache: BasicCache<String, i32> = BasicCache::new();
        let is_empty =  cache.is_empty();
        assert!(is_empty)
    }
}
