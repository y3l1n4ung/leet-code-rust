# System Design Thinking: Design Browser History

A browser history system allows you to visit URLs, go back in history, and go forward in history.

## 1. Requirements

### Functional Requirements
- `visit(url)`: Visits `url` from the current page. It clears up all the forward history.
- `back(steps)`: Move `steps` back in history. If you can only return $x$ steps where $x < steps$, return $x$ steps. Return the current URL after moving back.
- `forward(steps)`: Move `steps` forward in history. If you can only move forward $x$ steps where $x < steps$, move forward $x$ steps. Return the current URL after moving forward.

## 2. API Design

```rust
pub struct BrowserHistory {
    // Add your internal state here
}

impl BrowserHistory {
    pub fn new(homepage: String) -> Self;
    pub fn visit(&mut self, url: String);
    pub fn back(&mut self, steps: i32) -> String;
    pub fn forward(&mut self, steps: i32) -> String;
}
```

## 3. High-Level Architecture

We can choose between two main approaches to implement the history efficiently.

### Approach 1: Two Stacks
- **`history` stack**: Stores the past URLs (including the current one).
- **`future` stack**: Stores the URLs that were "backed" out of. When you `visit` a new URL, this stack is cleared.

### Approach 2: Dynamic Array (Vec) with Pointer
- **`history`: `Vec<String>`**: Stores the URLs.
- **`curr`: `usize`**: An index (pointer) to the current URL in the `history` vector.
- **`max`: `usize`**: An index representing the end of the current valid history (when we `visit` a new URL, we overwrite anything after the current pointer and set `max` to the new position).

## 4. Key Design Decisions

### Efficiency of `visit`
- Using the **Dynamic Array** approach is often more intuitive in Rust as it avoids frequent stack operations. 
- When a new URL is visited, any "forward" history must be invalidated. In the array approach, this simply means setting the new `max` index.

### Memory Management
- Since URLs are strings, be mindful of how you store them in the vector. 

## 5. Rust Implementation (Educational)

In the `mod.rs` file, you will implement this using a `Vec<String>` and an index.

### Key Concepts to Practice:
- Managing indices and boundaries.
- Using `std::cmp::min` and `std::cmp::max` for step calculations.
- Handling empty/initial state.
