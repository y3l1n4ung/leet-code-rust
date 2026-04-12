# Search in Rotated Sorted Array - Thinking 🦀

## Problem Statement
Given a sorted array `nums` that has been rotated at some pivot, find the index of a target value.

### Example
`[4, 5, 6, 7, 0, 1, 2]`, target = 0 -> output: 4.

## Theoretical Explanation
We can still achieve O(log n) by identifying which part of the search space is "strictly sorted."

### Key Logic
1. Calculate `mid`.
2. Check if `nums[left] <= nums[mid]`.
   - If true, the left half is sorted.
     - If `nums[left] <= target < nums[mid]`, target must be in the left half.
     - Else, it's in the right half.
3. Else, the right half must be sorted.
   - If `nums[mid] < target <= nums[right]`, target must be in the right half.
   - Else, it's in the left half.

### Complexity
- **Time**: O(log n)
- **Space**: O(1)
