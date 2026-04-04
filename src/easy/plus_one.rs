/// [66] Plus One
/// Difficulty: Easy
/// Topics: Array, Math
/// Tags: NeetCode150
///
/// You are given a large integer represented as an integer array digits, where each digits[i] is the ith digit of the integer. The digits are ordered from most significant to least significant in left-to-right order. The large integer does not contain any leading 0's.
/// Increment the large integer by one and return the resulting array of digits.
///
/// Link: https://leetcode.com/problems/plus-one/

struct Solution;

impl Solution {
    pub fn plus_one(digits: Vec<i32>) -> Vec<i32> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(Solution::plus_one(vec![1, 2, 3]), vec![1, 2, 4]);
    }

    #[test]
    fn test_2() {
        assert_eq!(Solution::plus_one(vec![4, 3, 2, 1]), vec![4, 3, 2, 2]);
    }

    #[test]
    fn test_3() {
        assert_eq!(Solution::plus_one(vec![9]), vec![1, 0]);
    }
}
