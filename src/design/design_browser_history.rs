/// [1472] Design Browser History
/// Difficulty: Medium
/// Topics: Array, Linked List, Stack, Design, Doubly-Linked List, Data Stream
/// Tags: RustMastery
///
/// You have a browser of one tab where you start on the homepage and you can visit another url, get back in the history number of steps or move forward in the history number of steps.
///
/// Link: https://leetcode.com/problems/design-browser-history/

struct BrowserHistory {
    todo: ()
}

impl BrowserHistory {
    pub fn new(homepage: String) -> Self {
        todo!()
    }
    
    pub fn visit(&self, url: String) {
        todo!()
    }
    
    pub fn back(&self, steps: i32) -> String {
        todo!()
    }
    
    pub fn forward(&self, steps: i32) -> String {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        // let bh = BrowserHistory::new("leetcode.com".to_string());
        // bh.visit("google.com".to_string());
        // bh.visit("facebook.com".to_string());
        // bh.visit("youtube.com".to_string());
        // assert_eq!(bh.back(1), "facebook.com".to_string());
        // assert_eq!(bh.back(1), "google.com".to_string());
        // assert_eq!(bh.forward(1), "facebook.com".to_string());
        // bh.visit("linkedin.com".to_string());
        // assert_eq!(bh.forward(2), "linkedin.com".to_string());
        // assert_eq!(bh.back(2), "google.com".to_string());
        // assert_eq!(bh.back(7), "leetcode.com".to_string());
        todo!();
    }
}
