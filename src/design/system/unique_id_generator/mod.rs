/// [System Design] Unique ID Generator
/// Topic: Distributed Systems, Scalability
/// Tags: Snowflake, UUID, DistributedSystems
///
/// Link: https://bytebytego.com/courses/system-design-interview/design-a-unique-id-generator-in-distributed-systems

use std::time::{SystemTime, UNIX_EPOCH};

pub struct SnowflakeGenerator {
    worker_id: u64,
    datacenter_id: u64,
    sequence: u64,
    last_timestamp: u64,
    
    // Snowflake Structure (64 bits)
    // 1 bit: unused
    // 41 bits: timestamp
    // 5 bits: datacenter id
    // 5 bits: worker id
    // 12 bits: sequence
}

impl SnowflakeGenerator {
    pub fn new(worker_id: u64, datacenter_id: u64) -> Self {
        Self {
            worker_id,
            datacenter_id,
            sequence: 0,
            last_timestamp: 0,
        }
    }

    /// Generates a unique 64-bit Snowflake ID
    pub fn next_id(&mut self) -> Result<u64, String> {
        todo!("Implement Snowflake ID generation logic with bitwise operations")
    }

    /// Helper function to get current timestamp in milliseconds
    fn current_timestamp() -> u64 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis() as u64
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_uniqueness() {
        let mut generator = SnowflakeGenerator::new(1, 1);
        let mut ids = std::collections::HashSet::new();
        
        for _ in 0..1000 {
            if let Ok(id) = generator.next_id() {
                assert!(ids.insert(id), "Duplicate ID generated: {}", id);
            }
        }
    }

    #[test]
    fn test_sortable_by_time() {
        let mut generator = SnowflakeGenerator::new(1, 1);
        
        let id1 = generator.next_id().unwrap();
        std::thread::sleep(std::time::Duration::from_millis(10));
        let id2 = generator.next_id().unwrap();
        
        assert!(id1 < id2, "IDs should be roughly sortable by time");
    }
}
