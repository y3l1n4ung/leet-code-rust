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

    fn node(val: i32) -> Option<Rc<RefCell<TreeNode>>> {
        Some(Rc::new(RefCell::new(TreeNode::new(val))))
    }

/// Helper to build a tree from a level-order vector (LeetCode format)
    /// null is represented as -1 for simplicity in this example
    fn build_tree(values: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
        if values.is_empty() || values[0] == -1 {
            return None;
        }
        let root = node(values[0]);
        let mut queue = std::collections::VecDeque::new();
        queue.push_back(root.clone());
        let mut i = 1;
        while i < values.len() {
            if let Some(curr) = queue.pop_front().flatten() {
                if i < values.len() && values[i] != -1 {
                    let left = node(values[i]);
                    curr.borrow_mut().left = left.clone();
                    queue.push_back(left);
                }
                i += 1;
                if i < values.len() && values[i] != -1 {
                    let right = node(values[i]);
                    curr.borrow_mut().right = right.clone();
                    queue.push_back(right);
                }
                i += 1;
            }
        }
        root
    }

    #[test]
    fn test_empty_tree() {
        assert_eq!(Solution::is_balanced(None), true);
    }

    #[test]
    fn test_single_node() {
        assert_eq!(Solution::is_balanced(node(1)), true);
    }

    #[test]
    fn test_leetcode_example_1() {
        // [3,9,20,null,null,15,7]
        let root = build_tree(&[3, 9, 20, -1, -1, 15, 7]);
        assert_eq!(Solution::is_balanced(root), true);
    }

    #[test]
    fn test_leetcode_example_2() {
        // [1,2,2,3,3,null,null,4,4]
        let root = build_tree(&[1, 2, 2, 3, 3, -1, -1, 4, 4]);
        assert_eq!(Solution::is_balanced(root), false);
    }

    #[test]
    fn test_balanced_skewed() {
        //     1
        //    / \
        //   2   3
        //  /
        // 4
        let root = build_tree(&[1, 2, 3, 4]);
        assert_eq!(Solution::is_balanced(root), true);
    }

    #[test]
    fn test_perfectly_balanced() {
        //      1
        //    /   \
        //   2     3
        //  / \   / \
        // 4   5 6   7
        let root = build_tree(&[1, 2, 3, 4, 5, 6, 7]);
        assert_eq!(Solution::is_balanced(root), true);
    }

    #[test]
    fn test_unbalanced_deep() {
        // Unbalanced deep in the right subtree
        // [1,2,2,3,3,null,null,4,4] but on the right
        let root = build_tree(&[1, 2, 2, -1, -1, 3, 3, -1, -1, 4, 4]);
        assert_eq!(Solution::is_balanced(root), false);
    }
}
