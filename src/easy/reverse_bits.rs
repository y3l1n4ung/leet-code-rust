/// [190] Reverse Bits
/// Difficulty: Easy
/// Topics: Divide and Conquer, Bit Manipulation
/// Tags: Blind75, NeetCode150
///
/// Reverse bits of a given 32 bits unsigned integer.
///
/// Link: https://leetcode.com/problems/reverse-bits/

struct Solution;

impl Solution {
    pub fn reverse_bits(x: u32) -> u32 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(Solution::reverse_bits(43261596), 964176192);
    }

    #[test]
    fn test_2() {
        assert_eq!(Solution::reverse_bits(1), 2147483648);
    }

    #[test]
    fn test_3() {
        assert_eq!(Solution::reverse_bits(0), 0);
    }
}
