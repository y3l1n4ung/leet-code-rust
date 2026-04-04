/// [50] Pow(x, n)
/// Difficulty: Medium
/// Topics: Math, Recursion
/// Tags: NeetCode150
///
/// Implement pow(x, n), which calculates x raised to the power n (i.e., x^n).
///
/// Link: https://leetcode.com/problems/powx-n/

struct Solution;

impl Solution {
    pub fn my_pow(x: f64, n: i32) -> f64 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(Solution::my_pow(2.00000, 10), 1024.00000);
    }

    #[test]
    fn test_2() {
        assert_eq!(Solution::my_pow(2.10000, 3), 9.26100);
    }

    #[test]
    fn test_3() {
        assert_eq!(Solution::my_pow(2.00000, -2), 0.25000);
    }
}
