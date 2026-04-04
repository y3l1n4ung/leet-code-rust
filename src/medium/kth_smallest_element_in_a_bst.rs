/// [230] Kth Smallest Element in a BST
/// Difficulty: Medium
/// Topics: Tree, Depth-First Search, Binary Search Tree, Binary Tree
/// Tags: Blind75, NeetCode150
///
/// Given the root of a binary search tree, and an integer k, return the kth smallest value (1-indexed) of all the values of the nodes in the tree.
///
/// Link: https://leetcode.com/problems/kth-smallest-element-in-a-bst/

use std::rc::Rc;
use std::cell::RefCell;

// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
  pub val: i32,
  pub left: Option<Rc<RefCell<TreeNode>>>,
  pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
  #[inline]
  pub fn new(val: i32) -> Self {
    TreeNode {
      val,
      left: None,
      right: None
    }
  }
}

struct Solution;

impl Solution {
    pub fn kth_smallest(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> i32 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        // Input: root = [3,1,4,null,2], k = 1
        // Output: 1
        todo!();
    }

    #[test]
    fn test_2() {
        // Input: root = [5,3,6,2,4,null,null,1], k = 3
        // Output: 3
        todo!();
    }

    #[test]
    fn test_3() {
        // Input: root = [1], k = 1
        // Output: 1
        todo!();
    }
}
