mod basic;
mod cache_RwLock;
mod custom_key;
mod lru_cache;

pub fn handle_test() {
    // basic::handle_test();
    // lru_cache::handle_test();
    // custom_key::handle_test();
    cache_RwLock::handle_test();
}
