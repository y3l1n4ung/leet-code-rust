/// [98] Validate Binary Search Tree
/// Difficulty: Medium
/// Topics: Tree, Depth-First Search, Binary Search Tree, Binary Tree
/// Tags: Blind75, NeetCode150
///
/// Given the root of a binary tree, determine if it is a valid binary search tree (BST).
/// A valid BST is defined as follows:
/// - The left subtree of a node contains only nodes with keys less than the node's key.
/// - The right subtree of a node contains only nodes with keys greater than the node's key.
/// - Both the left and right subtrees must also be binary search trees.
///
/// Link: https://leetcode.com/problems/validate-binary-search-tree/

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
    pub fn is_valid_bst(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        // Input: root = [2,1,3]
        // Output: true
        todo!();
    }

    #[test]
    fn test_2() {
        // Input: root = [5,1,4,null,null,3,6]
        // Output: false
        todo!();
    }

    #[test]
    fn test_3() {
        assert_eq!(Solution::is_valid_bst(None), true);
    }
}
