/// [115] Distinct Subsequences
/// Difficulty: Hard
/// Topics: String, Dynamic Programming
/// Tags: NeetCode150
///
/// Given two strings s and t, return the number of distinct subsequences of s which equals t.
///
/// Link: https://leetcode.com/problems/distinct-subsequences/

struct Solution;

impl Solution {
    pub fn num_distinct(s: String, t: String) -> i32 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            Solution::num_distinct("rabbbit".to_string(), "rabbit".to_string()),
            3
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(
            Solution::num_distinct("babgbag".to_string(), "bag".to_string()),
            5
        );
    }

    #[test]
    fn test_3() {
        assert_eq!(Solution::num_distinct("".to_string(), "a".to_string()), 0);
    }
}
