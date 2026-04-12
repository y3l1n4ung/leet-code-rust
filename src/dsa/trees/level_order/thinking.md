# Binary Tree Level Order Traversal - Thinking 🦀

## Problem Statement
Given the root of a binary tree, return the level order traversal of its nodes' values. (i.e., from left to right, level by level).

## Theoretical Explanation
This is a classic Breadth-First Search (BFS) on a tree. 

### Key Logic
- Use a **Queue** to store nodes to be visited.
- At each level, determine the number of nodes in the queue (`level_size`).
- Iterate `level_size` times to process all nodes at the current level.
- Collect their values into a `level_vec` and push their children into the queue.

### Complexity
- **Time**: O(n), where n is the number of nodes.
- **Space**: O(n), to store the queue (max width of the tree).
