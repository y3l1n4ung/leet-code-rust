# Rust LeetCode Practice Lab

A structured environment for practicing LeetCode challenges in Rust, based on the NeetCode 150 list.

## 🚀 Getting Started

Each challenge is organized into directories by difficulty: `easy`, `medium`, and `hard`.

### 🛠 Prerequisites
- [Rust](https://www.rust-lang.org/tools/install)
- [cargo-watch](https://github.com/watchexec/cargo-watch) (Optional, but highly recommended)
  ```bash
  cargo install cargo-watch
  ```

### 🏃 How to Practice
1. Browse to a challenge file, e.g., `src/easy/two_sum.rs`.
2. Implement your solution by replacing the `todo!()` macro.
3. Run tests for that specific challenge:
   ```bash
   cargo test --lib easy::two_sum
   ```
4. (Optional) Run tests automatically on save:
   ```bash
   cargo watch -x "test --lib easy::two_sum"
   ```

## 📊 Progress Tracker

- [ ] Easy (30/30 set up)
- [ ] Medium (96/96 set up)
- [ ] Hard (24/24 set up)

Total: **150** Challenges ready for implementation.

## 💡 Better Practice Style

- **Debugging**: Use `dbg!(variable)` to inspect values with file/line context.
- **Ownership**: Remember that LeetCode's `Option<Box<ListNode>>` and `Option<Rc<RefCell<TreeNode>>>` signatures are designed to teach you about Rust's memory management.
- **Benchmarks**: Add timing to your tests to measure performance:
  ```rust
  let start = std::time::Instant::now();
  // ... call function
  println!("Time: {:?}", start.elapsed());
  ```

---
Happy Coding! 🦀
