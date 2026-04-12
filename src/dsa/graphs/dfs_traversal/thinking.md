# Depth-First Search (DFS) Traversal - Thinking 🦀

## Theoretical Explanation
DFS is a graph traversal algorithm that explores as far as possible along each branch before backtracking.

### Key Logic
- Use **Recursion** (Implicit stack) or an **Explicit Stack**.
- Keep track of **Visited** nodes using a `HashSet` to avoid cycles and infinite loops.

### Complexity
- **Time**: O(V + E) where V is vertices and E is edges.
- **Space**: O(V) for the visited set and recursive call stack.

## Implementation Pattern
1. Start with a `HashSet` to keep track of visited nodes.
2. For the current node:
   - Mark as visited.
   - For each neighbor that hasn't been visited:
     - Recursively call DFS.
