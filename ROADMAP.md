# Roadmap & Future Plans

## 🎯 Phase 1: Core Implementation
- [ ] Implement all Blind 75 subset challenges (The essential set).
- [ ] Implement remaining NeetCode 150 challenges.
- [ ] Document complexity (Time & Space) for each solution.

## 🛠 Phase 2: Tooling & Utils
- [ ] Create `src/utils.rs` for:
    - `Vec` to `ListNode` conversion (for easier testing).
    - `Vec` to `TreeNode` conversion (level-order input to Tree).
- [ ] Add `criterion.rs` for advanced benchmarking.

## 📂 Phase 3: Alternative Organizations
Instead of just `easy/medium/hard`, we plan to reorganize or tag challenges by:

### 1. By Algorithmic Pattern
Organizing by the **way** you solve them:
- `pattern/sliding_window`
- `pattern/two_pointers`
- `pattern/fast_slow_pointers`
- `pattern/backtracking`
- `pattern/dynamic_programming`

### 2. By Data Structure (Topic)
- `topic/arrays_and_hashing`
- `topic/linked_lists`
- `topic/trees`
- `topic/graphs`
- `topic/heaps`

### 3. By "Curated Lists"
- `list/blind75`
- `list/neetcode150`
- `list/grind75`

## 🧪 Phase 4: CI/CD
- [ ] Set up GitHub Actions to run `cargo test` on every push.
- [ ] Automate progress reporting in `README.md`.
