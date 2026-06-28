use std::cell::RefCell;
use std::collections::HashMap;
use std::hash::Hash;
use std::rc::Rc;

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
}