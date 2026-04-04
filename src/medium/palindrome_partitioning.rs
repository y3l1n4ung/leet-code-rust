/// [131] Palindrome Partitioning
/// Difficulty: Medium
/// Topics: String, Dynamic Programming, Backtracking
/// Tags: Blind75, NeetCode150
///
/// Given a string s, partition s such that every substring of the partition is a palindrome. Return all possible palindrome partitioning of s.
///
/// Link: https://leetcode.com/problems/palindrome-partitioning/

struct Solution;

impl Solution {
    pub fn partition(s: String) -> Vec<Vec<String>> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let mut result = Solution::partition("aab".to_string());
        result.sort();
        let mut expected = vec![vec!["a".to_string(),"a".to_string(),"b".to_string()],vec!["aa".to_string(),"b".to_string()]];
        expected.sort();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_2() {
        assert_eq!(Solution::partition("a".to_string()), vec![vec!["a".to_string()]]);
    }

    #[test]
    fn test_3() {
        assert_eq!(Solution::partition("".to_string()), vec![Vec::<String>::new()]);
    }
}
