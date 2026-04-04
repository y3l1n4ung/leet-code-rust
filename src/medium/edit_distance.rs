/// [72] Edit Distance
/// Difficulty: Medium
/// Topics: String, Dynamic Programming
/// Tags: Blind75, NeetCode150
///
/// Given two strings word1 and word2, return the minimum number of operations required to convert word1 to word2.
/// You have the following three operations permitted on a word:
/// - Insert a character
/// - Delete a character
/// - Replace a character
///
/// Link: https://leetcode.com/problems/edit-distance/

struct Solution;

impl Solution {
    pub fn min_distance(word1: String, word2: String) -> i32 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(Solution::min_distance("horse".to_string(), "ros".to_string()), 3);
    }

    #[test]
    fn test_2() {
        assert_eq!(Solution::min_distance("intention".to_string(), "execution".to_string()), 5);
    }

    #[test]
    fn test_3() {
        assert_eq!(Solution::min_distance("".to_string(), "".to_string()), 0);
    }
}
