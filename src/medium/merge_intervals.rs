/// [56] Merge Intervals
/// Difficulty: Medium
/// Topics: Array, Sorting
/// Tags: Blind75, NeetCode150
///
/// Given an array of intervals where intervals[i] = [starti, endi], merge all overlapping intervals, and return an array of the non-overlapping intervals that cover all the intervals in the input.
///
/// Link: https://leetcode.com/problems/merge-intervals/

struct Solution;

impl Solution {
    pub fn merge(intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let intervals = vec![vec![1, 3], vec![2, 6], vec![8, 10], vec![15, 18]];
        assert_eq!(
            Solution::merge(intervals),
            vec![vec![1, 6], vec![8, 10], vec![15, 18]]
        );
    }

    #[test]
    fn test_2() {
        let intervals = vec![vec![1, 4], vec![4, 5]];
        assert_eq!(Solution::merge(intervals), vec![vec![1, 5]]);
    }

    #[test]
    fn test_3() {
        assert_eq!(Solution::merge(vec![vec![1, 4]]), vec![vec![1, 4]]);
    }
}
