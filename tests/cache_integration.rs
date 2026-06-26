use lru_core::cache::Cache;
use lru_core::basic_cache::BasicCache;
#[cfg(test)]
mod tests{
    use super::*;
    #[test]
    fn test_integration_flow(){
        let fetched_value:i32 = 5;
        let mut cache = BasicCache::new();
        cache.put(String::from("test"), fetched_value).unwrap();
        let cached_value  =  cache.get(&"test".to_string());
        assert_eq!(cached_value,Some(&fetched_value));
    }
    #[test]
    fn test_multiple_deletion(){
        let mut cache:BasicCache<String,i32>  = BasicCache::new();
        cache.put(String::from("one"), 1).unwrap();
        cache.put(String::from("two"), 2).unwrap();
        cache.put(String::from("three"), 3).unwrap();
        cache.put(String::from("four"), 4).unwrap();
        cache.put(String::from("five"), 5).unwrap();
        let _ = cache.remove(&"one".to_string());
        let _ =cache.remove(&"two".to_string());
        let _ =cache.remove(&"three".to_string());
        let _ =cache.remove(&"four".to_string());
        let _ = cache.remove(&"five".to_string());
        assert!(cache.is_empty());
    }

    #[test]
    fn test_overwrite_flow(){
        let mut cache:BasicCache<String,i32>  = BasicCache::new();
        cache.put(String::from("one"), 1).unwrap();
        cache.put(String::from("one"), 5).unwrap();
        assert_eq!(cache.get(&"one".to_string()),Some(&5));
    }
}