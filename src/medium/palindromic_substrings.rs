/// [647] Palindromic Substrings
/// Difficulty: Medium
/// Topics: String, Dynamic Programming
/// Tags: Blind75, NeetCode150
///
/// Given a string s, return the number of palindromic substrings in it.
/// A string is a palindrome when it reads the same backward as forward.
/// A substring is a contiguous sequence of characters within the string.
///
/// Link: https://leetcode.com/problems/palindromic-substrings/

struct Solution;

impl Solution {
    pub fn count_substrings(s: String) -> i32 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(Solution::count_substrings("abc".to_string()), 3);
    }

    #[test]
    fn test_2() {
        assert_eq!(Solution::count_substrings("aaa".to_string()), 6);
    }

    #[test]
    fn test_3() {
        assert_eq!(Solution::count_substrings("a".to_string()), 1);
    }
}
