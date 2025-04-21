use std::{
    collections::HashMap,
    sync::RwLock,
    time::{Duration, Instant},
};

#[derive(Debug)]
pub struct Cache {
    data: RwLock<HashMap<String, (String, Instant)>>,
    ttl: Duration,
}

impl Cache {
    fn new(ttl_secs: u64) -> Self {
        Self {
            data: RwLock::new(HashMap::new()),
            ttl: Duration::from_secs(ttl_secs),
        }
    }

    fn set(&self, key: String, value: String) {
        let mut data = self.data.write().unwrap();
        data.insert(key, (value, Instant::now()));
    }

    fn get(&self, key: &str) -> Option<String> {
        let data = self.data.read().unwrap();
        if let Some((value, timestamp)) = data.get(key) {
            if timestamp.elapsed() < self.ttl {
                return Some(value.clone());
            }
        }
        None
    }

    fn cleanup(&self) {
        let mut data = self.data.write().unwrap();
        data.retain(|_, (_, timestamp)| timestamp.elapsed() < self.ttl);
    }
}

pub fn handle_test() {
    let data = Cache::new(60);
    data.set("hello".to_string(), "world".to_string());

    if let Some(v) = data.get("hello") {
        println!("hello: {:?}", v); // hello: "world"
    }

    data.cleanup();
    println!("data: {:?}", data);
    // data: Cache { data: RwLock { data: {"hello": ("world", Instant { tv_sec: 283563, tv_nsec: 317218864 })}, poisoned: false, .. }, ttl: 60s }
}
