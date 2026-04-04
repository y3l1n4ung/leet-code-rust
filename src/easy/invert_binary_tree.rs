/// [226] Invert Binary Tree
/// Difficulty: Easy
/// Topics: Tree, Depth-First Search, Breadth-First Search, Binary Tree
/// Tags: Blind75, NeetCode150
///
/// Given the root of a binary tree, invert the tree, and return its root.
///
/// Link: https://leetcode.com/problems/invert-binary-tree/

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
    pub fn invert_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        // Input: root = [4,2,7,1,3,6,9]
        // Output: [4,7,2,9,6,3,1]
        todo!();
    }

    #[test]
    fn test_2() {
        // Input: root = [2,1,3]
        // Output: [2,3,1]
        todo!();
    }

    #[test]
    fn test_3() {
        // Input: root = []
        // Output: []
        assert_eq!(Solution::invert_tree(None), None);
    }
}
