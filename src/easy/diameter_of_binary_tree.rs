use std::cell::RefCell;
/// [543] Diameter of Binary Tree
/// Difficulty: Easy
/// Topics: Tree, Depth-First Search, Binary Tree
/// Tags: NeetCode150
///
/// Given the root of a binary tree, return the length of the diameter of the tree.
/// The diameter of a binary tree is the length of the longest path between any two nodes in a tree. This path may or may not pass through the root.
/// The length of a path between two nodes is represented by the number of edges between them.
///
/// Link: https://leetcode.com/problems/diameter-of-binary-tree/
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
    pub fn diameter_of_binary_tree(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
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
        // Input: root = [1,2,3,4,5]
        let root = node(1);
        let n2 = node(2);
        let n3 = node(3);
        let n4 = node(4);
        let n5 = node(5);

        if let Some(ref n2_node) = n2 {
            n2_node.borrow_mut().left = n4;
            n2_node.borrow_mut().right = n5;
        }
        if let Some(ref r) = root {
            r.borrow_mut().left = n2;
            r.borrow_mut().right = n3;
        }
        assert_eq!(Solution::diameter_of_binary_tree(root), 3);
    }

    #[test]
    fn test_2() {
        // Input: root = [1,2]
        let root = node(1);
        root.as_ref().unwrap().borrow_mut().left = node(2);
        assert_eq!(Solution::diameter_of_binary_tree(root), 1);
    }

    #[test]
    fn test_3() {
        assert_eq!(Solution::diameter_of_binary_tree(None), 0);
    }
}
