/// [235] Lowest Common Ancestor of a Binary Search Tree
/// Difficulty: Easy
/// Topics: Tree, Depth-First Search, Binary Search Tree, Binary Tree
/// Tags: Blind75, NeetCode150
///
/// Given a binary search tree (BST), find the lowest common ancestor (LCA) node of two given nodes in the BST.
/// According to the definition of LCA on Wikipedia: "The lowest common ancestor is defined between two nodes p and q as the lowest node in T that has both p and q as descendants (where we allow a node to be a descendant of itself)."
///
/// Link: https://leetcode.com/problems/lowest-common-ancestor-of-a-binary-search-tree/

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
    pub fn lowest_common_ancestor(root: Option<Rc<RefCell<TreeNode>>>, p: Option<Rc<RefCell<TreeNode>>>, q: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        // Input: root = [6,2,8,0,4,7,9,null,null,3,5], p = 2, q = 8
        // Output: 6
        todo!();
    }

    #[test]
    fn test_2() {
        // Input: root = [6,2,8,0,4,7,9,null,null,3,5], p = 2, q = 4
        // Output: 2
        todo!();
    }

    #[test]
    fn test_3() {
        assert_eq!(Solution::lowest_common_ancestor(None, None, None), None);
    }
}
