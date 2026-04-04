/// [79] Word Search
/// Difficulty: Medium
/// Topics: Array, Backtracking, Matrix
/// Tags: Blind75, NeetCode150
///
/// Given an m x n grid of characters board and a string word, return true if word exists in the grid.
/// The word can be constructed from letters of sequentially adjacent cells, where adjacent cells are horizontally or vertically neighboring. The same letter cell may not be used more than once.
///
/// Link: https://leetcode.com/problems/word-search/

struct Solution;

impl Solution {
    pub fn exist(board: Vec<Vec<char>>, word: String) -> bool {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let board = vec![vec!['A','B','C','E'],vec!['S','F','C','S'],vec!['A','D','E','E']];
        assert_eq!(Solution::exist(board, "ABCCED".to_string()), true);
    }

    #[test]
    fn test_2() {
        let board = vec![vec!['A','B','C','E'],vec!['S','F','C','S'],vec!['A','D','E','E']];
        assert_eq!(Solution::exist(board, "SEE".to_string()), true);
    }

    #[test]
    fn test_3() {
        let board = vec![vec!['A','B','C','E'],vec!['S','F','C','S'],vec!['A','D','E','E']];
        assert_eq!(Solution::exist(board, "ABCB".to_string()), false);
    }
}
