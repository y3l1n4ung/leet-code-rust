/// [15] 3Sum
/// Difficulty: Medium
/// Topics: Array, Two Pointers, Sorting
/// Tags: Blind75, NeetCode150
///
/// Given an integer array nums, return all the triplets [nums[i], nums[j], nums[k]] such that i != j, i != k, and j != k, and nums[i] + nums[j] + nums[k] == 0.
/// Notice that the solution set must not contain duplicate triplets.
///
/// Link: https://leetcode.com/problems/3sum/

struct Solution;

impl Solution {
    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let mut result = Solution::three_sum(vec![-1, 0, 1, 2, -1, -4]);
        for r in result.iter_mut() {
            r.sort();
        }
        result.sort();
        let mut expected = vec![vec![-1, -1, 2], vec![-1, 0, 1]];
        for e in expected.iter_mut() {
            e.sort();
        }
        expected.sort();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_2() {
        assert_eq!(Solution::three_sum(vec![0, 1, 1]), Vec::<Vec<i32>>::new());
    }

    #[test]
    fn test_3() {
        assert_eq!(Solution::three_sum(vec![0, 0, 0]), vec![vec![0, 0, 0]]);
    }
}
