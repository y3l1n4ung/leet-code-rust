/// [104] Maximum Depth of Binary Tree
/// Difficulty: Easy
/// Topics: Tree, Depth-First Search, Breadth-First Search, Binary Tree
/// Tags: Blind75, NeetCode150
///
/// Given the root of a binary tree, return its maximum depth.
/// A binary tree's maximum depth is the number of nodes along the longest path from the root node down to the farthest leaf node.
///
/// Link: https://leetcode.com/problems/maximum-depth-of-binary-tree/

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
    pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
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
        // Output: 3
        let root = node(3);
        let n9 = node(9);
        let n20 = node(20);
        let n15 = node(15);
        let n7 = node(7);

        if let Some(ref n) = n20 {
            n.borrow_mut().left = n15;
            n.borrow_mut().right = n7;
        }
        if let Some(ref n) = root {
            n.borrow_mut().left = n9;
            n.borrow_mut().right = n20;
        }
        assert_eq!(Solution::max_depth(root), 3);
    }

    #[test]
    fn test_2() {
        // Input: root = [1,null,2]
        // Output: 2
        let root = node(1);
        root.as_ref().unwrap().borrow_mut().right = node(2);
        assert_eq!(Solution::max_depth(root), 2);
    }

    #[test]
    fn test_3() {
        assert_eq!(Solution::max_depth(None), 0);
    }
}
