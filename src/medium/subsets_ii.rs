/// [90] Subsets II
/// Difficulty: Medium
/// Topics: Array, Backtracking, Bit Manipulation
/// Tags: NeetCode150
///
/// Given an integer array nums that may contain duplicates, return all possible subsets (the power set).
/// The solution set must not contain duplicate subsets. Return the solution in any order.
///
/// Link: https://leetcode.com/problems/subsets-ii/

struct Solution;

impl Solution {
    pub fn subsets_with_dup(nums: Vec<i32>) -> Vec<Vec<i32>> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let mut result = Solution::subsets_with_dup(vec![1, 2, 2]);
        result.sort();
        assert_eq!(result.len(), 6);
    }

    #[test]
    fn test_2() {
        assert_eq!(Solution::subsets_with_dup(vec![0]), vec![vec![], vec![0]]);
    }

    #[test]
    fn test_3() {
        let mut result = Solution::subsets_with_dup(vec![1, 1]);
        result.sort();
        assert_eq!(result, vec![vec![], vec![1], vec![1, 1]]);
    }
}
