# Search a 2D Matrix - Thinking 🦀

## Problem Statement
Given an `m x n` integer matrix with the following properties:
1. Each row is sorted in non-decreasing order.
2. The first integer of each row is greater than the last integer of the previous row.

## Theoretical Explanation
Because of the properties, we can treat the 2D matrix as a **single sorted 1D array** of length `m * n`.

### Mapping 1D Index to 2D Coordinates
- **Row**: `index / cols`
- **Col**: `index % cols`

### Complexity
- **Time**: O(log(m * n))
- **Space**: O(1)
