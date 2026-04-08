# Thinking: Cache with Expiration (TTL)

## Problem Understanding
A common caching pattern where items have a Time-To-Live (TTL).
- `set(key, value, ttl)`: Store a value that is valid for a specific duration.
- `get(key)`: Retrieve the value if it exists and is still valid.
- `cleanup()`: Manually purge expired items.

## Data Structure Design
- **Storage**: `HashMap<K, (V, Instant)>`. The `Instant` marks when the entry was created.

## Scalability Considerations
- **Eviction Policies**:
  - **Passive Eviction**: Check for expiration on `get()`. If expired, remove and return `None`.
  - **Active Eviction**: A background thread or a manual `cleanup()` call that iterates over the entire cache.
- **Precision**: Using `Instant` provides high-resolution timing, ideal for short TTLs.
