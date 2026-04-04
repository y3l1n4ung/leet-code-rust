/// [207] Course Schedule
/// Difficulty: Medium
/// Topics: Depth-First Search, Breadth-First Search, Graph, Topological Sort
/// Tags: Blind75, NeetCode150
///
/// There are a total of numCourses courses you have to take, labeled from 0 to numCourses - 1. You are given an array prerequisites where prerequisites[i] = [ai, bi] indicates that you must take course bi first if you want to take course ai.
/// Return true if you can finish all courses. Otherwise, return false.
///
/// Link: https://leetcode.com/problems/course-schedule/

struct Solution;

impl Solution {
    pub fn can_finish(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> bool {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(Solution::can_finish(2, vec![vec![1, 0]]), true);
    }

    #[test]
    fn test_2() {
        assert_eq!(Solution::can_finish(2, vec![vec![1, 0], vec![0, 1]]), false);
    }

    #[test]
    fn test_3() {
        assert_eq!(Solution::can_finish(1, vec![]), true);
    }
}
