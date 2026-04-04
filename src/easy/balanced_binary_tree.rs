/// [110] Balanced Binary Tree
/// Difficulty: Easy
/// Topics: Tree, Depth-First Search, Binary Tree
/// Tags: NeetCode150
///
/// Given a binary tree, determine if it is height-balanced.
/// A height-balanced binary tree is a binary tree in which the depth of the two subtrees of every node never differs by more than one.
///
/// Link: https://leetcode.com/problems/balanced-binary-tree/

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
    pub fn is_balanced(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        // Input: root = [3,9,20,null,null,15,7]
        // Output: true
        todo!();
    }

    #[test]
    fn test_2() {
        // Input: root = [1,2,2,3,3,null,null,4,4]
        // Output: false
        todo!();
    }

    #[test]
    fn test_3() {
        assert_eq!(Solution::is_balanced(None), true);
    }
}
