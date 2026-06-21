use crate::CacheError;

pub trait Cache<K, V> {
    fn put(&mut self, key: K, value: V)->Result<(),CacheError>;
    fn get(&self, key: &K) -> Option<&V>;
    fn remove(&mut self, key: &K) -> Result<V, CacheError>;
    fn len(&self) -> usize;
}