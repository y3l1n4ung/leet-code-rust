/// [1448] Count Good Nodes in Binary Tree
/// Difficulty: Medium
/// Topics: Tree, Depth-First Search, Breadth-First Search, Binary Tree
/// Tags: NeetCode150
///
/// Given a binary tree root, a node X in the tree is named good if in the path from root to X, there are no nodes with a value greater than X.
/// Return the number of good nodes in the binary tree.
///
/// Link: https://leetcode.com/problems/count-good-nodes-in-binary-tree/

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
    pub fn good_nodes(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
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
        // Input: root = [3,1,4,3,null,1,5]
        // Output: 4
        let root = node(3);
        let n1 = node(1);
        let n4 = node(4);
        let n3 = node(3);
        let n1_2 = node(1);
        let n5 = node(5);

        if let Some(ref n) = n1 { n.borrow_mut().left = n3; }
        if let Some(ref n) = n4 { 
            n.borrow_mut().left = n1_2;
            n.borrow_mut().right = n5;
        }
        if let Some(ref r) = root {
            r.borrow_mut().left = n1;
            r.borrow_mut().right = n4;
        }
        assert_eq!(Solution::good_nodes(root), 4);
    }

    #[test]
    fn test_2() {
        // Input: root = [3,3,null,4,2]
        // Output: 3
        let root = node(3);
        let n3 = node(3);
        let n4 = node(4);
        let n2 = node(2);

        if let Some(ref n) = n3 {
            n.borrow_mut().left = n4;
            n.borrow_mut().right = n2;
        }
        if let Some(ref r) = root {
            r.borrow_mut().left = n3;
        }
        assert_eq!(Solution::good_nodes(root), 3);
    }

    #[test]
    fn test_3() {
        assert_eq!(Solution::good_nodes(None), 0);
    }
}
