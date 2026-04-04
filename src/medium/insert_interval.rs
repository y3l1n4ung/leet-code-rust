/// [57] Insert Interval
/// Difficulty: Medium
/// Topics: Array
/// Tags: Blind75, NeetCode150
///
/// You are given an array of non-overlapping intervals intervals where intervals[i] = [starti, endi] represent the start and the end of the ith interval and intervals is sorted in ascending order by starti. You are also given an interval newInterval = [start, end] that represents the start and end of another interval.
/// Insert newInterval into intervals such that intervals is still sorted in ascending order by starti and intervals still does not have any overlapping intervals (merge overlapping intervals if necessary).
/// Return intervals after the insertion.
///
/// Link: https://leetcode.com/problems/insert-interval/

struct Solution;

impl Solution {
    pub fn insert(intervals: Vec<Vec<i32>>, new_interval: Vec<i32>) -> Vec<Vec<i32>> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let intervals = vec![vec![1,3],vec![6,9]];
        let new_interval = vec![2,5];
        assert_eq!(Solution::insert(intervals, new_interval), vec![vec![1,5],vec![6,9]]);
    }

    #[test]
    fn test_2() {
        let intervals = vec![vec![1,2],vec![3,5],vec![6,7],vec![8,10],vec![12,16]];
        let new_interval = vec![4,8];
        assert_eq!(Solution::insert(intervals, new_interval), vec![vec![1,2],vec![3,10],vec![12,16]]);
    }

    #[test]
    fn test_3() {
        assert_eq!(Solution::insert(vec![], vec![5,7]), vec![vec![5,7]]);
    }
}
