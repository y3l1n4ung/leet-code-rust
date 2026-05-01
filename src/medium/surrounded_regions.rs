/// [130] Surrounded Regions
/// Difficulty: Medium
/// Topics: Array, Depth-First Search, Breadth-First Search, Union Find, Matrix
/// Tags: NeetCode150
///
/// Given an m x n matrix board containing 'X' and 'O', capture all regions that are 4-directionally surrounded by 'X'.
/// A region is captured by flipping all 'O's into 'X's in that surrounded region.
///
/// Link: https://leetcode.com/problems/surrounded-regions/

struct Solution;

impl Solution {
    pub fn solve(board: &mut Vec<Vec<char>>) {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let mut board = vec![
            vec!['X', 'X', 'X', 'X'],
            vec!['X', 'O', 'O', 'X'],
            vec!['X', 'X', 'O', 'X'],
            vec!['X', 'O', 'X', 'X'],
        ];
        let expected = vec![
            vec!['X', 'X', 'X', 'X'],
            vec!['X', 'X', 'X', 'X'],
            vec!['X', 'X', 'X', 'X'],
            vec!['X', 'O', 'X', 'X'],
        ];
        Solution::solve(&mut board);
        assert_eq!(board, expected);
    }

    #[test]
    fn test_2() {
        let mut board = vec![vec!['X']];
        Solution::solve(&mut board);
        assert_eq!(board, vec![vec!['X']]);
    }

    #[test]
    fn test_3() {
        let mut board = vec![vec!['O']];
        Solution::solve(&mut board);
        assert_eq!(board, vec![vec!['O']]);
    }
}
