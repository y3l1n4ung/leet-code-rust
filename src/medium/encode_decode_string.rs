/// [659] Encode and Decode Strings
/// Difficulty: Medium
/// Topics: Array, String
/// Tags: Blind75, NeetCode150
///
/// Design an algorithm to encode a list of strings to a string. The encoded string is then sent over the network and is decoded back to the original list of strings.
///
/// Link: https://leetcode.com/problems/encode-and-decode-strings/ (Premium)
/// Free Link: https://lintcode.com/problem/659/

struct Solution;

impl Solution {
    pub fn encode(strs: Vec<String>) -> String {
        todo!()
    }

    pub fn decode(s: String) -> Vec<String> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let input = vec![
            "lint".to_string(),
            "code".to_string(),
            "love".to_string(),
            "you".to_string(),
        ];
        let encoded = Solution::encode(input.clone());
        assert_eq!(Solution::decode(encoded), input);
    }

    #[test]
    fn test_2() {
        let input = vec![
            "we".to_string(),
            "say".to_string(),
            ":".to_string(),
            "yes".to_string(),
        ];
        let encoded = Solution::encode(input.clone());
        assert_eq!(Solution::decode(encoded), input);
    }

    #[test]
    fn test_3() {
        let input = vec!["".to_string()];
        let encoded = Solution::encode(input.clone());
        assert_eq!(Solution::decode(encoded), input);
    }
}
