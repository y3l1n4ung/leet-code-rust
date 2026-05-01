/// [1143] Longest Common Subsequence
/// Difficulty: Medium
/// Topics: String, Dynamic Programming
/// Tags: Blind75, NeetCode150
///
/// Given two strings text1 and text2, return the length of their longest common subsequence. If there is no common subsequence, return 0.
/// A subsequence of a string is a new string generated from the original string with some characters (can be none) deleted without changing the relative order of the remaining characters.
///
/// Link: https://leetcode.com/problems/longest-common-subsequence/

struct Solution;

impl Solution {
    pub fn longest_common_subsequence(text1: String, text2: String) -> i32 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            Solution::longest_common_subsequence("abcde".to_string(), "ace".to_string()),
            3
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(
            Solution::longest_common_subsequence("abc".to_string(), "abc".to_string()),
            3
        );
    }

    #[test]
    fn test_3() {
        assert_eq!(
            Solution::longest_common_subsequence("abc".to_string(), "def".to_string()),
            0
        );
    }
}
