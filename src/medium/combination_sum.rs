/// [39] Combination Sum
/// Difficulty: Medium
/// Topics: Array, Backtracking
/// Tags: Blind75, NeetCode150
///
/// Given an array of distinct integers candidates and a target integer target, return a list of all unique combinations of candidates where the chosen numbers sum to target. You may return the combinations in any order.
/// The same number may be chosen from candidates an unlimited number of times. Two combinations are unique if the frequency of at least one of the chosen numbers is different.
///
/// Link: https://leetcode.com/problems/combination-sum/

struct Solution;

impl Solution {
    pub fn combination_sum(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let mut result = Solution::combination_sum(vec![2, 3, 6, 7], 7);
        result.sort();
        let mut expected = vec![vec![2, 2, 3], vec![7]];
        expected.sort();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_2() {
        let mut result = Solution::combination_sum(vec![2, 3, 5], 8);
        result.sort();
        let mut expected = vec![vec![2, 2, 2, 2], vec![2, 3, 3], vec![3, 5]];
        expected.sort();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_3() {
        assert_eq!(
            Solution::combination_sum(vec![2], 1),
            Vec::<Vec<i32>>::new()
        );
    }
}
