# System Design Thinking: LFU Cache (Least Frequently Used)

An LFU (Least Frequently Used) cache is a type of cache that evicts the items that are accessed least frequently when it reaches its capacity.

## 1. Requirements

### Functional Requirements
- `get(key)`: Returns the value of the key if it exists in the cache, otherwise returns -1.
- `put(key, value)`: Inserts or updates the value of the key. If the cache is full, it evicts the least frequently used key before inserting the new one.
- **Tie-breaking**: If multiple keys have the same minimum frequency, the least recently used (LRU) key among them is evicted.

### Non-Functional Requirements
- Both `get` and `put` operations must have **$O(1)$ time complexity**.

## 2. API Design

```rust
pub struct LFUCache {
    capacity: usize,
    // Add your internal state here
}

impl LFUCache {
    pub fn new(capacity: i32) -> Self;
    pub fn get(&mut self, key: i32) -> i32;
    pub fn put(&mut self, key: i32, value: i32);
}
```

## 3. High-Level Architecture

To achieve $O(1)$ for both `get` and `put`, we need to track both **frequency** and **recency** efficiently.

### Data Structures
- **Key-Value Map (`key_to_val`)**: Maps `key` to its `(value, frequency)`.
- **Frequency Map (`freq_to_keys`)**: Maps `frequency` to a **Doubly Linked List** of keys that share that frequency. The head of the list is the most recently used, and the tail is the least recently used (LRU) for that frequency.
- **Minimum Frequency (`min_freq`)**: Tracks the current minimum frequency in the cache to enable quick eviction.

## 4. Key Design Decisions

### Eviction Logic
1. Find the list of keys with `min_freq`.
2. Evict the **tail** (LRU) of that list.
3. Remove that key from the `key_to_val` map.

### Access Logic (get or update)
1. Retrieve the value and current frequency of the key.
2. Remove the key from its current frequency list in `freq_to_keys`.
3. If the list becomes empty and its frequency was `min_freq`, increment `min_freq`.
4. Add the key to the head of the list for `frequency + 1`.

## 5. Rust Implementation (Educational)

In the `mod.rs` file, you will implement this using `HashMap` and either `std::collections::LinkedList` or a custom doubly-linked list logic.

### Key Concepts to Practice:
- Using `HashMap` to store metadata.
- Managing multiple data structures that must stay in sync.
- Handling the case where `capacity` is 0.
