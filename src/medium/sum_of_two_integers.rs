/// [371] Sum of Two Integers
/// Difficulty: Medium
/// Topics: Math, Bit Manipulation
/// Tags: Blind75, NeetCode150
///
/// Given two integers a and b, return the sum of the two integers without using the operators + and -.
///
/// Link: https://leetcode.com/problems/sum-of-two-integers/

struct Solution;

impl Solution {
    pub fn get_sum(a: i32, b: i32) -> i32 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(Solution::get_sum(1, 2), 3);
    }

    #[test]
    fn test_2() {
        assert_eq!(Solution::get_sum(2, 3), 5);
    }

    #[test]
    fn test_3() {
        assert_eq!(Solution::get_sum(-1, 1), 0);
    }
}
