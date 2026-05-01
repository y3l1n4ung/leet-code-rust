//! # Build an Array With Stack Operations
//!
//! You are given an array `target` and an integer `n`.
//! You have an empty stack with a two operations:
//! - "Push": Read the next number from the stream 1, 2, 3, ..., n and push it onto the stack.
//! - "Pop": Remove the top element from the stack.
//! You need to return the operations to build the `target` array using these stack operations.
//!
//! Example 1:
//! Input: target = [1,3], n = 3
//! Output: ["Push","Push","Pop","Push"]
//! Explanation:
//! Read 1: Push 1.
//! Read 2: Push 2, Pop 2.
//! Read 3: Push 3.

struct Solution;

impl Solution {
    pub fn build_array(target: Vec<i32>, n: i32) -> Vec<String> {
        let mut stack: Vec<String> = vec![];
        let mut current_stream = 1;

        for &expected in &target {
            while current_stream < expected {
                stack.push("Push".to_string());
                stack.push("Pop".to_string());
                current_stream += 1;
            }
            stack.push("Push".to_string());
            current_stream += 1;
        }

        stack
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let target = vec![1, 3];
        let n = 3;
        let result = Solution::build_array(target, n);
        assert_eq!(result, vec!["Push", "Push", "Pop", "Push"]);
    }

    #[test]
    fn test_example_2() {
        let target = vec![1, 2, 3];
        let n = 3;
        let result = Solution::build_array(target, n);
        assert_eq!(result, vec!["Push", "Push", "Push"]);
    }

    #[test]
    fn test_example_3() {
        let target = vec![1, 2];
        let n = 4;
        let result = Solution::build_array(target, n);
        assert_eq!(result, vec!["Push", "Push"]);
    }
}
