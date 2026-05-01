/// [778] Swim in Rising Water
/// Difficulty: Hard
/// Topics: Array, Binary Search, Depth-First Search, Breadth-First Search, Union Find, Heap (Priority Queue), Matrix
/// Tags: NeetCode150
///
/// You are given an n x n integer matrix grid where each value grid[i][j] represents the elevation at that point (i, j).
/// The rain starts to fall. At time t, the depth of the water everywhere is t. You can swim from a square to another 4-directionally adjacent square if and only if both squares have an elevation at most t. You can swim infinite distances in zero time. Of course, you must stay within the boundaries of the grid during your swim.
/// Return the least time until you can reach the bottom right square (n - 1, n - 1) if you start at the top left square (0, 0).
///
/// Link: https://leetcode.com/problems/swim-in-rising-water/

struct Solution;

impl Solution {
    pub fn swim_in_water(grid: Vec<Vec<i32>>) -> i32 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let grid = vec![vec![0, 2], vec![1, 3]];
        assert_eq!(Solution::swim_in_water(grid), 3);
    }

    #[test]
    fn test_2() {
        let grid = vec![
            vec![0, 1, 2, 3, 4],
            vec![24, 23, 22, 21, 5],
            vec![12, 13, 14, 15, 16],
            vec![11, 17, 18, 19, 20],
            vec![10, 9, 8, 7, 6],
        ];
        assert_eq!(Solution::swim_in_water(grid), 16);
    }
}
