/// [268] Missing Number
/// Difficulty: Easy
/// Topics: Array, Hash Table, Math, Binary Search, Bit Manipulation, Sorting
/// Tags: Blind75, NeetCode150
///
/// Given an array nums containing n distinct numbers in the range [0, n], return the only number in the range that is missing from the array.
///
/// Link: https://leetcode.com/problems/missing-number/

struct Solution;

impl Solution {
    pub fn missing_number(nums: Vec<i32>) -> i32 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(Solution::missing_number(vec![3, 0, 1]), 2);
    }

    #[test]
    fn test_2() {
        assert_eq!(Solution::missing_number(vec![0, 1]), 2);
    }

    #[test]
    fn test_3() {
        assert_eq!(Solution::missing_number(vec![9, 6, 4, 2, 3, 5, 7, 0, 1]), 8);
    }
}
