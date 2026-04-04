/// [338] Counting Bits
/// Difficulty: Easy
/// Topics: Dynamic Programming, Bit Manipulation
/// Tags: Blind75, NeetCode150
///
/// Given an integer n, return an array ans of length n + 1 such that for each i (0 <= i <= n), ans[i] is the number of 1's in the binary representation of i.
///
/// Link: https://leetcode.com/problems/counting-bits/

struct Solution;

impl Solution {
    pub fn count_bits(n: i32) -> Vec<i32> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(Solution::count_bits(2), vec![0, 1, 1]);
    }

    #[test]
    fn test_2() {
        assert_eq!(Solution::count_bits(5), vec![0, 1, 1, 2, 1, 2]);
    }

    #[test]
    fn test_3() {
        assert_eq!(Solution::count_bits(0), vec![0]);
    }
}
