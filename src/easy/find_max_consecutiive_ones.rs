struct Solution;

impl Solution {
    pub fn find_max_consecutive_ones(nums: Vec<i32>) -> i32 {
        let mut largest_counter = 0;
        let mut current_counter = 0;

        for i in nums {
            if i == 1 {
                current_counter += 1;
                if current_counter > largest_counter {
                    largest_counter = current_counter;
                }
            } else {
                current_counter = 0;
            }
        }

        largest_counter
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_shuffle() {
        let result = Solution::find_max_consecutive_ones(vec![1, 1, 0, 0, 1, 1, 1]);
        dbg!(result);
    }
}
