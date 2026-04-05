/// [System Design] Notification System
/// Topic: Distributed Systems, Message Queues
/// Tags: Reliability, Retry, Scalability
///
/// Link: https://bytebytego.com/courses/system-design-interview/design-a-notification-system

use std::collections::HashMap;

/// Trait for a notification provider (e.g., Email, SMS, Push)
pub trait NotificationProvider {
    fn send(&self, user_id: &str, message: &str) -> Result<(), String>;
}

pub struct NotificationService {
    providers: HashMap<String, Box<dyn NotificationProvider>>,
    // Mock queue for demonstration
    queue: Vec<(String, String, String)>, // (provider_type, user_id, message)
}

impl NotificationService {
    pub fn new() -> Self {
        Self {
            providers: HashMap::new(),
            queue: Vec::new(),
        }
    }

    /// Registers a provider for a specific notification type
    pub fn register_provider(&mut self, provider_type: String, provider: Box<dyn NotificationProvider>) {
        self.providers.insert(provider_type, provider);
    }

    /// Enqueues a notification to be sent later
    pub fn enqueue_notification(&mut self, provider_type: &str, user_id: String, message: String) {
        todo!("Add the notification to the internal queue for later processing")
    }

    /// Processes all notifications in the queue
    pub fn process_queue(&mut self) -> Vec<Result<(), String>> {
        todo!("Pull each notification from the queue and send it via the registered provider")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Mutex;
    use std::sync::Arc;

    // Mock provider for testing
    struct MockProvider {
        sent_messages: Arc<Mutex<Vec<(String, String)>>>,
    }

    impl MockProvider {
        fn new() -> (Self, Arc<Mutex<Vec<(String, String)>>>) {
            let sent_messages = Arc::new(Mutex::new(Vec::new()));
            (Self { sent_messages: sent_messages.clone() }, sent_messages)
        }
    }

    impl NotificationProvider for MockProvider {
        fn send(&self, user_id: &str, message: &str) -> Result<(), String> {
            self.sent_messages.lock().unwrap().push((user_id.to_string(), message.to_string()));
            Ok(())
        }
    }

    #[test]
    fn test_notification_flow() {
        let mut service = NotificationService::new();
        let (email_provider, sent_emails) = MockProvider::new();
        let (sms_provider, sent_sms) = MockProvider::new();
        
        service.register_provider("email".to_string(), Box::new(email_provider));
        service.register_provider("sms".to_string(), Box::new(sms_provider));
        
        service.enqueue_notification("email", "user1".to_string(), "Welcome!".to_string());
        service.enqueue_notification("sms", "user2".to_string(), "Your code is 1234".to_string());
        
        let results = service.process_queue();
        assert_eq!(results.len(), 2);
        assert!(results.iter().all(|r| r.is_ok()));
        
        assert_eq!(sent_emails.lock().unwrap().len(), 1);
        assert_eq!(sent_sms.lock().unwrap().len(), 1);
    }
}
