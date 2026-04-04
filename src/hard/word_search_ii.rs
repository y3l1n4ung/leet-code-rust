/// [212] Word Search II
/// Difficulty: Hard
/// Topics: Array, String, Backtracking, Trie, Matrix
/// Tags: Blind75, NeetCode150
///
/// Given an m x n board of characters and a list of strings words, return all words on the board.
/// Each word must be constructed from letters of sequentially adjacent cells, where adjacent cells are horizontally or vertically neighboring. The same letter cell may not be used more than once in a word.
///
/// Link: https://leetcode.com/problems/word-search-ii/

struct Solution;

impl Solution {
    pub fn find_words(board: Vec<Vec<char>>, words: Vec<String>) -> Vec<String> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let board = vec![
            vec!['o','a','a','n'],
            vec!['e','t','a','e'],
            vec!['i','h','k','r'],
            vec!['i','f','l','v']
        ];
        let words = vec!["oath".to_string(),"pea".to_string(),"eat".to_string(),"rain".to_string()];
        let mut result = Solution::find_words(board, words);
        result.sort();
        let mut expected = vec!["eat".to_string(),"oath".to_string()];
        expected.sort();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_2() {
        let board = vec![vec!['a','b'],vec!['c','d']];
        let words = vec!["abcb".to_string()];
        assert_eq!(Solution::find_words(board, words), Vec::<String>::new());
    }

    #[test]
    fn test_3() {
        assert_eq!(Solution::find_words(vec![], vec![]), Vec::<String>::new());
    }
}
