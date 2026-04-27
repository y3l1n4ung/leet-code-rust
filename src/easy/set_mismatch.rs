struct Solution;
impl Solution {
    pub fn find_error_nums(nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();
        let mut slot = vec![0; n + 1];
        let mut missing = 0;
        let mut duplicate = 0;

        // 1,2,2,4
        // 1,2,0,1 -> MISSING IS INDEX 3 , count == 2 = DUPLICATE

        for &num in &nums {
            slot[num as usize] += 1;
        }
        for i in 1..=n {
            if slot[i] == 2 {
                duplicate = i as i32;
            } else if slot[i] == 0 {
                missing = i as i32;
            }
        }
        vec![duplicate, missing]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_one() {
        let r = Solution::find_error_nums(vec![1, 2, 2, 4]);
        assert_eq!(r, vec![2, 3])
    }
}
