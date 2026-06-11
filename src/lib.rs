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

impl<K, V> Cache<K, V> for BasicCache<K, V>
where
    K: Eq + Hash,
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
         assert_eq!(value,&1);
    }

    #[test]
    fn test_empty_cache() {
         let mut cache: BasicCache<String, i32> = BasicCache::new();
        assert_eq!(cache.len(), 0);
        let value = cache.get(&"vini".to_string());
        assert!(value.is_none())
    }
}
