/// [10] Regular Expression Matching
/// Difficulty: Hard
/// Topics: String, Dynamic Programming, Recursion
/// Tags: Blind75, NeetCode150
///
/// Given an input string s and a pattern p, implement regular expression matching with support for '.' and '*' where:
/// - '.' Matches any single character.
/// - '*' Matches zero or more of the preceding element.
/// The matching should cover the entire input string (not partial).
///
/// Link: https://leetcode.com/problems/regular-expression-matching/

struct Solution;

impl Solution {
    pub fn is_match(s: String, p: String) -> bool {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(Solution::is_match("aa".to_string(), "a".to_string()), false);
    }

    #[test]
    fn test_2() {
        assert_eq!(Solution::is_match("aa".to_string(), "a*".to_string()), true);
    }

    #[test]
    fn test_3() {
        assert_eq!(Solution::is_match("ab".to_string(), ".*".to_string()), true);
    }
}
