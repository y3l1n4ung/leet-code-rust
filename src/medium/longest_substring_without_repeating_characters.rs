/// [3] Longest Substring Without Repeating Characters
/// Difficulty: Medium
/// Topics: Hash Table, String, Sliding Window
/// Tags: Blind75, NeetCode150
///
/// Given a string s, find the length of the longest substring without repeating characters.
///
/// Link: https://leetcode.com/problems/longest-substring-without-repeating-characters/

struct Solution;

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(Solution::length_of_longest_substring("abcabcbb".to_string()), 3);
    }

    #[test]
    fn test_2() {
        assert_eq!(Solution::length_of_longest_substring("bbbbb".to_string()), 1);
    }

    #[test]
    fn test_3() {
        assert_eq!(Solution::length_of_longest_substring("pwwkew".to_string()), 3);
    }
}
