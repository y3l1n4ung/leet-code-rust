use std::cell::RefCell;
/// [235] Lowest Common Ancestor of a Binary Search Tree
/// Difficulty: Easy
/// Topics: Tree, Depth-First Search, Binary Search Tree, Binary Tree
/// Tags: Blind75, NeetCode150
///
/// Given a binary search tree (BST), find the lowest common ancestor (LCA) node of two given nodes in the BST.
/// According to the definition of LCA on Wikipedia: "The lowest common ancestor is defined between two nodes p and q as the lowest node in T that has both p and q as descendants (where we allow a node to be a descendant of itself)."
///
/// Link: https://leetcode.com/problems/lowest-common-ancestor-of-a-binary-search-tree/
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
    pub fn lowest_common_ancestor(
        root: Option<Rc<RefCell<TreeNode>>>,
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
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
        // Input: root = [6,2,8,0,4,7,9,null,null,3,5], p = 2, q = 8
        // Output: 6
        let root = node(6);
        let n2 = node(2);
        let n8 = node(8);
        let n0 = node(0);
        let n4 = node(4);
        let n7 = node(7);
        let n9 = node(9);
        let n3 = node(3);
        let n5 = node(5);

        if let Some(ref n) = n4 {
            n.borrow_mut().left = n3;
            n.borrow_mut().right = n5;
        }
        if let Some(ref n) = n2 {
            n.borrow_mut().left = n0;
            n.borrow_mut().right = n4.clone();
        }
        if let Some(ref n) = n8 {
            n.borrow_mut().left = n7;
            n.borrow_mut().right = n9;
        }
        if let Some(ref n) = root {
            n.borrow_mut().left = n2.clone();
            n.borrow_mut().right = n8.clone();
        }

        let lca = Solution::lowest_common_ancestor(root.clone(), n2, n8);
        assert_eq!(lca.as_ref().unwrap().borrow().val, 6);
    }

    #[test]
    fn test_2() {
        // Input: root = [6,2,8,0,4,7,9,null,null,3,5], p = 2, q = 4
        // Output: 2
        let root = node(6);
        let n2 = node(2);
        let n8 = node(8);
        let n0 = node(0);
        let n4 = node(4);

        if let Some(ref n) = n2 {
            n.borrow_mut().left = n0;
            n.borrow_mut().right = n4.clone();
        }
        if let Some(ref n) = root {
            n.borrow_mut().left = n2.clone();
            n.borrow_mut().right = n8;
        }

        let lca = Solution::lowest_common_ancestor(root, n2, n4);
        assert_eq!(lca.as_ref().unwrap().borrow().val, 2);
    }

    #[test]
    fn test_3() {
        assert_eq!(Solution::lowest_common_ancestor(None, None, None), None);
    }
}
