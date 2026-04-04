/// [253] Meeting Rooms II
/// Difficulty: Medium
/// Topics: Array, Two Pointers, Greedy, Sorting, Heap (Priority Queue)
/// Tags: NeetCode150
///
/// Given an array of meeting time intervals intervals where intervals[i] = [starti, endi], return the minimum number of conference rooms required.
///
/// Link: https://leetcode.com/problems/meeting-rooms-ii/ (Premium)
/// Free Link: https://www.lintcode.com/problem/919/

struct Solution;

impl Solution {
    pub fn min_meeting_rooms(intervals: Vec<Vec<i32>>) -> i32 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let intervals = vec![vec![0,30],vec![5,10],vec![15,20]];
        assert_eq!(Solution::min_meeting_rooms(intervals), 2);
    }

    #[test]
    fn test_2() {
        let intervals = vec![vec![7,10],vec![2,4]];
        assert_eq!(Solution::min_meeting_rooms(intervals), 1);
    }

    #[test]
    fn test_3() {
        assert_eq!(Solution::min_meeting_rooms(vec![]), 0);
    }
}
