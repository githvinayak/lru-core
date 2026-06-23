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

    fn detach(&mut self,index:usize){
      let prev = self.nodes[index].prev;
        let next = self.nodes[index].next;
        match prev {
           Some(p) => self.nodes[p].next = next,
           None => self.head = next
       }
        match next {
          Some(n)=>  self.nodes[n].prev = prev,
           None=> self.tail = prev
        }
    }

    fn attach_to_head(&mut self,index:usize){
        if self.nodes.is_empty(){
            self.head = Some(index);
            self.tail = Some(index);
        }else{
            self.nodes[index].next = self.head;
            self.nodes[index].prev = None;
            self.nodes[self.head.unwrap()].prev = Some(index);
            self.head = Some(index);

        }
    }

}