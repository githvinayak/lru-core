use std::cell::RefCell;
use std::collections::HashMap;
use std::hash::Hash;
use std::rc::Rc;
use crate::errors::CacheError;

type NodePtr<K,V> = Rc<RefCell<Node<K,V>>>;
#[derive(Debug,PartialEq)]
pub struct Node<K,V>{
   pub prev: Option<NodePtr<K,V>>,
   pub next: Option<NodePtr<K,V>>,
   pub key: K,
   pub value: V,

}

pub struct LruCacheRc<K,V> {
    pub  head:Option<NodePtr<K,V>>,
    pub tail:Option<NodePtr<K,V>>,
    pub  map:HashMap<K,NodePtr<K,V>>,
    pub  capacity:usize,
    pub len:usize
}

impl<K, V> LruCacheRc<K, V>
where
    K: Eq + Hash + Clone + std::fmt::Debug,V:Clone
{
    pub fn new(capacity: usize) -> Self {
        LruCacheRc {
            len: 0,
            map: HashMap::new(),
            head: None,
            tail: None,
            capacity,
        }
    }

        pub fn detach(&mut self, node: NodePtr<K,V>) {
            let prev = node.borrow_mut().prev.clone();
            let next = node.borrow_mut().next.clone();
            match prev.clone() {
                Some(p) => p.borrow_mut().next = next.clone(),
                None => self.head = next.clone(),
            }
            match next {
                Some(n) => n.borrow_mut().prev = prev,
                None => self.tail = prev,
            }
        }
    pub fn attach_to_head(&mut self, node: NodePtr<K,V>) {
        if self.head.is_none() {
            self.head = Some(node.clone());
            self.tail = Some(node.clone());
        } else {
            node.borrow_mut().next = self.head.clone();
           node.borrow_mut().prev = None;
            if let Some(old_head) = self.head.clone() {
               old_head.borrow_mut().prev  = Some(node.clone())
            }
            self.head = Some(node);
        }
    }

    pub fn get(&mut self, key: &K) -> Option<V> {
        let index = match self.map.get(key) {
            Some(i) => i.clone(),
            None => return None,
        };
        self.detach(index.clone());
        self.attach_to_head(index.clone());
        Some(index.borrow().value.clone())
    }

    pub fn put(&mut self, key: K, value: V) -> Result<(), CacheError> {
        if let Some(index) = self.map.get(&key) {
            let index = index.clone();
            index.borrow_mut().value = value;
            self.detach(index.clone());
            self.attach_to_head(index.clone());
            Ok(())
        } else {
            let index = self.len;
            if index == self.capacity{
                match self.tail.clone() {
                    Some( t) => {
                        let evict_key = t.borrow().key.clone();
                        self.detach(t.clone());
                        self.map.remove(&evict_key);
                        let node: Node<K, V> = Node{key:key.clone(), value, prev:None, next:None};
                        let ptr: NodePtr<K,V> = Rc::new(RefCell::new(node));
                        self.map.insert(key.clone(), ptr.clone());
                        self.attach_to_head(ptr);
                        return Ok(());
                    },
                    None => ()
                }

            }
            let key_to_put = key.clone();
            let node: Node<K, V> = Node{key:key_to_put, value, prev:None, next:None};
            let ptr: NodePtr<K,V> = Rc::new(RefCell::new(node));
            self.map.insert(key.clone(), ptr.clone());
            self.len += 1;
            self.attach_to_head(ptr);
            Ok(())
        }
    }
}



#[cfg(test)]

mod tests{
    
    use super::*;

    #[test]
    fn test_get_and_put(){
        let mut cache:LruCacheRc<String,i32> = LruCacheRc::new(3);
        let _ =cache.put("vini".to_string(),5);
        assert_eq!(cache.get(&"vini".to_string()),Some(5));
    }

    #[test]
    fn test_eviction(){
        let mut cache:LruCacheRc<String,i32> = LruCacheRc::new(3);
        let _ =cache.put("a".to_string(),5);
        let _ =cache.put("b".to_string(),4);
        let _ = cache.put("c".to_string(),3);
        let _ = cache.put("d".to_string(),2);
        assert!(!cache.map.contains_key(&"a".to_string()));
        assert!(cache.map.contains_key(&"d".to_string()));
    }
    #[test]
    fn test_recency_order(){
        let mut cache:LruCacheRc<String,i32> = LruCacheRc::new(3);
        let _ =cache.put("a".to_string(),5);
        let _ =cache.put("b".to_string(),4);
        let _ = cache.put("c".to_string(),3);
        cache.get(&"a".to_string());
        cache.get(&"b".to_string());

        let mid_idx = cache.head.clone().unwrap().borrow().next.clone();
        let tail_idx = cache.tail;

        assert_eq!(cache.head.unwrap().borrow().key,"b");
        assert_eq!(mid_idx.unwrap().borrow().key,"a");
        assert_eq!(tail_idx.unwrap().borrow().key,"c");
    }

    #[test]
    fn test_capacity_one(){
        let mut cache:LruCacheRc<String,i32> = LruCacheRc::new(1);
        let _ =cache.put("a".to_string(),5);
        let _ =cache.put("b".to_string(),4);
        assert_eq!(cache.head,cache.tail);
        assert!(!cache.map.contains_key("a"));
        assert!(cache.map.contains_key("b"))
    }

    #[test]
    fn repeated_keys(){
        let mut cache:LruCacheRc<String,i32> = LruCacheRc::new(3);
        let _ =cache.put("a".to_string(),5);
        let _ =cache.put("a".to_string(),4);
        assert_eq!(cache.len,1)
    }
    #[test]
    fn eviction_updates_order(){
        let mut cache:LruCacheRc<String,i32> = LruCacheRc::new(2);
        let _ =cache.put("a".to_string(),5);
        let _ =cache.put("b".to_string(),4);
        cache.get(&"a".to_string());
        let _ = cache.put("c".to_string(),3);
        assert!(!cache.map.contains_key("b"));
    }
    #[test]
    fn test_get_on_empty_cache(){
        let mut cache:LruCacheRc<String,i32> = LruCacheRc::new(3);
        cache.get(&"a".to_string());
        assert!(cache.head.is_none());
    }
    #[test]
    fn test_single_element(){
        let mut cache:LruCacheRc<String,i32> = LruCacheRc::new(3);
        let _ =cache.put("a".to_string(),5);
        cache.get(&"a".to_string());

        assert_eq!(cache.head.unwrap().borrow().key,"a");
        assert_eq!(cache.tail.unwrap().borrow().key,"a");
    }

    #[test]
    fn test_repeated_evictions(){
        let mut cache:LruCacheRc<String,i32> = LruCacheRc::new(1);
        let _ =cache.put("a".to_string(),5);
        let _ =cache.put("b".to_string(),5);
        let _ =cache.put("c".to_string(),5);
        assert_eq!(cache.len,1)
    }
}