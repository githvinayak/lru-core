use std::collections::HashMap;
use std::hash::Hash;
struct Node<K,V>{
    prev : Option<usize>,
    next : Option<usize>,
    key:K,
    value:V
}

struct LruCache<K,V>
where
    K: Eq + Hash
{
    nodes:Vec<Node<K,V>>,
    map:HashMap<K,usize>,
    head:Option<usize>,
    tail:Option<usize>,
    capacity:usize
}

impl<K,V> LruCache<K,V>
where
    K: Eq + Hash
{
    fn new(capacity:usize) -> Self {
        LruCache{
            nodes:Vec::new(),
            map:HashMap::new(),
            head:None,
            tail:None,
            capacity,
        }
    }
}