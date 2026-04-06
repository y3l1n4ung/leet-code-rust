use std::collections::{HashMap, HashSet};

/// [217] Contains Duplicate
/// Difficulty: Easy
/// Topics: Array, Hash Table, Sorting
/// Tags: Blind75, NeetCode150
///
/// Given an integer array nums, return true if any value appears at least twice in the array,
/// and return false if every element is distinct.
///
/// Link: https://leetcode.com/problems/contains-duplicate/

struct Solution;

impl Solution {
    pub fn contains_duplicate(nums: Vec<i32>) -> bool {
        let len = nums.len();
        let set: HashSet<i32> = HashSet::from_iter(nums);
        set.len() != len
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(Solution::contains_duplicate(vec![1, 2, 3, 1]), true);
    }

    #[test]
    fn test_2() {
        assert_eq!(Solution::contains_duplicate(vec![1, 2, 3, 4]), false);
    }

    #[test]
    fn test_3() {
        assert_eq!(
            Solution::contains_duplicate(vec![1, 1, 1, 3, 3, 4, 3, 2, 4, 2]),
            true
        );
    }
}
