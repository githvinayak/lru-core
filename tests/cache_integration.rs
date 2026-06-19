use lru_core::BasicCache;


#[cfg(test)]
mod tests{
    use lru_core::Cache;
    use super::BasicCache;
    #[test]
    fn test_integration_flow(){
        let fetched_value:i32 = 5;
        let mut cache = BasicCache::new();
        cache.put(String::from("test"), fetched_value).unwrap();
        let cached_value  =  cache.get(&"test".to_string());
        assert_eq!(cached_value,Some(&fetched_value));
    }
}