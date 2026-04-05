/// [LeetCode 1472] Design Browser History
/// Difficulty: Medium
/// Topics: Array, Stack, Design
///
/// Link: https://leetcode.com/problems/design-browser-history/

pub struct BrowserHistory {
    history: Vec<String>,
    current_index: usize,
    max_index: usize,
}

impl BrowserHistory {
    pub fn new(homepage: String) -> Self {
        todo!("Initialize the history with the homepage")
    }

    pub fn visit(&mut self, url: String) {
        todo!("Visit a new URL and clear the forward history")
    }

    pub fn back(&mut self, steps: i32) -> String {
        todo!("Move back in history by the specified number of steps")
    }

    pub fn forward(&mut self, steps: i32) -> String {
        todo!("Move forward in history by the specified number of steps")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_browser_history_basic() {
        let mut history = BrowserHistory::new("leetcode.com".to_string());
        history.visit("google.com".to_string());
        history.visit("facebook.com".to_string());
        history.visit("youtube.com".to_string());
        
        assert_eq!(history.back(1), "facebook.com");
        assert_eq!(history.back(1), "google.com");
        assert_eq!(history.forward(1), "facebook.com");
        
        history.visit("linkedin.com".to_string()); // google.com -> facebook.com -> linkedin.com (youtube.com is cleared)
        assert_eq!(history.forward(2), "linkedin.com"); // can't go forward
        assert_eq!(history.back(2), "google.com");
        assert_eq!(history.back(7), "leetcode.com"); // stay at homepage
    }
}
