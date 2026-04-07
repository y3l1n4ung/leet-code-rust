/// [125] Valid Palindrome
/// Difficulty: Easy
/// Topics: Two Pointers, String
/// Tags: Blind75, NeetCode150
///
/// A phrase is a palindrome if, after converting all uppercase letters into lowercase letters and removing all non-alphanumeric characters, it reads the same forward and backward. Alphanumeric characters include letters and numbers.
/// Given a string s, return true if it is a palindrome, or false otherwise.
///
/// Link: https://leetcode.com/problems/valid-palindrome/

struct Solution;

impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        let (mut left, mut right) = (0, s.len() - 1);
        let chars: Vec<char> = s.chars().collect();
        if chars.is_empty() {
            return true;
        }
        while left < right {
            if !chars[left].is_alphanumeric() {
                left += 1;
                continue;
            }
            if !chars[right].is_alphanumeric() {
                right -= 1;
                continue;
            }
            if chars[left].to_ascii_lowercase() != chars[right].to_ascii_lowercase() {
                return false;
            }

            left += 1;
            right -= 1;
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            Solution::is_palindrome("A man, a plan, a canal: Panama".to_string()),
            true
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(Solution::is_palindrome("race a car".to_string()), false);
    }

    #[test]
    fn test_3() {
        assert_eq!(Solution::is_palindrome(" ".to_string()), true);
    }
}
