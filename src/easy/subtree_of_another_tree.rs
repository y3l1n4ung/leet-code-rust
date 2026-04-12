/// [572] Subtree of Another Tree
/// Difficulty: Easy
/// Topics: Tree, Depth-First Search, String Matching, Binary Tree
/// Tags: Blind75, NeetCode150
///
/// Given the roots of two binary trees root and subRoot, return true if there is a subtree of root with the same structure and node values of subRoot and false otherwise.
/// A subtree of a binary tree tree is a tree that consists of a node in tree and all of this node's descendants. The tree tree could also be considered as a subtree of itself.
///
/// Link: https://leetcode.com/problems/subtree-of-another-tree/

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
    pub fn is_subtree(root: Option<Rc<RefCell<TreeNode>>>, sub_root: Option<Rc<RefCell<TreeNode>>>) -> bool {
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
        // Input: root = [3,4,5,1,2], subRoot = [4,1,2]
        // Output: true
        let root = node(3);
        let n4 = node(4);
        let n5 = node(5);
        let n1 = node(1);
        let n2 = node(2);

        if let Some(ref n) = n4 {
            n.borrow_mut().left = n1.clone();
            n.borrow_mut().right = n2.clone();
        }
        if let Some(ref n) = root {
            n.borrow_mut().left = n4.clone();
            n.borrow_mut().right = n5;
        }

        let sub_root = node(4);
        sub_root.as_ref().unwrap().borrow_mut().left = n1;
        sub_root.as_ref().unwrap().borrow_mut().right = n2;

        assert_eq!(Solution::is_subtree(root, sub_root), true);
    }

    #[test]
    fn test_2() {
        // Input: root = [3,4,5,1,2,null,null,null,null,0], subRoot = [4,1,2]
        // Output: false
        let root = node(3);
        let n4 = node(4);
        let n5 = node(5);
        let n1 = node(1);
        let n2 = node(2);
        let n0 = node(0);

        if let Some(ref n) = n2 {
            n.borrow_mut().left = n0;
        }
        if let Some(ref n) = n4 {
            n.borrow_mut().left = n1.clone();
            n.borrow_mut().right = n2.clone();
        }
        if let Some(ref n) = root {
            n.borrow_mut().left = n4;
            n.borrow_mut().right = n5;
        }

        let sub_root = node(4);
        sub_root.as_ref().unwrap().borrow_mut().left = n1;
        sub_root.as_ref().unwrap().borrow_mut().right = node(2); // different node 2

        assert_eq!(Solution::is_subtree(root, sub_root), false);
    }

    #[test]
    fn test_3() {
        assert_eq!(Solution::is_subtree(None, None), true);
    }
}
