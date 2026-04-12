// Search in a 2D Matrix 🦀

pub struct Solution;

impl Solution {
    pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        if matrix.is_empty() { return false; }
        
        let rows = matrix.len();
        let cols = matrix[0].len();
        let mut left = 0;
        let mut right = (rows * cols) as i32 - 1;

        while left <= right {
            let mid = left + (right - left) / 2;
            let mid_val = matrix[(mid as usize) / cols][(mid as usize) % cols];
            
            if mid_val == target {
                return true;
            } else if mid_val < target {
                left = mid + 1;
            } else {
                right = mid - 1;
            }
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_search_matrix() {
        let matrix = vec![
            vec![1, 3, 5, 7],
            vec![10, 11, 16, 20],
            vec![23, 30, 34, 60]
        ];
        assert_eq!(Solution::search_matrix(matrix.clone(), 3), true);
        assert_eq!(Solution::search_matrix(matrix, 13), false);
    }
}
