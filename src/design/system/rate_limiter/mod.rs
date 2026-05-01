/// [System Design] Rate Limiter
/// Topic: Distributed Systems, Algorithms
/// Tags: TokenBucket, LeakyBucket, FixedWindow
///
/// Link: https://bytebytego.com/courses/system-design-interview/design-a-rate-limiter
use std::collections::HashMap;
use std::time::{Duration, Instant};

/// A simple Token Bucket Rate Limiter
pub struct TokenBucket {
    capacity: f64,
    tokens: f64,
    refill_rate: f64, // tokens per second
    last_refill: Instant,
}

impl TokenBucket {
    pub fn new(capacity: f64, refill_rate: f64) -> Self {
        Self {
            capacity,
            tokens: capacity,
            refill_rate,
            last_refill: Instant::now(),
        }
    }

    pub fn allow_request(&mut self) -> bool {
        todo!("Implement the allow_request logic using the token bucket algorithm")
    }

    fn refill(&mut self) {
        todo!("Implement the refill logic based on elapsed time")
    }
}

pub struct RateLimiter {
    clients: HashMap<String, TokenBucket>,
    default_capacity: f64,
    default_refill_rate: f64,
}

impl RateLimiter {
    pub fn new(capacity: f64, refill_rate: f64) -> Self {
        Self {
            clients: HashMap::new(),
            default_capacity: capacity,
            default_refill_rate: refill_rate,
        }
    }

    pub fn allow_request(&mut self, client_id: &str) -> bool {
        todo!("Implement the per-client rate limiting logic")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;

    #[test]
    fn test_token_bucket_basic() {
        // Note: This test will fail until you implement allow_request and refill
        let mut limiter = RateLimiter::new(2.0, 100.0); // High refill rate for testing

        assert!(limiter.allow_request("client1"));
        assert!(limiter.allow_request("client1"));
        assert!(!limiter.allow_request("client1"));
    }

    #[test]
    fn test_multi_client() {
        let mut limiter = RateLimiter::new(1.0, 1.0);

        assert!(limiter.allow_request("client1"));
        assert!(limiter.allow_request("client2"));

        assert!(!limiter.allow_request("client1"));
        assert!(!limiter.allow_request("client2"));
    }
}
