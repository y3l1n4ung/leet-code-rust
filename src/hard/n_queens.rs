/// [51] N-Queens
/// Difficulty: Hard
/// Topics: Array, Backtracking
/// Tags: NeetCode150
///
/// The n-queens puzzle is the problem of placing n queens on an n x n chessboard such that no two queens attack each other.
/// Given an integer n, return all distinct solutions to the n-queens puzzle. You may return the answer in any order.
/// Each solution contains a distinct board configuration of the n-queens' placement, where 'Q' and '.' both indicate a queen and an empty space, respectively.
///
/// Link: https://leetcode.com/problems/n-queens/

struct Solution;

impl Solution {
    pub fn solve_n_queens(n: i32) -> Vec<Vec<String>> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let result = Solution::solve_n_queens(4);
        assert_eq!(result.len(), 2);
    }

    #[test]
    fn test_2() {
        let result = Solution::solve_n_queens(1);
        assert_eq!(result, vec![vec!["Q".to_string()]]);
    }

    #[test]
    fn test_3() {
        let result = Solution::solve_n_queens(2);
        assert_eq!(result.len(), 0);
    }
}
