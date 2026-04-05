# NeetCode 150 Learning Path

This roadmap is a curriculum designed to take you from Rust fundamentals to advanced algorithmic mastery. Each stage builds on the last, introducing new Rust concepts alongside classic patterns.

## 🧱 Level 1: The Foundations
Focus on vectors, strings, and hash tables.
- [ ] **Arrays & Hashing**: Learn to use `Vec`, `String`, and `HashMap`.
- [ ] **Two Pointers**: Master indexed access and simple loops.
- [ ] **Sliding Window**: Introduction to efficient sub-array traversal.
- **Rust Focus**: Basic ownership, borrowing, and the `mut` keyword.

## 🏗 Level 2: Data Structures & Pointers
Focus on more complex memory management.
- [ ] **Stack**: Using vectors as LIFO structures.
- [ ] **Linked Lists**: Master the `Option<Box<ListNode>>` pattern.
- [ ] **Trees**: Learn to work with `Option<Rc<RefCell<TreeNode>>>`.
- **Rust Focus**: Heap allocation (`Box`), Reference Counting (`Rc`), and Interior Mutability (`RefCell`).

## 🔍 Level 3: Advanced Logic
Focus on recursion, state management, and complex traversals.
- [ ] **Binary Search**: Fine-tune your control over ranges and indices.
- [ ] **Backtracking**: Introduction to recursion and state restoration.
- [ ] **Graphs**: Use `HashMap` or `Vec<Vec<T>>` for adjacency lists.
- **Rust Focus**: Recursion, closures, and lifetime management in graph structures.

## 🚀 Level 4: Optimization & Efficiency
Focus on dynamic programming and advanced heuristics.
- [ ] **Dynamic Programming (1D/2D)**: Master state transitions and memoization.
- [ ] **Greedy Algorithms**: Learn to make locally optimal choices.
- [ ] **Tries**: Build prefix trees for efficient string prefix searches.
- [ ] **Heap / Priority Queue**: Master `std::collections::BinaryHeap`.
- **Rust Focus**: Advanced traits, generic programming, and standard library collections.

## 🌐 Level 5: System Design & Architecture
Focus on building scalable, distributed systems and advanced object-oriented structures.
- [ ] **Distributed Systems**: Learn patterns like Consistent Hashing, Rate Limiting, and Message Queues (`src/design/system/`).
- [ ] **Data Structure Design**: Implement custom complex structures like LFU Cache, LRU Cache, and Skiplists.
- **Rust Focus**: Concurrency (`Arc`, `Mutex`, `RwLock`), advanced traits (dynamic dispatch), and managing complex shared state in multi-threaded environments.

---
### 🚦 Next Steps
Each challenge in `src/` is a lesson. When you finish a challenge, don't just move on—**read another person's Rust solution** to see if there's a more efficient or "Rust-like" way to achieve the same result.
