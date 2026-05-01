/// [System Design] Message Queue
/// Topic: Distributed Systems, Messaging
/// Tags: Producer, Consumer, PubSub, Decoupling
///
/// Link: https://bytebytego.com/courses/system-design-interview/design-a-message-queue
use std::collections::{HashMap, VecDeque};

#[derive(Clone, Debug, PartialEq)]
pub struct Message {
    pub id: String,
    pub payload: String,
}

pub struct MessageQueue {
    /// In-memory storage for messages
    queues: HashMap<String, VecDeque<Message>>,
    /// Messages that are being processed but not yet acknowledged
    inflight: HashMap<String, Message>,
}

impl MessageQueue {
    pub fn new() -> Self {
        Self {
            queues: HashMap::new(),
            inflight: HashMap::new(),
        }
    }

    /// Producer: Sends a message to a specific topic
    pub fn produce(&mut self, topic: &str, message: Message) {
        todo!("Add the message to the queue for the specified topic")
    }

    /// Consumer: Pulls a message from a specific topic
    pub fn consume(&mut self, topic: &str) -> Option<Message> {
        todo!("Pull a message from the queue and track it in the inflight map")
    }

    /// Consumer: Acknowledges successful processing of a message
    pub fn acknowledge(&mut self, message_id: &str) {
        todo!("Remove the message from the inflight map")
    }

    /// Broker: Retries processing of messages that were not acknowledged
    pub fn retry_inflight(&mut self, topic: &str) {
        todo!("Identify and re-queue messages that have been in-flight for too long")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_produce_consume() {
        let mut mq = MessageQueue::new();
        let msg = Message {
            id: "1".to_string(),
            payload: "Hello".to_string(),
        };

        mq.produce("test_topic", msg.clone());
        let consumed = mq.consume("test_topic");

        assert_eq!(consumed, Some(msg));
    }

    #[test]
    fn test_acknowledgement() {
        let mut mq = MessageQueue::new();
        let msg = Message {
            id: "1".to_string(),
            payload: "Important".to_string(),
        };

        mq.produce("orders", msg.clone());
        let consumed = mq.consume("orders").unwrap();

        mq.acknowledge(&consumed.id);

        // Ensure it's not re-queued
        mq.retry_inflight("orders");
        assert_eq!(mq.consume("orders"), None);
    }

    #[test]
    fn test_retry_without_ack() {
        let mut mq = MessageQueue::new();
        let msg = Message {
            id: "1".to_string(),
            payload: "Retry me".to_string(),
        };

        mq.produce("alerts", msg.clone());
        let _ = mq.consume("alerts");

        // Simulate no ACK and a retry trigger
        mq.retry_inflight("alerts");

        // Should be available again
        assert_eq!(mq.consume("alerts"), Some(msg));
    }
}
