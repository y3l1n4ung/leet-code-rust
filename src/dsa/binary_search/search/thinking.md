# Classic Binary Search - Thinking 🦀

## Theoretical Explanation
Binary search narrows down a target in a **sorted** range by checking the middle and halving the search space each time.

### Complexity
- **Time**: O(log n)
- **Space**: O(1)

## Key Implementation
- Calculate `mid` with `left + (right - left) / 2`.
- Update `left` to `mid + 1` or `right` to `mid - 1`.
