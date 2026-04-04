/// [100] Same Tree
/// Difficulty: Easy
/// Topics: Tree, Depth-First Search, Breadth-First Search, Binary Tree
/// Tags: Blind75, NeetCode150
///
/// Given the roots of two binary trees p and q, write a function to check if they are the same or not.
/// Two binary trees are considered the same if they are structurally identical, and the nodes have the same value.
///
/// Link: https://leetcode.com/problems/same-tree/

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
    pub fn is_same_tree(p: Option<Rc<RefCell<TreeNode>>>, q: Option<Rc<RefCell<TreeNode>>>) -> bool {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        // Input: p = [1,2,3], q = [1,2,3]
        // Output: true
        todo!();
    }

    #[test]
    fn test_2() {
        // Input: p = [1,2], q = [1,null,2]
        // Output: false
        todo!();
    }

    #[test]
    fn test_3() {
        assert_eq!(Solution::is_same_tree(None, None), true);
    }
}
