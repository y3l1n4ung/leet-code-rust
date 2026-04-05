# System Design Thinking: Design Circular Queue

A circular queue is a linear data structure in which the operations are performed based on FIFO (First In First Out) principle and the last position is connected back to the first position to make a circle. It is also called "Ring Buffer".

## 1. Requirements

### Functional Requirements
- `en_queue(value)`: Insert an element into the circular queue. Return true if the operation is successful.
- `de_queue()`: Delete an element from the circular queue. Return true if the operation is successful.
- `front()`: Get the front item from the queue. If the queue is empty, return -1.
- `rear()`: Get the last item from the queue. If the queue is empty, return -1.
- `is_empty()`: Checks whether the circular queue is empty or not.
- `is_full()`: Checks whether the circular queue is full or not.

## 2. Advantages of Circular Queue
- Over a standard array-based queue: A standard queue suffers from the limitation that even if there is space available at the front (after dequeues), new elements cannot be inserted if the rear pointer has reached the end. A circular queue reuses the empty spaces at the front.

## 3. High-Level Architecture
- Use a fixed-size array (`Vec` in Rust initialized with size `k`).
- Use two pointers/indices: `head` and `tail`.
- Use a `count` or `size` variable to track how many elements are currently in the queue, or compute it using `(tail - head + capacity) % capacity`.

## 4. Rust Implementation (Educational)
In `mod.rs`, implement a thread-safe or single-threaded circular queue using an array-backed structure.
