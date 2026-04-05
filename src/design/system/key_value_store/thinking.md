# System Design Thinking: Key-Value Store

A Key-Value (KV) store is a non-relational database that stores data as a collection of key-value pairs. Designing a distributed KV store requires balancing consistency, availability, and partition tolerance.

## 1. Requirements

### Functional Requirements
- `put(key, value)`: Insert or update a value associated with a key.
- `get(key)`: Retrieve the value associated with a key.

### Non-Functional Requirements
- **Scalability**: Handle large amounts of data by distributing it across multiple nodes.
- **High Availability**: Remain operational even if some nodes fail.
- **Tunable Consistency**: Allow users to choose between strong and eventual consistency.
- **Low Latency**: Provide fast read and write operations.

## 2. API Design

```rust
pub trait KVStore {
    fn put(&mut self, key: String, value: String);
    fn get(&self, key: &str) -> Option<String>;
}
```

## 3. Key Architectural Concepts

### Data Partitioning (Sharding)
- **Consistent Hashing**: Use consistent hashing to distribute data across nodes. This minimizes data movement when nodes are added or removed.

### Data Replication
- Replicate data across $N$ nodes to ensure high availability and durability.

### Consistency (CAP Theorem)
- **Consistency (C)**: Every read receives the most recent write or an error.
- **Availability (A)**: Every request receives a (non-error) response.
- **Partition Tolerance (P)**: The system continues to operate despite an arbitrary number of messages being dropped or delayed by the network between nodes.
- *In a distributed system, you can only pick two.*

### Quorum Consensus
- For a system with $N$ replicas:
    - $W$: Write Quorum (number of nodes that must acknowledge a write).
    - $R$: Read Quorum (number of nodes that must respond to a read).
- If $W + R > N$, you generally achieve strong consistency.

## 4. Rust Implementation (Educational)

In the `mod.rs` file, you will implement a **simple in-memory version** of a KV store.

### Key Concepts to Practice:
- `HashMap` for the core storage.
- Thinking about how you would extend this to a distributed system (e.g., using a mock network or consistent hashing logic).
- Thread-safety using `Arc<RwLock<T>>` if you were to make it concurrent.
