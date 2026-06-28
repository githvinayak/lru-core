use crate::errors::CacheError;
use std::collections::HashMap;
use std::hash::Hash;

#[derive(Clone, Debug)]
pub struct Node<K, V> {
    prev: Option<usize>,
    next: Option<usize>,
    key: K,
    value: V,
}

pub struct LruCache<K, V>
where
    K: Eq + Hash + Clone,
{
    nodes: Vec<Node<K, V>>,
    map: HashMap<K, usize>,
    head: Option<usize>,
    tail: Option<usize>,
    capacity: usize,
}

impl<K, V> Node<K, V>
where
    K: Eq + Hash,
{
    fn new(key: K, value: V, prev: Option<usize>, next: Option<usize>) -> Self {
        Node {
            prev,
            next,
            key,
            value,
        }
    }
}
impl<K, V> LruCache<K, V>
where
    K: Eq + Hash + Clone + std::fmt::Debug,V:Clone
{
    pub fn new(capacity: usize) -> Self {
        LruCache {
            nodes: Vec::new(),
            map: HashMap::new(),
            head: None,
            tail: None,
            capacity,
        }
    }

   pub fn put(&mut self, key: K, value: V) -> Result<(), CacheError> {
        if let Some(index) = self.map.get(&key) {
            let index = *index;
            self.nodes[index].value = value;
             self.detach(index);
            self.attach_to_head(index);
            Ok(())
        } else {
            let index = self.nodes.len();
            if index == self.capacity{
                match self.tail {
                    Some(t) => {
                        let evict_key = self.nodes[t].key.clone();
                        self.detach(t);
                       self.map.remove(&evict_key);
                        self.map.insert(key.clone(), t);
                        let node: Node<K, V> = Node::new(key, value, None, None);
                         self.nodes[t] = node;
                        self.attach_to_head(t);
                         return Ok(());
                    },
                    None => ()
                }

            }
            self.map.insert(key.clone(), index);
            let node: Node<K, V> = Node::new(key, value, None, None);
            self.nodes.push(node);
            self.attach_to_head(index);
            Ok(())
        }
    }
   pub fn get(&mut self, key: &K) -> Option<&V> {
        let index = match self.map.get(key) {
            Some(i) => *i,
            None => return None,
        };
        self.detach(index);
        self.attach_to_head(index);
        Some(&self.nodes[index].value)
    }

    pub fn detach(&mut self, index: usize) {
        let prev = self.nodes[index].prev;
        let next = self.nodes[index].next;
        match prev {
            Some(p) => self.nodes[p].next = next,
            None => self.head = next,
        }
        match next {
            Some(n) => self.nodes[n].prev = prev,
            None => self.tail = prev,
        }
    }

    pub fn attach_to_head(&mut self, index: usize) {
        if self.head.is_none() {
            self.head = Some(index);
            self.tail = Some(index);
        } else {
            self.nodes[index].next = self.head;
            self.nodes[index].prev = None;
            match  self.head {
                Some(h)=> self.nodes[h].prev = Some(index),
                None => return
            }
            self.head = Some(index);
        }
    }
}


#[cfg(test)]

mod tests{
    use super::*;

    #[test]
    fn test_get_and_put(){
        let mut cache:LruCache<String,i32> = LruCache::new(5);
        let _ =cache.put("vini".to_string(),5);
        assert_eq!(cache.get(&"vini".to_string()),Some(&5));
    }

    #[test]
    fn test_eviction(){
        let mut cache:LruCache<String,i32> = LruCache::new(3);
        let _ =cache.put("a".to_string(),5);
        let _ =cache.put("b".to_string(),4);
        let _ = cache.put("c".to_string(),3);
        let _ = cache.put("d".to_string(),2);
        assert!(!cache.map.contains_key(&"a".to_string()));
        assert!(cache.map.contains_key(&"d".to_string()));
    }
    #[test]
    fn test_recency_order(){
        let mut cache:LruCache<String,i32> = LruCache::new(3);
        let _ =cache.put("a".to_string(),5);
        let _ =cache.put("b".to_string(),4);
        let _ = cache.put("c".to_string(),3);
        cache.get(&"a".to_string());
        cache.get(&"b".to_string());
        let head_idx = cache.head;
        let mid_idx = cache.nodes[head_idx.unwrap()].next;
        let tail_idx = cache.tail;

        assert_eq!(cache.nodes[head_idx.unwrap()].key,"b");
        assert_eq!(cache.nodes[mid_idx.unwrap()].key,"a");
        assert_eq!(cache.nodes[tail_idx.unwrap()].key,"c");
    }

    #[test]
    fn test_capacity_one(){
        let mut cache:LruCache<String,i32> = LruCache::new(1);
        let _ =cache.put("a".to_string(),5);
        let _ =cache.put("b".to_string(),4);
        assert_eq!(cache.head,cache.tail);
        assert!(!cache.map.contains_key("a"));
        assert!(cache.map.contains_key("b"))
    }

    #[test]
    fn repeated_keys(){
        let mut cache:LruCache<String,i32> = LruCache::new(5);
        let _ =cache.put("a".to_string(),5);
        let _ =cache.put("a".to_string(),4);
        assert_eq!(cache.nodes.len(),1)
}
    #[test]
    fn eviction_updates_order(){
        let mut cache:LruCache<String,i32> = LruCache::new(2);
        let _ =cache.put("a".to_string(),5);
        let _ =cache.put("b".to_string(),4);
        cache.get(&"a".to_string());
        let _ = cache.put("c".to_string(),3);
        assert!(!cache.map.contains_key("b"));
}
    #[test]
    fn test_get_on_empty_cache(){
        let mut cache:LruCache<String,i32> = LruCache::new(2);
        cache.get(&"a".to_string());
        assert!(cache.head.is_none());
    }
    #[test]
    fn test_single_element(){
        let mut cache:LruCache<String,i32> = LruCache::new(1);
        let _ =cache.put("a".to_string(),5);
        cache.get(&"a".to_string());
        let head_idx = cache.head;
        let tail_idx = cache.tail;
        assert_eq!(cache.nodes[head_idx.unwrap()].key,"a");
        assert_eq!(cache.nodes[tail_idx.unwrap()].key,"a");
    }

    #[test]
    fn test_repeated_evictions(){
        let mut cache:LruCache<String,i32> = LruCache::new(1);
        let _ =cache.put("a".to_string(),5);
        let _ =cache.put("b".to_string(),5);
        let _ =cache.put("c".to_string(),5);
        assert_eq!(cache.nodes.len(),1)
    }
}