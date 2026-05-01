/// [567] Permutation in String
/// Difficulty: Medium
/// Topics: Hash Table, Two Pointers, String, Sliding Window
/// Tags: NeetCode150
///
/// Given two strings s1 and s2, return true if s2 contains a permutation of s1, or false otherwise.
/// In other words, return true if one of s1's permutations is the substring of s2.
///
/// Link: https://leetcode.com/problems/permutation-in-string/

struct Solution;

impl Solution {
    pub fn check_inclusion(s1: String, s2: String) -> bool {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            Solution::check_inclusion("ab".to_string(), "eidbaooo".to_string()),
            true
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(
            Solution::check_inclusion("ab".to_string(), "eidboaoo".to_string()),
            false
        );
    }

    #[test]
    fn test_3() {
        assert_eq!(
            Solution::check_inclusion("adc".to_string(), "dcda".to_string()),
            true
        );
    }
}
