/// [973] K Closest Points to Origin
/// Difficulty: Medium
/// Topics: Array, Math, Divide and Conquer, Geometry, Sorting, Heap (Priority Queue), Quickselect
/// Tags: NeetCode150
///
/// Given an array of points where points[i] = [xi, yi] represents a point on the X-Y plane and an integer k, return the k closest points to the origin (0, 0).
/// The distance between two points on the X-Y plane is the Euclidean distance (i.e., √(x1 - x2)^2 + (y1 - y2)^2).
/// You may return the answer in any order. The answer is guaranteed to be unique (except for the order that it is in).
///
/// Link: https://leetcode.com/problems/k-closest-points-to-origin/

struct Solution;

impl Solution {
    pub fn k_closest(points: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let points = vec![vec![1, 3], vec![-2, 2]];
        assert_eq!(Solution::k_closest(points, 1), vec![vec![-2, 2]]);
    }

    #[test]
    fn test_2() {
        let points = vec![vec![3, 3], vec![5, -1], vec![-2, 4]];
        let mut result = Solution::k_closest(points, 2);
        result.sort();
        let mut expected = vec![vec![3, 3], vec![-2, 4]];
        expected.sort();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_3() {
        assert_eq!(Solution::k_closest(vec![], 0), Vec::<Vec<i32>>::new());
    }
}
