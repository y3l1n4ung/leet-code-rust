/// [128] Longest Consecutive Sequence
/// Difficulty: Medium
/// Topics: Array, Hash Table, Union Find
/// Tags: Blind75, NeetCode150
///
/// Given an unsorted array of integers nums, return the length of the longest consecutive elements sequence.
/// You must write an algorithm that runs in O(n) time.
///
/// Link: https://leetcode.com/problems/longest-consecutive-sequence/


struct Solution;

impl Solution {
    
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(Solution::longest_consecutive(vec![100, 4, 200, 1, 3, 2]), 4);
    }

    #[test]
    fn test_2() {
        assert_eq!(Solution::longest_consecutive(vec![0, 3, 7, 2, 5, 8, 4, 6, 0, 1]), 9);
    }

    #[test]
    fn test_3() {
        assert_eq!(Solution::longest_consecutive(vec![]), 0);
    }
}
