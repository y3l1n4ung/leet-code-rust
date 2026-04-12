# Breadth-First Search (BFS) Traversal - Thinking 🦀

## Theoretical Explanation
BFS is a graph traversal algorithm that explores all the neighbor nodes at the present depth prior to moving on to nodes at the next depth level.

### Key Logic
- Use a **Queue** (`std::collections::VecDeque`).
- Keep track of **Visited** nodes using a `HashSet` to avoid infinite loops.

### Complexity
- **Time**: O(V + E) where V is vertices and E is edges.
- **Space**: O(V) for the queue and visited set.

## Implementation Pattern
1. Start by pushing the source node into the queue and marking it as visited.
2. While the queue is not empty:
   - Pop the first node.
   - For each unvisited neighbor:
     - Mark as visited.
     - Push into the queue.
