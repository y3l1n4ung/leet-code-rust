# Learning Rust with LeetCode

This repository is an educational lab designed to help you master Rust through the lens of algorithmic problem-solving. It follows the **NeetCode 150** curriculum, providing a structured path from basic data structures to complex systems.

## 🎓 The Learning Journey

Don't just solve problems; use them to understand **why** Rust works the way it does.

### 1. Choose Your Path
- **By Difficulty**: Start with `src/easy/` to build confidence.
- **By Pattern**: Use `src/patterns/` to learn how to recognize and solve classes of problems (e.g., Two Pointers, Sliding Window).

### 2. The Learning Loop
1. **Understand the Problem**: Read the metadata and requirements in the challenge file (e.g., `src/easy/two_sum.rs`).
2. **Implement**: Replace `todo!()` with your logic.
3. **Verify**: Run the local tests to get immediate feedback:
   ```bash
   cargo test --lib easy::two_sum
   ```
4. **Refactor**: Once it passes, look for more "idiomatic" Rust ways to write it (using iterators, matches, etc.).

### 3. Mastering Rust Concepts
LeetCode challenges in Rust specifically highlight:
- **Ownership & Borrowing**: Handling `Box<ListNode>` or `Rc<RefCell<TreeNode>>`.
- **The Iterator Pattern**: Using `.map()`, `.filter()`, and `.collect()` instead of manual loops.
- **Memory Safety**: Understanding why certain pointer-heavy algorithms require specific Rust wrappers.

## 💡 Tips for Learners

- **Use `dbg!()`**: It prints the variable name, value, and line number—perfect for tracing logic.
- **Complexity Matters**: Every file is an opportunity to practice Big O analysis.
- **Read the Signatures**: The function signatures provided (like `Option<Box<T>>`) are intentional—they teach you how Rust handles nullability and heap allocation.

---
*Happy Learning! Focus on the concepts, and the solutions will follow.* 🦀
