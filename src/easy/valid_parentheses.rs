use std::collections::HashMap;

/// [20] Valid Parentheses
/// Difficulty: Easy
/// Topics: String, Stack
/// Tags: Blind75, NeetCode150
///
/// Given a string s containing just the characters '(', ')', '{', '}', '[' and ']', determine if the input string is valid.
/// An input string is valid if:
/// 1. Open brackets must be closed by the same type of brackets.
/// 2. Open brackets must be closed in the correct order.
/// 3. Every close bracket has a corresponding open bracket of the same type.
///
/// Link: https://leetcode.com/problems/valid-parentheses/

struct Solution;

impl Solution {
    pub fn is_valid(s: String) -> bool {
        let opening = "([{";
        let closing = ")]}";
        let map: HashMap<char, char> = HashMap::from([('(', ')'), ('{', '}'), ('[', ']')]);
        let mut stack: Vec<char> = Vec::new();
        for char in s.chars() {
            if opening.contains(char) {
                stack.push(char);
            } else {
                if stack.is_empty() {
                    return false;
                }
                let opening = stack.pop().unwrap();
                let closing = *map.get(&opening).unwrap();
                return closing == char;
            }
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(Solution::is_valid("()".to_string()), true);
    }

    #[test]
    fn test_2() {
        assert_eq!(Solution::is_valid("([]{})".to_string()), true);
    }

    #[test]
    fn test_3() {
        assert_eq!(Solution::is_valid("(]".to_string()), false);
    }
}
