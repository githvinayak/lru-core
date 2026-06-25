use crate::errors::CacheError;
use std::collections::HashMap;
use std::hash::Hash;
use std::path::MAIN_SEPARATOR;

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
    K: Eq + Hash + Clone,
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

    fn put(&mut self, key: K, value: V) -> Result<(), CacheError> {
        if let Some(index) = self.map.get(&key) {
            let index = *index;
            self.nodes[index].value = value;
             self.detach(index);
            self.attach_to_head(index);
            Ok(())
        } else {
            let index = self.nodes.len();
            self.map.insert(key.clone(), index);
            let node: Node<K, V> = Node::new(key, value, None, None);
            self.nodes.push(node);
            self.attach_to_head(index);
            Ok(())
        }
    }
    fn get(&mut self, key: &K) -> Option<&V> {
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
        if self.nodes.is_empty() {
            self.head = Some(index);
            self.tail = Some(index);
        } else {
            self.nodes[index].next = self.head;
            self.nodes[index].prev = None;
            self.nodes[self.head.unwrap()].prev = Some(index);
            self.head = Some(index);
        }
    }
}


