# System Design Thinking: Design Linked List

Designing a linked list from scratch helps solidify an understanding of pointers, dynamic memory allocation, and the structural differences between arrays and node-based lists.

## 1. Requirements

### Functional Requirements
- `get(index)`: Get the value of the `index`-th node in the linked list. If the index is invalid, return -1.
- `add_at_head(val)`: Add a node of value `val` before the first element of the linked list.
- `add_at_tail(val)`: Append a node of value `val` to the last element of the linked list.
- `add_at_index(index, val)`: Add a node of value `val` before the `index`-th node. If `index` equals the length of the list, append it. If `index` is greater, the node will not be inserted.
- `delete_at_index(index)`: Delete the `index`-th node in the linked list, if the index is valid.

## 2. High-Level Architecture

### Singly Linked List vs. Doubly Linked List
- **Singly Linked List**: Each node contains a value and a pointer to the next node. Requires $O(N)$ to find a node for insertion/deletion, but uses less memory.
- **Doubly Linked List**: Each node contains a value, a pointer to the next node, and a pointer to the previous node. Deletion is easier if you already have the node reference, but memory overhead is higher.

### Rust-Specific Challenges
Implementing a linked list in Rust is notoriously challenging because of the ownership model.
- You can use `Option<Box<Node>>` for a singly linked list.
- You can use `Rc<RefCell<Node>>` for a doubly linked list, but it can be verbose and prone to memory leaks (reference cycles).
- Alternatively, use a `Vec` to represent nodes (arena allocation) where pointers are just array indices.

## 3. Rust Implementation (Educational)
In `mod.rs`, you will implement a custom linked list. Practice managing ownership safely.
