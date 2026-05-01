/// [49] Group Anagrams
/// Difficulty: Medium
/// Topics: Array, Hash Table, String, Sorting
/// Tags: Blind75, NeetCode150
///
/// Given an array of strings strs, group the anagrams together. You can return the answer in any order.
///
/// Link: https://leetcode.com/problems/group-anagrams/

struct Solution;

impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let input = vec![
            "eat".to_string(),
            "tea".to_string(),
            "tan".to_string(),
            "ate".to_string(),
            "nat".to_string(),
            "bat".to_string(),
        ];
        let mut result = Solution::group_anagrams(input);
        for r in result.iter_mut() {
            r.sort();
        }
        result.sort_by_key(|a| a.len());
        // Simple length check or more detailed check
        assert_eq!(result.len(), 3);
    }

    #[test]
    fn test_2() {
        let input = vec!["".to_string()];
        assert_eq!(Solution::group_anagrams(input), vec![vec!["".to_string()]]);
    }

    #[test]
    fn test_3() {
        let input = vec!["a".to_string()];
        assert_eq!(Solution::group_anagrams(input), vec![vec!["a".to_string()]]);
    }
}
