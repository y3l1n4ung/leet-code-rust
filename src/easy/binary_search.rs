use core::num;

/// [704] Binary Search
/// Difficulty: Easy
/// Topics: Array, Binary Search
/// Tags: NeetCode150
///
/// Given an array of integers nums which is sorted in ascending order, and an integer target, write a function to search target in nums.
/// If target exists, then return its index. Otherwise, return -1.
/// You must write an algorithm with O(log n) runtime complexity.
///
/// Link: https://leetcode.com/problems/binary-search/

struct Solution;

impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let mut low: i32 = 0;
        let mut high: i32 = (nums.len() - 1) as i32;

        while low <= high {
            let mid: i32 = low + (high - low) / 2;
            let val: i32 = nums[mid as usize];
            if target == val {
                return mid as i32;
            } else if target > val as i32 {
                low = mid + 1;
            } else {
                high = mid - 1;
            }
        }

        -1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(Solution::search(vec![-1, 0, 3, 5, 9, 12], 9), 4);
    }

    #[test]
    fn test_2() {
        assert_eq!(Solution::search(vec![-1, 0, 3, 5, 9, 12], 2), -1);
    }

    #[test]
    fn test_3() {
        assert_eq!(Solution::search(vec![5], 5), 0);
    }
    #[test]
    fn test_4() {
        assert_eq!(Solution::search(vec![5], -5), -1);
    }

    #[test]
    fn test_5() {
        assert_eq!(Solution::search(vec![-1, 0, 3, 5, 9, 12], 0), 1);
    }
}
