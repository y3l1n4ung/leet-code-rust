# Merge Two Sorted Lists - Thinking 🦀

## Problem Statement
Merge two sorted linked lists and return it as a sorted list. The list should be made by splicing together the nodes of the first two lists.

## Theoretical Explanation
We can solve this by iteratively comparing the values of the nodes in both lists and appending the smaller one to our result list.

### Key Logic
- Use a **Dummy Node** to simplify the initial head setup.
- Keep a `curr` pointer to the last node in the result list.
- Compare `l1.val` and `l2.val`, then append the smaller one.
- After one list is empty, append the remainder of the other list.

### Complexity
- **Time**: O(n + m) where n, m are lengths of the lists.
- **Space**: O(1) (we are re-pointing existing nodes, not creating new ones).

## Rust Implementation Detail
Using `Option<Box<ListNode>>` makes it challenging to "splice" in-place without copying. We'll use a recursive approach or a safe iterative approach using `take()`.
