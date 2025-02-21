use std::collections::HashMap;
use std::collections::VecDeque;

pub struct LRUCache<K, V> {
    capacity: usize,
    map: HashMap<K, V>,
    keys: VecDeque<K>,
}

impl<K: Clone + Eq + std::hash::Hash, V> LRUCache<K, V> {
    pub fn new(capacity: usize) -> Self {
        Self {
            capacity,
            map: HashMap::with_capacity(capacity),
            keys: VecDeque::with_capacity(capacity),
        }
    }

    pub fn put(&mut self, key: K, value: V) {
        if self.map.len() >= self.capacity {
            if let Some(old_key) = self.keys.pop_back() {
                self.map.remove(&old_key);
            }
        }
        self.keys.push_front(key.clone());
        self.map.insert(key, value);
    }

    pub fn get(&mut self, key: &K) -> Option<&V> {
        if let Some(index) = self.keys.iter().position(|k| k == key) {
            let k = self.keys.remove(index).unwrap();
            self.keys.push_front(k);
        }
        self.map.get(key)
    }
}

pub fn handle_test() {
    let mut cache = LRUCache::new(10);
    cache.put(String::from("hello"), String::from("world"));

    if let Some(v) = cache.get(&String::from("hello")) {
        println!("hello: {:?}", v); // hello: "world"
    }

    cache.put(String::from("hello"), String::from("rust"));
    if let Some(v) = cache.get(&String::from("hello")) {
        println!("hello: {:?}", v); // hello: "rust"
    }
}
