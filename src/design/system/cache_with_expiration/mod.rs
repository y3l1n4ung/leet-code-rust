/// [Custom Design] Cache with Expiration (TTL)
/// Difficulty: Medium
/// Topics: Hash Table, Design, Time
///
/// Implement a cache that stores key-value pairs where each pair has an expiration time (TTL).
/// After the TTL has passed, the key-value pair should be considered invalid and removed.
use std::collections::HashMap;
use std::time::{Duration, Instant};

pub struct ExpirableCache<K, V> {
    storage: HashMap<K, (V, Instant)>,
}

impl<K: std::hash::Hash + Eq, V> ExpirableCache<K, V> {
    pub fn new() -> Self {
        Self {
            storage: HashMap::new(),
        }
    }

    /// Set a key-value pair with a time-to-live (TTL) duration.
    pub fn set(&mut self, key: K, value: V, ttl: Duration) {
        todo!("Store the value with its expiration timestamp")
    }

    /// Get the value for a key if it exists and has not expired.
    pub fn get(&self, key: &K) -> Option<&V> {
        todo!("Return the value if it exists and is not expired")
    }

    /// Clean up all expired entries from the cache.
    pub fn cleanup(&mut self) {
        todo!("Remove all entries that have passed their expiration time")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;

    #[test]
    fn test_expirable_cache_basic() {
        let mut cache = ExpirableCache::new();
        cache.set("key1", "value1", Duration::from_millis(100));

        assert_eq!(cache.get(&"key1"), Some(&"value1"));

        thread::sleep(Duration::from_millis(150));
        assert_eq!(cache.get(&"key1"), None);
    }
}
