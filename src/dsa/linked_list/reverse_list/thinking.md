# Reverse Linked List - Thinking 🦀

## Theoretical Explanation
A linked list is reversed by re-pointing each node's `next` pointer to the *previous* node. 

### Key Patterns
Using three pointers:
- **`prev`**: Tracks the node already reversed (initially `None`).
- **`curr`**: Tracks the node we are currently reversing.
- **`next`**: Temporary pointer to store the rest of the list before re-pointing.

### Complexity
- **Time**: O(n)
- **Space**: O(1) (iterative) or O(n) (recursive).

## Implementation Tips
In Rust, `Option<Box<ListNode>>` makes this tricky with ownership. 
1. `take()` the `next` node from `curr`.
2. Set `curr.next` to `prev`.
3. Move `prev` and `curr` forward.
