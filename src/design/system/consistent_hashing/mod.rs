/// [System Design] Consistent Hashing
/// Topic: Distributed Systems, Load Balancing
/// Tags: Hashing, Sharding, Scalability
///
/// Link: https://bytebytego.com/courses/system-design-interview/consistent-hashing
use std::collections::BTreeMap;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

pub struct ConsistentHashRing {
    /// Mapping from hash to node name
    ring: BTreeMap<u64, String>,
    /// Number of virtual nodes per physical node
    vnodes_per_node: usize,
}

impl ConsistentHashRing {
    pub fn new(vnodes_per_node: usize) -> Self {
        Self {
            ring: BTreeMap::new(),
            vnodes_per_node,
        }
    }

    /// Adds a physical node to the ring using virtual nodes
    pub fn add_node(&mut self, node_name: &str) {
        todo!("Hash the node_name multiple times (vnodes) and insert into the BTreeMap")
    }

    /// Removes a physical node (and its virtual nodes) from the ring
    pub fn remove_node(&mut self, node_name: &str) {
        todo!("Identify and remove all hashes belonging to this node from the ring")
    }

    /// Finds the node responsible for a given key
    pub fn get_node(&self, key: &str) -> Option<String> {
        todo!("Hash the key and find the first node clockwise on the ring")
    }

    /// Helper to compute hash of a string
    fn hash(key: &str) -> u64 {
        let mut s = DefaultHasher::new();
        key.hash(&mut s);
        s.finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_placement() {
        let mut ring = ConsistentHashRing::new(10);
        ring.add_node("node1");
        ring.add_node("node2");
        ring.add_node("node3");

        let n1 = ring.get_node("user_123");
        let n2 = ring.get_node("user_456");

        assert!(n1.is_some());
        assert!(n2.is_some());
    }

    #[test]
    fn test_rebalancing() {
        let mut ring = ConsistentHashRing::new(10);
        ring.add_node("node1");
        ring.add_node("node2");

        let original_node = ring.get_node("some_key").unwrap();

        // Add a new node
        ring.add_node("node3");
        let current_node = ring.get_node("some_key").unwrap();

        // The node should either be the same or the new one
        assert!(current_node == original_node || current_node == "node3");
    }

    #[test]
    fn test_empty_ring() {
        let ring = ConsistentHashRing::new(1);
        assert_eq!(ring.get_node("any_key"), None);
    }
}
