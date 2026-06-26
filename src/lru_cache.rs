use crate::errors::CacheError;
use std::collections::HashMap;
use std::hash::Hash;

#[derive(Clone)]
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
            println!("index: {}, capacity: {}", index, self.capacity);
            if index == self.capacity{
                println!("tail: {:?}", self.tail);
                match self.tail {
                    Some(t) => {
                        println!("evicting index: {}", t);
                        println!("tail key: {:?}", self.nodes[t].key);
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
        println!("{:?}", cache.map);
        assert!(!cache.map.contains_key(&"a".to_string()));
        assert!(cache.map.contains_key(&"d".to_string()));
    }
}