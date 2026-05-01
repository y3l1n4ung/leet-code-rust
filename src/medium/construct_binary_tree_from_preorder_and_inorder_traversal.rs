use std::cell::RefCell;
/// [105] Construct Binary Tree from Preorder and Inorder Traversal
/// Difficulty: Medium
/// Topics: Array, Hash Table, Divide and Conquer, Tree, Binary Tree
/// Tags: Blind75, NeetCode150
///
/// Given two integer arrays preorder and inorder where preorder is the preorder traversal of a binary tree and inorder is the inorder traversal of the same tree, construct and return the binary tree.
///
/// Link: https://leetcode.com/problems/construct-binary-tree-from-preorder-and-inorder-traversal/
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
    pub fn build_tree(preorder: Vec<i32>, inorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
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
        // Input: preorder = [3,9,20,15,7], inorder = [9,3,15,20,7]
        // Output: [3,9,20,null,null,15,7]
        let preorder = vec![3, 9, 20, 15, 7];
        let inorder = vec![9, 3, 15, 20, 7];
        let result = Solution::build_tree(preorder, inorder);

        let expected = node(3);
        let n20 = node(20);
        if let Some(ref n) = n20 {
            n.borrow_mut().left = node(15);
            n.borrow_mut().right = node(7);
        }
        if let Some(ref r) = expected {
            r.borrow_mut().left = node(9);
            r.borrow_mut().right = n20;
        }
        assert_eq!(result, expected);
    }

    #[test]
    fn test_2() {
        assert_eq!(
            Solution::build_tree(vec![-1], vec![-1]),
            Some(Rc::new(RefCell::new(TreeNode::new(-1))))
        );
    }

    #[test]
    fn test_3() {
        assert_eq!(Solution::build_tree(vec![], vec![]), None);
    }
}
