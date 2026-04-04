/// [543] Diameter of Binary Tree
/// Difficulty: Easy
/// Topics: Tree, Depth-First Search, Binary Tree
/// Tags: NeetCode150
///
/// Given the root of a binary tree, return the length of the diameter of the tree.
/// The diameter of a binary tree is the length of the longest path between any two nodes in a tree. This path may or may not pass through the root.
/// The length of a path between two nodes is represented by the number of edges between them.
///
/// Link: https://leetcode.com/problems/diameter-of-binary-tree/

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
    pub fn diameter_of_binary_tree(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        // Input: root = [1,2,3,4,5]
        // Output: 3
        todo!();
    }

    #[test]
    fn test_2() {
        // Input: root = [1,2]
        // Output: 1
        todo!();
    }

    #[test]
    fn test_3() {
        assert_eq!(Solution::diameter_of_binary_tree(None), 0);
    }
}
