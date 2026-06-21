pub mod errors;
pub mod cache;
pub mod basic_cache;
pub mod logging_cache;

use cache::Cache;
use basic_cache::BasicCache;
use logging_cache::LoggingCache;
use errors::CacheError;

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
        let cache: BasicCache<String, i32> = BasicCache::new();
        let value = cache.get(&"vini".to_string());
        assert!(value.is_none())
    }

    #[test]
    fn test_remove() {
        let mut cache: BasicCache<String, i32> = BasicCache::new();
        cache.put("vini".to_string(), 5).unwrap();
        let value_ref = cache.remove(&"vini".to_string());
        println!("{:?}", value_ref);
        assert_eq!(cache.len(), 0);
        assert_eq!(value_ref.unwrap(), 5);
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
        let cache: BasicCache<String, i32> = BasicCache::new();
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
    fn test_empty_cache_contains_value() {
        let cache: BasicCache<String, i32> = BasicCache::new();
        let is_contain =  cache.contains_value(&5);
        println!("{:?}", is_contain);
        assert!(!is_contain);
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
    fn test_non_empty_cache_is_empty() {
        let mut cache: BasicCache<String, i32> = BasicCache::new();
        cache.put("a".to_string(), 5).unwrap();
        let is_empty =  cache.is_empty();
        assert!(!is_empty)
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
    #[test]
    fn test_update_all(){
        let mut cache: BasicCache<String, i32> = BasicCache::new();
        let add  = |v:&mut i32| *v = *v + 5;
        cache.put("a".to_string(), 5).unwrap();
        cache.put("b".to_string(), 6).unwrap();
        cache.put("c".to_string(), 7).unwrap();
        cache.put("d".to_string(), 8).unwrap();
        cache.update_all(add);
        assert_eq!(*cache.get(&"a".to_string()).unwrap(), 10);
        assert_eq!(*cache.get(&"b".to_string()).unwrap(), 11);
        assert_eq!(*cache.get(&"c".to_string()).unwrap(), 12);
        assert_eq!(*cache.get(&"d".to_string()).unwrap(), 13);
    }
    #[test]
    fn test_remove_missing(){
        let mut cache: BasicCache<String, i32> = BasicCache::new();
        assert_eq!(cache.remove(&"missing".to_string()),Err(CacheError::NotFound));
    }
}