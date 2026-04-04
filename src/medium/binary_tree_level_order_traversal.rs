/// [102] Binary Tree Level Order Traversal
/// Difficulty: Medium
/// Topics: Tree, Breadth-First Search, Binary Tree
/// Tags: Blind75, NeetCode150
///
/// Given the root of a binary tree, return the level order traversal of its nodes' values. (i.e., from left to right, level by level).
///
/// Link: https://leetcode.com/problems/binary-tree-level-order-traversal/

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
    pub fn level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        // Input: root = [3,9,20,null,null,15,7]
        // Output: [[3],[9,20],[15,7]]
        todo!();
    }

    #[test]
    fn test_2() {
        assert_eq!(Solution::level_order(None), Vec::<Vec<i32>>::new());
    }

    #[test]
    fn test_3() {
        // Input: root = [1]
        // Output: [[1]]
        todo!();
    }
}
