/// [435] Non-overlapping Intervals
/// Difficulty: Medium
/// Topics: Array, Dynamic Programming, Greedy, Sorting
/// Tags: Blind75, NeetCode150
///
/// Given an array of intervals intervals where intervals[i] = [starti, endi], return the minimum number of intervals you need to remove to make the rest of the intervals non-overlapping.
///
/// Link: https://leetcode.com/problems/non-overlapping-intervals/

struct Solution;

impl Solution {
    pub fn erase_overlap_intervals(intervals: Vec<Vec<i32>>) -> i32 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let intervals = vec![vec![1, 2], vec![2, 3], vec![3, 4], vec![1, 3]];
        assert_eq!(Solution::erase_overlap_intervals(intervals), 1);
    }

    #[test]
    fn test_2() {
        let intervals = vec![vec![1, 2], vec![1, 2], vec![1, 2]];
        assert_eq!(Solution::erase_overlap_intervals(intervals), 2);
    }

    #[test]
    fn test_3() {
        let intervals = vec![vec![1, 2], vec![2, 3]];
        assert_eq!(Solution::erase_overlap_intervals(intervals), 0);
    }
}
