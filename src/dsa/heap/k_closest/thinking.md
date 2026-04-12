# K Closest Points to Origin - Thinking 🦀

## Problem Statement
Given an array of points where `points[i] = [xi, yi]` and an integer `k`, return the `k` closest points to the origin `(0, 0)`.

## Theoretical Explanation
The distance between two points `(x1, y1)` and `(x2, y2)` is `sqrt((x1 - x2)^2 + (y1 - y2)^2)`.
Since we are comparing distances to the origin `(0,0)`, we can simply compare `x^2 + y^2` to avoid `sqrt` and floating-point issues.

### Using a Max-Heap
- We can use a **Max-Heap** of size `k`.
- For each point:
  - If the heap has fewer than `k` elements, push the current point.
  - If the current point is closer than the point at the top of the heap, pop the top and push the current point.
- After iterating through all points, the heap will contain the `k` closest points.

### Complexity
- **Time**: O(n log k), where n is the number of points.
- **Space**: O(k) to store the heap.

## Rust Implementation Detail
In Rust, `std::collections::BinaryHeap` is a **Max-Heap** by default. We'll need a way to compare points by their squared distance.
