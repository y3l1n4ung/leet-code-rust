# System Design Thinking: Design Skiplist

A Skiplist is a probabilistic data structure that allows $O(\log n)$ search, addition, and erasure operations. It is a layered linked list where each upper layer acts as an "express lane" for the layer below.

## 1. Requirements

### Functional Requirements
- `search(target)`: Returns true if the `target` exists in the skiplist, false otherwise.
- `add(num)`: Inserts `num` into the skiplist.
- `erase(num)`: Removes `num` from the skiplist and returns true. If `num` doesn't exist, returns false.

## 2. Key Concepts

### Probability and Levels
- Every new node has a base level of 1.
- We toss a "coin" to decide if it should also appear in level 2, layer 3, etc., up to a maximum level.
- This creates sparse upper layers, allowing searches to skip over many nodes.

### Search Path
- Start at the highest layer of the head node.
- Move forward as long as the next node's value is less than the target.
- If the next node's value is greater than or equal to the target (or if it's null), drop down one level.
- Repeat until the target is found or we reach the bottom layer.

## 3. Rust Implementation Challenges

### Safe Pointers
- Implementing a multi-level linked list in safe Rust typically requires `Rc<RefCell<Node>>`.
- Alternatively, you can use an array-backed structure (arena allocation) to store nodes and refer to them via indices. This often circumvents borrow-checker issues and provides better cache locality.

## 4. Rust Implementation (Educational)
In `mod.rs`, you will implement this probabilistic structure.
