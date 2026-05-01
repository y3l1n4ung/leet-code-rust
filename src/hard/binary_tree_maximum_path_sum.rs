use std::cell::RefCell;
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
            right: None,
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

    fn node(val: i32) -> Option<Rc<RefCell<TreeNode>>> {
        Some(Rc::new(RefCell::new(TreeNode::new(val))))
    }

    #[test]
    fn test_1() {
        // Input: root = [1,2,3]
        // Output: 6
        let root = node(1);
        if let Some(ref r) = root {
            r.borrow_mut().left = node(2);
            r.borrow_mut().right = node(3);
        }
        assert_eq!(Solution::max_path_sum(root), 6);
    }

    #[test]
    fn test_2() {
        // Input: root = [-10,9,20,null,null,15,7]
        // Output: 42
        let root = node(-10);
        let n20 = node(20);
        if let Some(ref n) = n20 {
            n.borrow_mut().left = node(15);
            n.borrow_mut().right = node(7);
        }
        if let Some(ref r) = root {
            r.borrow_mut().left = node(9);
            r.borrow_mut().right = n20;
        }
        assert_eq!(Solution::max_path_sum(root), 42);
    }

    #[test]
    fn test_3() {
        assert_eq!(Solution::max_path_sum(node(5)), 5);
    }
}
