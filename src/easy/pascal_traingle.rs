//! # 118. Pascal's Triangle
//! 
//! Given an integer `numRows`, return the first `numRows` of Pascal's triangle.
//! 
//! ### Problem Statement
//! In Pascal's triangle, each number is the sum of the two numbers directly above it.

struct Solution;

impl Solution {
    pub fn generate(num_rows: i32) -> Vec<Vec<i32>> {
        let mut triangle: Vec<Vec<i32>> = Vec::with_capacity(num_rows as usize);
        
        for i in 0..num_rows {
            let i_idx = i as usize;
            let mut row = vec![1; i_idx + 1];
            
            for j in 1..i {
                let j_idx = j as usize;
                let prev_row = &triangle[i_idx - 1];
                
                let left_parent = prev_row[j_idx - 1]; 
                let right_parent = prev_row[j_idx];

                row[j_idx] = left_parent + right_parent;
            }
            triangle.push(row);
        }
        triangle
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_five_rows() {
        let expected = vec![
            vec![1],
            vec![1, 1],
            vec![1, 2, 1],
            vec![1, 3, 3, 1],
            vec![1, 4, 6, 4, 1],
        ];
        assert_eq!(Solution::generate(5), expected);
    }

    #[test]
    fn test_single_row() {
        assert_eq!(Solution::generate(1), vec![vec![1]]);
    }
}