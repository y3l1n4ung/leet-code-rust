# Thinking: Logger Rate Limiter

## Problem Understanding
A simple rate limiting problem for message logging.
- `should_print_message(timestamp, message)`: Returns `true` if it's been at least 10 seconds since the last time this message was printed.

## Data Structure Design
- **Message History**: Use a `HashMap<String, i32>` to store the timestamp of the last time each unique message was printed.

## Scalability Considerations
- **Memory**: The `HashMap` grows with the number of unique messages.
- **Eviction**: If memory becomes an issue, we could periodically remove old entries that are more than 10 seconds old relative to the current time, though the problem description doesn't explicitly require it.
