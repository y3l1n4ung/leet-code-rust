/// [127] Word Ladder
/// Difficulty: Hard
/// Topics: Hash Table, String, Breadth-First Search
/// Tags: Blind75, NeetCode150
///
/// A transformation sequence from word beginWord to word endWord using a dictionary wordList is a sequence of words beginWord -> s1 -> s2 -> ... -> sk such that:
/// - Every adjacent pair of words differs by a single letter.
/// - Every si for 1 <= i <= k is in wordList. Note that beginWord does not need to be in wordList.
/// - sk == endWord
/// Given two words, beginWord and endWord, and a dictionary wordList, return the number of words in the shortest transformation sequence from beginWord to endWord, or 0 if no such sequence exists.
///
/// Link: https://leetcode.com/problems/word-ladder/

struct Solution;

impl Solution {
    pub fn ladder_length(begin_word: String, end_word: String, word_list: Vec<String>) -> i32 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let word_list = vec!["hot".to_string(), "dot".to_string(), "dog".to_string(), "lot".to_string(), "log".to_string(), "cog".to_string()];
        assert_eq!(Solution::ladder_length("hit".to_string(), "cog".to_string(), word_list), 5);
    }

    #[test]
    fn test_2() {
        let word_list = vec!["hot".to_string(), "dot".to_string(), "dog".to_string(), "lot".to_string(), "log".to_string()];
        assert_eq!(Solution::ladder_length("hit".to_string(), "cog".to_string(), word_list), 0);
    }

    #[test]
    fn test_3() {
        let word_list = vec!["a".to_string(), "b".to_string(), "c".to_string()];
        assert_eq!(Solution::ladder_length("a".to_string(), "c".to_string(), word_list), 2);
    }
}
