/// [LeetCode 359] Logger Rate Limiter
/// Difficulty: Easy
/// Topics: Hash Table, Design
///
/// Link: https://leetcode.com/problems/logger-rate-limiter/

use std::collections::HashMap;

pub struct Logger {
    messages: HashMap<String, i32>,
}

impl Logger {
    pub fn new() -> Self {
        Self {
            messages: HashMap::new(),
        }
    }

    /// Returns true if the message should be printed in the given timestamp, otherwise returns false.
    /// If this method returns true, the message will be printed.
    /// The timestamp is in seconds granularity.
    pub fn should_print_message(&mut self, timestamp: i32, message: String) -> bool {
        todo!("Check if the message should be printed based on the 10-second rule")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_logger_basic() {
        let mut logger = Logger::new();
        assert!(logger.should_print_message(1, "foo".to_string()));
        assert!(logger.should_print_message(2, "bar".to_string()));
        assert!(!logger.should_print_message(3, "foo".to_string()));
        assert!(!logger.should_print_message(8, "bar".to_string()));
        assert!(!logger.should_print_message(10, "foo".to_string()));
        assert!(logger.should_print_message(11, "foo".to_string()));
    }
}
