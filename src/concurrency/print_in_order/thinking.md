# System Design Thinking: Print in Order

This concurrency challenge focuses on coordinating threads to ensure a specific execution order: `first()`, then `second()`, and finally `third()`.

## 1. Requirements

### Functional Requirements
- `first()`: Should always execute first.
- `second()`: Should wait for `first()` to complete.
- `third()`: Should wait for `second()` to complete.
- These methods will be called by different threads on the **same instance** of `Foo`.

## 2. Key Concepts

### Thread Synchronization
- We need a way for one thread to signal another thread that it has completed its task.
- **Spinning (Busy Waiting)**: A simple but inefficient way to wait for a condition to become true (e.g., a boolean flag).
- **Semaphores**: A classic synchronization primitive that can be used to control access to a resource or signal events between threads.
- **Condition Variables (`Condvar`)**: In Rust, `Condvar` is used with a `Mutex` to block a thread until a condition is met.

## 3. High-Level Architecture

```mermaid
graph LR
    T1[Thread 1] -- first() --> Done1((Done 1))
    T2[Thread 2] -- waits for Done 1 --> second()
    second() --> Done2((Done 2))
    T3[Thread 3] -- waits for Done 2 --> third()
```

1. **State Tracking**: We need a variable to track the current stage (e.g., `stage = 1`).
2. **Waiting**: Threads 2 and 3 should block (using a `Condvar` or similar) until the `stage` variable is updated.
3. **Signaling**: Thread 1 updates `stage` and signals the blocked threads.

## 4. Rust Implementation (Educational)

In the `mod.rs` file, you will implement this using common Rust synchronization primitives.

### Key Concepts to Practice:
- `Arc` for sharing the same `Foo` instance across threads.
- `Mutex<i32>` for managing the `stage` variable.
- `Condvar` for efficient waiting and signaling.
- Thinking about why simple atomics (`AtomicU32`) might also work here.
