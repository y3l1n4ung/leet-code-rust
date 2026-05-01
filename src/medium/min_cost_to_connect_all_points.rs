/// [1584] Min Cost to Connect All Points
/// Difficulty: Medium
/// Topics: Array, Union Find, Graph, Minimum Spanning Tree
/// Tags: NeetCode150
///
/// You are given an array points representing the integer coordinates of some points on a 2D-plane, where points[i] = [xi, yi].
/// The cost of connecting two points [xi, yi] and [xj, yj] is the manhattan distance between them: |xi - xj| + |yi - yj|, where |val| is the absolute value of val.
/// Return the minimum cost to make all points connected. All points are connected if there is exactly one simple path between any two points.
///
/// Link: https://leetcode.com/problems/min-cost-to-connect-all-points/

struct Solution;

impl Solution {
    pub fn min_cost_connect_points(points: Vec<Vec<i32>>) -> i32 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let points = vec![vec![0, 0], vec![2, 2], vec![3, 10], vec![5, 2], vec![7, 0]];
        assert_eq!(Solution::min_cost_connect_points(points), 20);
    }

    #[test]
    fn test_2() {
        let points = vec![vec![3, 12], vec![-2, 5], vec![-4, 1]];
        assert_eq!(Solution::min_cost_connect_points(points), 18);
    }

    #[test]
    fn test_3() {
        assert_eq!(Solution::min_cost_connect_points(vec![vec![0, 0]]), 0);
    }
}
