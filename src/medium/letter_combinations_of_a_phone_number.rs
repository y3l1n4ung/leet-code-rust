/// [17] Letter Combinations of a Phone Number
/// Difficulty: Medium
/// Topics: Hash Table, String, Backtracking
/// Tags: NeetCode150
///
/// Given a string containing digits from 2-9 inclusive, return all possible letter combinations that the number could represent. Return the answer in any order.
/// A mapping of digits to letters (just like on the telephone buttons) is given below. Note that 1 does not map to any letters.
///
/// Link: https://leetcode.com/problems/letter-combinations-of-a-phone-number/

struct Solution;

impl Solution {
    pub fn letter_combinations(digits: String) -> Vec<String> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let mut result = Solution::letter_combinations("23".to_string());
        result.sort();
        let mut expected = vec!["ad", "ae", "af", "bd", "be", "bf", "cd", "ce", "cf"];
        expected.sort();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_2() {
        assert_eq!(
            Solution::letter_combinations("".to_string()),
            Vec::<String>::new()
        );
    }

    #[test]
    fn test_3() {
        let mut result = Solution::letter_combinations("2".to_string());
        result.sort();
        let mut expected = vec!["a", "b", "c"];
        expected.sort();
        assert_eq!(result, expected);
    }
}
