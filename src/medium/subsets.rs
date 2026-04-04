/// [78] Subsets
/// Difficulty: Medium
/// Topics: Array, Backtracking, Bit Manipulation
/// Tags: Blind75, NeetCode150
///
/// Given an integer array nums of unique elements, return all possible subsets (the power set).
/// The solution set must not contain duplicate subsets. Return the solution in any order.
///
/// Link: https://leetcode.com/problems/subsets/

struct Solution;

impl Solution {
    pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let mut result = Solution::subsets(vec![1, 2, 3]);
        result.sort();
        assert_eq!(result.len(), 8);
    }

    #[test]
    fn test_2() {
        assert_eq!(Solution::subsets(vec![0]), vec![vec![], vec![0]]);
    }

    #[test]
    fn test_3() {
        assert_eq!(Solution::subsets(vec![]), vec![vec![]]);
    }
}
