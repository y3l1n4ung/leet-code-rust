/// [269] Alien Dictionary
/// Difficulty: Hard
/// Topics: Array, String, Depth-First Search, Breadth-First Search, Graph, Topological Sort
/// Tags: Blind75, NeetCode150
///
/// There is a new alien language that uses the English alphabet. However, the order of the letters is unknown to you.
/// You are given a list of strings words from the alien language's dictionary, where the strings in words are sorted lexicographically by the rules of this new language.
/// Return a string of the unique letters in the new alien language sorted in lexicographically increasing order by the new language's rules. If there is no solution, return "". If there are multiple solutions, return any of them.
///
/// Link: https://leetcode.com/problems/alien-dictionary/ (Premium)
/// Free Link: https://www.lintcode.com/problem/892/

struct Solution;

impl Solution {
    pub fn alien_order(words: Vec<String>) -> String {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let words = vec!["wrt".to_string(), "wrf".to_string(), "er".to_string(), "ett".to_string(), "rftt".to_string()];
        assert_eq!(Solution::alien_order(words), "wertf".to_string());
    }

    #[test]
    fn test_2() {
        let words = vec!["z".to_string(), "x".to_string()];
        assert_eq!(Solution::alien_order(words), "zx".to_string());
    }

    #[test]
    fn test_3() {
        let words = vec!["z".to_string(), "x".to_string(), "z".to_string()];
        assert_eq!(Solution::alien_order(words), "".to_string());
    }
}
