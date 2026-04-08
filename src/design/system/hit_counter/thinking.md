# Thinking: Hit Counter

## Problem Understanding
The goal is to count the number of hits in the last 5 minutes (300 seconds).
- `hit(timestamp)`: Record a hit at a given time.
- `get_hits(timestamp)`: Get total hits in `[timestamp - 300 + 1, timestamp]`.

## Scalability Considerations
- **Memory**: If we store every hit as a timestamp in a `Vec`, it can grow indefinitely.
- **Precision**: We only care about the last 300 seconds.
- **Optimization**: We can use a fixed-size array (circular buffer) of size 300 to store hit counts for each second.

## Implementation Ideas
1. **Simple Vector**: Store all timestamps. `get_hits` uses binary search to find the range.
2. **Circular Buffer**: Two arrays of size 300: one for `times` and one for `counts`.
   - `index = timestamp % 300`
   - If `times[index] != timestamp`, it's a new second; reset `counts[index] = 1` and `times[index] = timestamp`.
   - Else, increment `counts[index]`.
