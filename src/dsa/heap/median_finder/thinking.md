# Find Median from Data Stream - Thinking 🦀

## Problem Statement
Implement a data structure that supports two operations:
1. `add_num(num: i32)`: Add an integer from the data stream.
2. `find_median() -> f64`: Return the median of all elements seen so far.

## Theoretical Explanation
A powerful technique for this is using **Two Heaps**:
1. **Max-Heap** (`small`): To store the smaller half of the numbers.
2. **Min-Heap** (`large`): To store the larger half of the numbers.

### Key Logic
- Every element in `small` must be <= every element in `large`.
- The difference in size between `small` and `large` must be <= 1.

### Insertion Process
1. Push `num` into `small`.
2. Move the largest element from `small` to `large` to maintain order.
3. If `large` becomes bigger than `small`, move the smallest element from `large` to `small`.

### Finding Median
- If `small.len() > large.len()`, the median is the top of `small`.
- Else, the median is `(small.top() + large.top()) / 2.0`.

### Complexity
- **Time**: O(log n) for `add_num`, O(1) for `find_median`.
- **Space**: O(n), to store all numbers.
