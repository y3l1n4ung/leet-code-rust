/// [150] Evaluate Reverse Polish Notation
/// Difficulty: Medium
/// Topics: Array, Math, Stack
/// Tags: NeetCode150
///
/// You are given an array of strings tokens that represents an arithmetic expression in a Reverse Polish Notation.
/// Evaluate the expression. Return an integer that represents the value of the expression.
///
/// Link: https://leetcode.com/problems/evaluate-reverse-polish-notation/

struct Solution;

impl Solution {
    pub fn eval_rpn(tokens: Vec<String>) -> i32 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let tokens = vec![
            "2".to_string(),
            "1".to_string(),
            "+".to_string(),
            "3".to_string(),
            "*".to_string(),
        ];
        assert_eq!(Solution::eval_rpn(tokens), 9);
    }

    #[test]
    fn test_2() {
        let tokens = vec![
            "4".to_string(),
            "13".to_string(),
            "5".to_string(),
            "/".to_string(),
            "+".to_string(),
        ];
        assert_eq!(Solution::eval_rpn(tokens), 6);
    }

    #[test]
    fn test_3() {
        let tokens = vec![
            "10".to_string(),
            "6".to_string(),
            "9".to_string(),
            "3".to_string(),
            "+".to_string(),
            "-11".to_string(),
            "*".to_string(),
            "/".to_string(),
            "*".to_string(),
            "17".to_string(),
            "+".to_string(),
            "5".to_string(),
            "+".to_string(),
        ];
        assert_eq!(Solution::eval_rpn(tokens), 22);
    }
}
