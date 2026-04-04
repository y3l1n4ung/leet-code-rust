/// [105] Construct Binary Tree from Preorder and Inorder Traversal
/// Difficulty: Medium
/// Topics: Array, Hash Table, Divide and Conquer, Tree, Binary Tree
/// Tags: Blind75, NeetCode150
///
/// Given two integer arrays preorder and inorder where preorder is the preorder traversal of a binary tree and inorder is the inorder traversal of the same tree, construct and return the binary tree.
///
/// Link: https://leetcode.com/problems/construct-binary-tree-from-preorder-and-inorder-traversal/

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
    pub fn build_tree(preorder: Vec<i32>, inorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        // Input: preorder = [3,9,20,15,7], inorder = [9,3,15,20,7]
        // Output: [3,9,20,null,null,15,7]
        todo!();
    }

    #[test]
    fn test_2() {
        assert_eq!(Solution::build_tree(vec![-1], vec![-1]), Some(Rc::new(RefCell::new(TreeNode::new(-1)))));
    }

    #[test]
    fn test_3() {
        assert_eq!(Solution::build_tree(vec![], vec![]), None);
    }
}
