struct Solution;
impl Solution {
    pub fn shuffle(nums: Vec<i32>, n: i32) -> Vec<i32> {
        let mut result: Vec<i32> = vec![];
        for i in 0..nums.len() {
            result.push(nums[i]);
            result.push(nums[n as usize + i]);
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_shuffle() {
        Solution::shuffle(vec![2, 5, 1, 3, 4, 7], 3);
    }
}
