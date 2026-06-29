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
}