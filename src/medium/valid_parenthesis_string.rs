/// [678] Valid Parenthesis String
/// Difficulty: Medium
/// Topics: String, Dynamic Programming, Stack, Greedy
/// Tags: NeetCode150
///
/// Given a string s containing only three types of characters: '(', ')' and '*', return true if s is valid.
/// The following rules define a valid string:
/// - Any left parenthesis '(' must have a corresponding right parenthesis ')'.
/// - Any right parenthesis ')' must have a corresponding left parenthesis '('.
/// - Left parenthesis '(' must go before the corresponding right parenthesis ')'.
/// - '*' could be treated as a single right parenthesis ')' or a single left parenthesis '(' or an empty string "".
///
/// Link: https://leetcode.com/problems/valid-parenthesis-string/

struct Solution;

impl Solution {
    pub fn check_valid_string(s: String) -> bool {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(Solution::check_valid_string("()".to_string()), true);
    }

    #[test]
    fn test_2() {
        assert_eq!(Solution::check_valid_string("(*)".to_string()), true);
    }

    #[test]
    fn test_3() {
        assert_eq!(Solution::check_valid_string("(*))".to_string()), true);
    }
}
