/// [97] Interleaving String
/// Difficulty: Medium
/// Topics: String, Dynamic Programming
/// Tags: NeetCode150
///
/// Given strings s1, s2, and s3, find whether s3 is formed by an interleaving of s1 and s2.
///
/// Link: https://leetcode.com/problems/interleaving-string/

struct Solution;

impl Solution {
    pub fn is_interleave(s1: String, s2: String, s3: String) -> bool {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            Solution::is_interleave(
                "aabcc".to_string(),
                "dbbca".to_string(),
                "aadbbcbcac".to_string()
            ),
            true
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(
            Solution::is_interleave(
                "aabcc".to_string(),
                "dbbca".to_string(),
                "aadbbbaccc".to_string()
            ),
            false
        );
    }

    #[test]
    fn test_3() {
        assert_eq!(
            Solution::is_interleave("".to_string(), "".to_string(), "".to_string()),
            true
        );
    }
}
