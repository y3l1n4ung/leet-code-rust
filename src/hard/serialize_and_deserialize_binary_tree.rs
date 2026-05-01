use std::cell::RefCell;
/// [297] Serialize and Deserialize Binary Tree
/// Difficulty: Hard
/// Topics: String, Tree, Depth-First Search, Breadth-First Search, Design, Binary Tree
/// Tags: Blind75, NeetCode150
///
/// Serialization is the process of converting a data structure or object into a sequence of bits so that it can be stored in a file or memory buffer, or transmitted across a network connection link to be reconstructed later in the same or another computer environment.
/// Design an algorithm to serialize and deserialize a binary tree. There is no restriction on how your serialization/deserialization algorithm should work. You just need to ensure that a binary tree can be serialized to a string and this string can be deserialized to the original tree structure.
///
/// Link: https://leetcode.com/problems/serialize-and-deserialize-binary-tree/
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

struct Codec {
    todo: (),
}

impl Codec {
    fn new() -> Self {
        todo!()
    }

    fn serialize(&self, root: Option<Rc<RefCell<TreeNode>>>) -> String {
        todo!()
    }

    fn deserialize(&self, data: String) -> Option<Rc<RefCell<TreeNode>>> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let codec = Codec::new();
        let root = Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: Some(Rc::new(RefCell::new(TreeNode::new(2)))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 3,
                left: Some(Rc::new(RefCell::new(TreeNode::new(4)))),
                right: Some(Rc::new(RefCell::new(TreeNode::new(5)))),
            }))),
        })));
        assert_eq!(codec.deserialize(codec.serialize(root.clone())), root);
    }

    #[test]
    fn test_empty() {
        let codec = Codec::new();
        assert_eq!(codec.deserialize(codec.serialize(None)), None);
    }
}
