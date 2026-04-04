/// [1899] Merge Triplets to Form Target Sum
/// Difficulty: Medium
/// Topics: Array, Greedy
/// Tags: NeetCode150
///
/// A triplet is an array of three integers. You are given a 2D integer array triplets, where triplets[i] = [ai, bi, ci] describes the ith triplet. You are also given an integer array target = [x, y, z] that describes the target triplet we want to form.
/// To form the target triplet, you may select any number of indices (possibly none) and update target as the element-wise maximum of all selected triplets.
/// Return true if it is possible to form the target triplet [x, y, z] as described above, or false otherwise.
///
/// Link: https://leetcode.com/problems/merge-triplets-to-form-target-sum/

struct Solution;

impl Solution {
    pub fn merge_triplets(triplets: Vec<Vec<i32>>, target: Vec<i32>) -> bool {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let triplets = vec![vec![2,5,3],vec![1,8,4],vec![1,7,5]];
        let target = vec![2,7,5];
        assert_eq!(Solution::merge_triplets(triplets, target), true);
    }

    #[test]
    fn test_2() {
        let triplets = vec![vec![3,4,5],vec![4,5,6]];
        let target = vec![3,2,5];
        assert_eq!(Solution::merge_triplets(triplets, target), false);
    }

    #[test]
    fn test_3() {
        let triplets = vec![vec![2,5,3],vec![2,3,4],vec![1,2,5],vec![5,2,3]];
        let target = vec![5,5,5];
        assert_eq!(Solution::merge_triplets(triplets, target), true);
    }
}
