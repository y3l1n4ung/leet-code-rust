/// [191] Number of 1 Bits
/// Difficulty: Easy
/// Topics: Divide and Conquer, Bit Manipulation
/// Tags: Blind75, NeetCode150
///
/// Write a function that takes the binary representation of a positive integer and returns the number of set bits it has (also known as the Hamming weight).
///
/// Link: https://leetcode.com/problems/number-of-1-bits/

struct Solution;

impl Solution {
    pub fn hamming_weight(n: i32) -> i32 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(Solution::hamming_weight(11), 3);
    }

    #[test]
    fn test_2() {
        assert_eq!(Solution::hamming_weight(128), 1);
    }

    #[test]
    fn test_3() {
        assert_eq!(Solution::hamming_weight(2147483645), 30);
    }
}
