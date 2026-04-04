/// [22] Generate Parentheses
/// Difficulty: Medium
/// Topics: String, Dynamic Programming, Backtracking
/// Tags: NeetCode150
///
/// Given n pairs of parentheses, write a function to generate all combinations of well-formed parentheses.
///
/// Link: https://leetcode.com/problems/generate-parentheses/

struct Solution;

impl Solution {
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let mut result = Solution::generate_parenthesis(3);
        result.sort();
        let mut expected = vec!["((()))", "(()())", "(())()", "()(())", "()()()"];
        expected.sort();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_2() {
        assert_eq!(Solution::generate_parenthesis(1), vec!["()".to_string()]);
    }

    #[test]
    fn test_3() {
        let mut result = Solution::generate_parenthesis(2);
        result.sort();
        let mut expected = vec!["(())", "()()"];
        expected.sort();
        assert_eq!(result, expected);
    }
}
