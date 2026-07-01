use std::cell::RefCell;
use std::collections::HashMap;
use std::hash::Hash;
use std::rc::Rc;
use crate::errors::CacheError;

type NodePtr<K,V> = Rc<RefCell<Node<K,V>>>;
struct Node<K,V>{
    prev: Option<NodePtr<K,V>>,
    next: Option<NodePtr<K,V>>,
    key: K,
    value: V,

}

struct LruCacheRc<K,V> {
    head:Option<NodePtr<K,V>>,
    tail:Option<NodePtr<K,V>>,
    map:HashMap<K,NodePtr<K,V>>,
    capacity:usize,
    len:usize
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

