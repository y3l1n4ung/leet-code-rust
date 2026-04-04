/// [124] Binary Tree Maximum Path Sum
/// Difficulty: Hard
/// Topics: Dynamic Programming, Tree, Depth-First Search, Binary Tree
/// Tags: Blind75, NeetCode150
///
/// A path in a binary tree is a sequence of nodes where each pair of adjacent nodes in the sequence has an edge connecting them. A node can only appear in the sequence at most once. Note that the path does not need to pass through the root.
/// The path sum of a path is the sum of the node's values in the path.
/// Given the root of a binary tree, return the maximum path sum of any non-empty path.
///
/// Link: https://leetcode.com/problems/binary-tree-maximum-path-sum/

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
    pub fn max_path_sum(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        // Input: root = [1,2,3]
        // Output: 6
        todo!();
    }

    #[test]
    fn test_2() {
        // Input: root = [-10,9,20,null,null,15,7]
        // Output: 42
        todo!();
    }

    #[test]
    fn test_3() {
        assert_eq!(Solution::max_path_sum(Some(Rc::new(RefCell::new(TreeNode::new(5))))), 5);
    }
}
