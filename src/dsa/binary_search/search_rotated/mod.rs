// Search in Rotated Sorted Array Practice 🦀

pub struct Solution;

impl Solution {
    /// Practice: Search in Rotated Sorted Array
    /// Finds the index of target in nums, or -1 if not found.
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let mut left = 0;
        let mut right = nums.len() as i32 - 1;

        while left <= right {
            let mid = left + (right - left) / 2;
            let m = mid as usize;

            if nums[m] == target {
                return mid;
            }

            // Left part is sorted
            if nums[left as usize] <= nums[m] {
                if nums[left as usize] <= target && target < nums[m] {
                    right = mid - 1;
                } else {
                    left = mid + 1;
                }
            }
            // Right part is sorted
            else {
                if nums[m] < target && target <= nums[right as usize] {
                    left = mid + 1;
                } else {
                    right = mid - 1;
                }
            }
        }
        -1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_search_rotated() {
        assert_eq!(Solution::search(vec![4, 5, 6, 7, 0, 1, 2], 0), 4);
        assert_eq!(Solution::search(vec![4, 5, 6, 7, 0, 1, 2], 3), -1);
        assert_eq!(Solution::search(vec![1], 0), -1);
    }
}
