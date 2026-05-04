//! # 206. Reverse Linked List
//! 
//! Given the `head` of a singly linked list, reverse the list, and return the reversed list.
//! 
//! ### Example 1:
//! **Input:** `head = [1,2,3,4,5]`
//! **Output:** `[5,4,3,2,1]`
//! 
//! ### Example 2:
//! **Input:** `head = [1,2]`
//! **Output:** `[2,1]`
//! 
//! ### Example 3:
//! **Input:** `head = []`
//! **Output:** `[]`
//! 
//! ### Constraints:
//! * The number of nodes in the list is the range `[0, 5000]`.
//! * `-5000 <= Node.val <= 5000`
//! 
//! ---
//!
//! ### Myanmar Explanation
//! Singly Linked List တစ်ခုကို ပြောင်းပြန် (Reverse) ပြန်လှန်ခိုင်းတာ ဖြစ်ပါတယ်။
//! 
//! **စဉ်းစားပုံ အဆင့်ဆင့်:**
//! ၁။ လက်ရှိ node (`curr`) ရဲ့ pointer ကို နောက်က node ဆီ မညွှန်ခိုင်းတော့ဘဲ သူ့ရဲ့ အရှေ့က node (`prev`) ဆီကို ပြန်ညွှန်ခိုင်းရပါမယ်။
//! ၂။ ဒါပေမဲ့ pointer ကို ဖြတ်လိုက်ရင် နောက်က node တွေနဲ့ အဆက်အသွယ် ပြတ်သွားမှာ ဖြစ်တဲ့အတွက် `next` node ကို temporary သိမ်းထားဖို့ လိုပါတယ်။
//! ၃။ Loop တစ်ခုပတ်ပြီး တစ်ဆင့်ချင်းစီ pointer လှန်သွားရင်း နောက်ဆုံး node ရောက်တဲ့အခါ အဲ့ဒီ node ဟာ reverse ဖြစ်သွားတဲ့ list ရဲ့ head အသစ် ဖြစ်လာပါလိမ့်မယ်။

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

struct Solution;

impl Solution {
    pub fn reverse_list(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
      let mut prev_node:Option<Box<ListNode>> = None;
       while head.is_some() {
        let mut node =head.take().unwrap();
        head = node.next;
        node.next = prev_node;
        prev_node =  Some(node); 
      }
      prev_node
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn to_list(vec: Vec<i32>) -> Option<Box<ListNode>> {
        let mut head = None;
        for &val in vec.iter().rev() {
            let mut node = ListNode::new(val);
            node.next = head;
            head = Some(Box::new(node));
        }
        head
    }

    fn to_vec(mut head: Option<Box<ListNode>>) -> Vec<i32> {
        let mut vec = Vec::new();
        while let Some(node) = head {
            vec.push(node.val);
            head = node.next;
        }
        vec
    }

    #[test]
    fn test_1() {
        let input = to_list(vec![1, 2, 3, 4, 5]);
        let output = Solution::reverse_list(input);
        assert_eq!(to_vec(output), vec![5, 4, 3, 2, 1]);
    }

    #[test]
    fn test_2() {
        let input = to_list(vec![1, 2]);
        let output = Solution::reverse_list(input);
        assert_eq!(to_vec(output), vec![2, 1]);
    }

    #[test]
    fn test_3() {
        assert_eq!(Solution::reverse_list(None), None);
    }
}
