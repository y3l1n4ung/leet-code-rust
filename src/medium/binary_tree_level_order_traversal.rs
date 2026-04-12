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

    fn node(val: i32) -> Option<Rc<RefCell<TreeNode>>> {
        Some(Rc::new(RefCell::new(TreeNode::new(val))))
    }

    #[test]
    fn test_1() {
        // Input: root = [3,9,20,null,null,15,7]
        // Output: [[3],[9,20],[15,7]]
        let root = node(3);
        let n20 = node(20);
        if let Some(ref n) = n20 {
            n.borrow_mut().left = node(15);
            n.borrow_mut().right = node(7);
        }
        if let Some(ref r) = root {
            r.borrow_mut().left = node(9);
            r.borrow_mut().right = n20;
        }
        assert_eq!(Solution::level_order(root), vec![vec![3], vec![9, 20], vec![15, 7]]);
    }

    #[test]
    fn test_2() {
        assert_eq!(Solution::level_order(None), Vec::<Vec<i32>>::new());
    }

    #[test]
    fn test_3() {
        // Input: root = [1]
        // Output: [[1]]
        assert_eq!(Solution::level_order(node(1)), vec![vec![1]]);
    }
}
