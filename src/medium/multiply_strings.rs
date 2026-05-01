/// [43] Multiply Strings
/// Difficulty: Medium
/// Topics: Math, String, Simulation
/// Tags: NeetCode150
///
/// Given two non-negative integers num1 and num2 represented as strings, return the product of num1 and num2, also represented as a string.
/// Note: You must not use any built-in BigInteger library or convert the inputs to integer directly.
///
/// Link: https://leetcode.com/problems/multiply-strings/

struct Solution;

impl Solution {
    pub fn multiply(num1: String, num2: String) -> String {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            Solution::multiply("2".to_string(), "3".to_string()),
            "6".to_string()
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(
            Solution::multiply("123".to_string(), "456".to_string()),
            "56088".to_string()
        );
    }

    #[test]
    fn test_3() {
        assert_eq!(
            Solution::multiply("0".to_string(), "0".to_string()),
            "0".to_string()
        );
    }
}
