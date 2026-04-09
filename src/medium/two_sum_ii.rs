/// [167] Two Sum II - Input Array Is Sorted
/// Difficulty: Medium
/// Topics: Array, Two Pointers, Binary Search
/// Tags: NeetCode150
///
/// Given a 1-indexed array of integers numbers that is already sorted in non-decreasing order,
/// find two numbers such that they add up to a specific target number.
///
/// Return the indices of the two numbers, index1 and index2, added by one as an integer array [index1, index2] of length 2.
/// The tests are generated such that there is exactly one solution. You may not use the same element twice.
/// Your solution must use only constant extra space.
///
/// Link: https://leetcode.com/problems/two-sum-ii-input-array-is-sorted/

struct Solution;

impl Solution {
    pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
        // [1, 2, 3, 4], 3)
        let (mut left, mut right) = (0, numbers.len() - 1);
        while left < right {
            let sum: i32 = numbers[left] + numbers[right];
            if sum == target {
                // 1-indexed အဖြေတောင်းထားလို့ +1 လုပ်ပေးရပါမယ်
                return vec![left as i32 + 1, right as i32 + 1];
            } else if sum > target {
                right -= 1;
            } else {
                left += 1;
            }
        }
        vec![]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(Solution::two_sum(vec![1, 2, 3, 4], 3), vec![1, 2]);
    }

    #[test]
    fn test_2() {
        assert_eq!(Solution::two_sum(vec![2, 3, 4], 6), vec![1, 3]);
    }

    #[test]
    fn test_3() {
        assert_eq!(Solution::two_sum(vec![-1, 0], -1), vec![1, 2]);
    }

    #[test]
    fn test_large_numbers() {
        assert_eq!(Solution::two_sum(vec![5, 25, 75], 100), vec![2, 3]);
    }

    #[test]
    fn test_case_5() {
        assert_eq!(
            Solution::two_sum(vec![-5, -3, 0, 2, 4, 6, 8], 5),
            vec![2, 7]
        );
    }
}
