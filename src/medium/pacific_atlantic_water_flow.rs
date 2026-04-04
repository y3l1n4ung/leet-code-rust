/// [417] Pacific Atlantic Water Flow
/// Difficulty: Medium
/// Topics: Array, Depth-First Search, Breadth-First Search, Matrix
/// Tags: Blind75, NeetCode150
///
/// There is an m x n rectangular island that borders both the Pacific Ocean and Atlantic Ocean. The Pacific Ocean touches the island's left and top edges, and the Atlantic Ocean touches the island's right and bottom edges.
/// The island is partitioned into a grid of square cells. You are given an m x n integer matrix heights where heights[r][c] represents the height above sea level of the cell at coordinate (r, c).
/// High-precipitation rain falls on the island. Rain water can flow to neighboring cells directly north, south, east, and west if the neighboring cell's height is less than or equal to the current cell's height. Water can flow from any cell adjacent to an ocean into the ocean.
/// Return a 2D list of grid coordinates result where result[i] = [ri, ci] denotes that rain water can flow from cell (ri, ci) to both the Pacific and Atlantic oceans.
///
/// Link: https://leetcode.com/problems/pacific-atlantic-water-flow/

struct Solution;

impl Solution {
    pub fn pacific_atlantic(heights: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let heights = vec![
            vec![1,2,2,3,5],
            vec![3,2,3,4,4],
            vec![2,4,5,3,1],
            vec![6,7,1,4,5],
            vec![5,1,1,2,4]
        ];
        let mut result = Solution::pacific_atlantic(heights);
        result.sort();
        let mut expected = vec![vec![0,4],vec![1,3],vec![1,4],vec![2,2],vec![3,0],vec![3,1],vec![4,0]];
        expected.sort();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_2() {
        let heights = vec![vec![1]];
        assert_eq!(Solution::pacific_atlantic(heights), vec![vec![0,0]]);
    }

    #[test]
    fn test_3() {
        assert_eq!(Solution::pacific_atlantic(vec![]), Vec::<Vec<i32>>::new());
    }
}
