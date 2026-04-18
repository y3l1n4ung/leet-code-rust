/// [981] Time Based Key-Value Store
/// Difficulty: Medium
/// Topics: Hash Table, String, Binary Search, Design
/// Tags: Blind75, NeetCode150
///
/// Design a time-based key-value data structure that can store multiple values for the same key at different time stamps and retrieve the key's value at a certain timestamp.
/// Implement the TimeMap class:
/// - TimeMap() Initializes the object.
/// - void set(String key, String value, int timestamp) Stores the key key with the value value at the given time timestamp.
/// - String get(String key, int timestamp) Returns a value such that set was called previously, with timestamp_prev <= timestamp. If there are multiple such values, it returns the value associated with the largest timestamp_prev. If there are no such values, it returns "".
///
/// Link: https://leetcode.com/problems/time-based-key-value-store/

struct TimeMap {
    todo: (),
}

impl TimeMap {
    pub fn new() -> Self {
        todo!()
    }

    pub fn set(&self, key: String, value: String, timestamp: i32) {
        todo!()
    }

    pub fn get(&self, key: String, timestamp: i32) -> String {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let time_map = TimeMap::new();
        time_map.set("foo".to_string(), "bar".to_string(), 1);
        assert_eq!(time_map.get("foo".to_string(), 1), "bar".to_string());
        assert_eq!(time_map.get("foo".to_string(), 3), "bar".to_string());
        time_map.set("foo".to_string(), "bar2".to_string(), 4);
        assert_eq!(time_map.get("foo".to_string(), 4), "bar2".to_string());
        assert_eq!(time_map.get("foo".to_string(), 5), "bar2".to_string());
    }
}
