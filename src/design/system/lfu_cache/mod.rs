/// [LeetCode 460] LFU Cache
/// Difficulty: Hard
/// Topics: Hash Table, Linked List, Design
///
/// Link: https://leetcode.com/problems/lfu-cache/

use std::collections::{HashMap, VecDeque};

pub struct LFUCache {
    capacity: usize,
    // Add internal state here
}

impl LFUCache {
    pub fn new(capacity: i32) -> Self {
        todo!("Initialize the LFU cache state with the given capacity")
    }

    pub fn get(&mut self, key: i32) -> i32 {
        todo!("Retrieve the value and update the frequency of the key")
    }

    pub fn put(&mut self, key: i32, value: i32) {
        todo!("Insert or update the value and frequency of the key, and evict if necessary")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lfu_basic() {
        let mut lfu = LFUCache::new(2);
        lfu.put(1, 1);
        lfu.put(2, 2);
        assert_eq!(lfu.get(1), 1);
        
        lfu.put(3, 3); // evicts key 2
        assert_eq!(lfu.get(2), -1);
        assert_eq!(lfu.get(3), 3);
        
        lfu.put(4, 4); // evicts key 1
        assert_eq!(lfu.get(1), -1);
        assert_eq!(lfu.get(3), 3);
        assert_eq!(lfu.get(4), 4);
    }

    #[test]
    fn test_zero_capacity() {
        let mut lfu = LFUCache::new(0);
        lfu.put(1, 1);
        assert_eq!(lfu.get(1), -1);
    }
}
