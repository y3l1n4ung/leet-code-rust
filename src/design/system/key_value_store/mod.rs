/// [System Design] Key-Value Store
/// Topic: Distributed Systems, Databases
/// Tags: CAPTheorem, Sharding, Replication
///
/// Link: https://bytebytego.com/courses/system-design-interview/design-a-key-value-store

use std::collections::HashMap;

/// A simple in-memory Key-Value store
pub struct KeyValueStore {
    storage: HashMap<String, String>,
}

impl KeyValueStore {
    pub fn new() -> Self {
        Self {
            storage: HashMap::new(),
        }
    }

    /// Sets the value for a key
    pub fn put(&mut self, key: String, value: String) {
        todo!("Implement the put operation")
    }

    /// Gets the value for a key
    pub fn get(&self, key: &str) -> Option<String> {
        todo!("Implement the get operation")
    }

    /// Deletes a key-value pair
    pub fn delete(&mut self, key: &str) -> bool {
        todo!("Implement the delete operation")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_kv_store_basic() {
        let mut store = KeyValueStore::new();
        
        store.put("name".to_string(), "Rust".to_string());
        assert_eq!(store.get("name"), Some("Rust".to_string()));
        
        store.put("name".to_string(), "Rustacean".to_string());
        assert_eq!(store.get("name"), Some("Rustacean".to_string()));
        
        assert!(store.delete("name"));
        assert_eq!(store.get("name"), None);
    }

    #[test]
    fn test_get_nonexistent() {
        let store = KeyValueStore::new();
        assert_eq!(store.get("unknown"), None);
    }
}
