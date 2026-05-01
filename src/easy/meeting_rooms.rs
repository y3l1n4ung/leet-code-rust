/// [252] Meeting Rooms
/// Difficulty: Easy
/// Topics: Array, Sorting
/// Tags: NeetCode150
///
/// Given an array of meeting time intervals where intervals[i] = [starti, endi], determine if a person could attend all meetings.
///
/// Link: https://leetcode.com/problems/meeting-rooms/ (Premium)
/// Free Link: https://www.lintcode.com/problem/920/

struct Solution;

impl Solution {
    pub fn can_attend_meetings(intervals: Vec<Vec<i32>>) -> bool {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let intervals = vec![vec![0, 30], vec![5, 10], vec![15, 20]];
        assert_eq!(Solution::can_attend_meetings(intervals), false);
    }

    #[test]
    fn test_2() {
        let intervals = vec![vec![7, 10], vec![2, 4]];
        assert_eq!(Solution::can_attend_meetings(intervals), true);
    }

    #[test]
    fn test_3() {
        assert_eq!(Solution::can_attend_meetings(vec![]), true);
    }
}
