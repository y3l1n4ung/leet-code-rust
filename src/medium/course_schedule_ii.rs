/// [210] Course Schedule II
/// Difficulty: Medium
/// Topics: Depth-First Search, Breadth-First Search, Graph, Topological Sort
/// Tags: NeetCode150
///
/// There are a total of numCourses courses you have to take, labeled from 0 to numCourses - 1. You are given an array prerequisites where prerequisites[i] = [ai, bi] indicates that you must take course bi first if you want to take course ai.
/// Return the ordering of courses you should take to finish all courses. If there are many valid answers, return any of them. If it is impossible to finish all courses, return an empty array.
///
/// Link: https://leetcode.com/problems/course-schedule-ii/

struct Solution;

impl Solution {
    pub fn find_order(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> Vec<i32> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(Solution::find_order(2, vec![vec![1, 0]]), vec![0, 1]);
    }

    #[test]
    fn test_2() {
        let result = Solution::find_order(4, vec![vec![1, 0], vec![2, 0], vec![3, 1], vec![3, 2]]);
        assert!(result == vec![0, 1, 2, 3] || result == vec![0, 2, 1, 3]);
    }

    #[test]
    fn test_3() {
        assert_eq!(Solution::find_order(1, vec![]), vec![0]);
    }
}
