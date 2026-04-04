/// [200] Number of Islands
/// Difficulty: Medium
/// Topics: Array, Depth-First Search, Breadth-First Search, Union Find, Matrix
/// Tags: Blind75, NeetCode150
///
/// Given an m x n 2D binary grid grid which represents a map of '1's (land) and '0's (water), return the number of islands.
/// An island is surrounded by water and is formed by connecting adjacent lands horizontally or vertically. You may assume all four edges of the grid are all surrounded by water.
///
/// Link: https://leetcode.com/problems/number-of-islands/

struct Solution;

impl Solution {
    pub fn num_islands(grid: Vec<Vec<char>>) -> i32 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let grid = vec![
            vec!['1','1','1','1','0'],
            vec!['1','1','0','1','0'],
            vec!['1','1','0','0','0'],
            vec!['0','0','0','0','0']
        ];
        assert_eq!(Solution::num_islands(grid), 1);
    }

    #[test]
    fn test_2() {
        let grid = vec![
            vec!['1','1','0','0','0'],
            vec!['1','1','0','0','0'],
            vec!['0','0','1','0','0'],
            vec!['0','0','0','1','1']
        ];
        assert_eq!(Solution::num_islands(grid), 3);
    }

    #[test]
    fn test_3() {
        assert_eq!(Solution::num_islands(vec![]), 0);
    }
}
