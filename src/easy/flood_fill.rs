/// [733] Flood Fill
/// Difficulty: Easy
/// Topics: Array, Depth-First Search, Breadth-First Search, Matrix
/// Tags: NeetCode150
///
/// An image is represented by an m x n integer grid image where image[i][j] represents the pixel value of the image.
/// You are also given three integers sr, sc, and color. You should perform a flood fill on the image starting from the pixel image[sr][sc].
/// To perform a flood fill, consider the starting pixel, plus any pixels connected 4-directionally to the starting pixel of the same color as the starting pixel, plus any pixels connected 4-directionally to those pixels (also with the same color), and so on. Replace the color of all of the aforementioned pixels with color.
/// Return the modified image after performing the flood fill.
///
/// Link: https://leetcode.com/problems/flood-fill/

struct Solution;

impl Solution {
    pub fn flood_fill(image: Vec<Vec<i32>>, sr: i32, sc: i32, color: i32) -> Vec<Vec<i32>> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let image = vec![vec![1,1,1],vec![1,1,0],vec![1,0,1]];
        let expected = vec![vec![2,2,2],vec![2,2,0],vec![2,0,1]];
        assert_eq!(Solution::flood_fill(image, 1, 1, 2), expected);
    }

    #[test]
    fn test_2() {
        let image = vec![vec![0,0,0],vec![0,0,0]];
        let expected = vec![vec![0,0,0],vec![0,0,0]];
        assert_eq!(Solution::flood_fill(image, 0, 0, 0), expected);
    }

    #[test]
    fn test_3() {
        let image = vec![vec![0,0,0],vec![0,1,1]];
        let expected = vec![vec![0,0,0],vec![0,2,2]];
        assert_eq!(Solution::flood_fill(image, 1, 1, 2), expected);
    }
}
