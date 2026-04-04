/// [7] Reverse Integer
/// Difficulty: Medium
/// Topics: Math
/// Tags: NeetCode150
///
/// Given a signed 32-bit integer x, return x with its digits reversed. If reversing x causes the value to go outside the signed 32-bit integer range [-2^31, 2^31 - 1], then return 0.
///
/// Link: https://leetcode.com/problems/reverse-integer/

struct Solution;

impl Solution {
    pub fn reverse(x: i32) -> i32 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(Solution::reverse(123), 321);
    }

    #[test]
    fn test_2() {
        assert_eq!(Solution::reverse(-123), -321);
    }

    #[test]
    fn test_3() {
        assert_eq!(Solution::reverse(120), 21);
    }
}
