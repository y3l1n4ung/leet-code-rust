use std::collections::HashMap;

/// [242] Valid Anagram
/// Difficulty: Easy
/// Topics: Hash Table, String, Sorting
/// Tags: Blind75, NeetCode150
///
/// Given two strings s and t, return true if t is an anagram of s, and false otherwise.
///
/// Link: https://leetcode.com/problems/valid-anagram/

struct Solution;

impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        // if not found in key , + , for s , - for t.
        let mut map: HashMap<char, i32> = HashMap::new();
        // { 'a' : 3}
        for char in s.chars() {
            *map.entry(char).or_insert(0) += 1
        }
        for char in t.chars() {
            *map.entry(char).or_insert(0) -= 1;
        }
        map.values().all(|f| *f == 0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            Solution::is_anagram("anagram".to_string(), "nagaram".to_string()),
            true
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(
            Solution::is_anagram("rat".to_string(), "car".to_string()),
            false
        );
    }

    #[test]
    fn test_3() {
        assert_eq!(
            Solution::is_anagram("a".to_string(), "ab".to_string()),
            false
        );
    }
}
