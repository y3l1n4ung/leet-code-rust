/// [5] Longest Palindromic Substring
/// Difficulty: Medium
/// Topics: String, Dynamic Programming
/// Tags: Blind75, NeetCode150
///
/// Given a string s, return the longest palindromic substring in s.
///
/// Link: https://leetcode.com/problems/longest-palindromic-substring/

struct Solution;

impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let result = Solution::longest_palindrome("babad".to_string());
        assert!(result == "bab" || result == "aba");
    }

    #[test]
    fn test_2() {
        assert_eq!(Solution::longest_palindrome("cbbd".to_string()), "bb");
    }

    #[test]
    fn test_3() {
        assert_eq!(Solution::longest_palindrome("a".to_string()), "a");
    }
}
