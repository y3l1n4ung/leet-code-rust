# Maximum Depth of Binary Tree - Thinking 🦀

## Problem Statement
Given the root of a binary tree, return its maximum depth.
A binary tree's maximum depth is the number of nodes along the longest path from the root node down to the farthest leaf node.

## Theoretical Explanation
This problem is best solved using a recursive Depth-First Search (DFS) approach.

### Key Logic
- If the root is `None`, depth is `0`.
- Otherwise, the depth is `1 + max(depth(left), depth(right))`.

### Complexity
- **Time**: O(n), since we visit every node exactly once.
- **Space**: O(h), where `h` is the height of the tree (due to recursive call stack).
