/// [146] LRU Cache
/// Difficulty: Hard
/// Topics: Hash Table, Linked List, Design, Doubly-Linked List
/// Tags: Blind75, NeetCode150
///
/// Design a data structure that follows the constraints of a Least Recently Used (LRU) cache.
/// Implement the LRUCache class:
/// - LRUCache(int capacity) Initialize the LRU cache with positive size capacity.
/// - int get(int key) Return the value of the key if the key exists, otherwise return -1.
/// - void put(int key, int value) Update the value of the key if the key exists. Otherwise, add the key-value pair to the cache. If the number of keys exceeds the capacity from this operation, evict the least recently used key.
/// The functions get and put must each run in O(1) average time complexity.
///
/// Link: https://leetcode.com/problems/lru-cache/

struct Node {
    key: i32,
    value: i32,
    prev: Option<Box<Node>>,
}
struct LRUCache {
    todo: (),
}

impl LRUCache {
    pub fn new(capacity: i32) -> Self {
        todo!()
    }

    pub fn get(&self, key: i32) -> i32 {
        todo!()
    }

    pub fn put(&self, key: i32, value: i32) {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let lru = LRUCache::new(2);
        lru.put(1, 1);
        lru.put(2, 2);
        assert_eq!(lru.get(1), 1);
        lru.put(3, 3);
        assert_eq!(lru.get(2), -1);
        lru.put(4, 4);
        assert_eq!(lru.get(1), -1);
        assert_eq!(lru.get(3), 3);
        assert_eq!(lru.get(4), 4);
    }
}
