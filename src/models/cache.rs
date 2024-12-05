use serde_json::Value;
use std::collections::HashMap;
use std::sync::Mutex;
use std::time::SystemTime;

#[derive(Clone)]
pub struct CacheEntry {
    pub data: Value,
    pub timestamp: SystemTime,
}

lazy_static::lazy_static! {
    pub static ref CACHE: Mutex<HashMap<String, CacheEntry>> = Mutex::new(HashMap::new());
}
