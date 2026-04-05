/// [System Design] URL Shortener
/// Topic: Distributed Systems, Web Architecture
/// Tags: Base62, TinyURL, Redirection
///
/// Link: https://bytebytego.com/courses/system-design-interview/design-a-url-shortener

use std::collections::HashMap;

pub struct TinyUrl {
    db: HashMap<String, String>,
    counter: u64,
}

impl TinyUrl {
    pub fn new() -> Self {
        Self {
            db: HashMap::new(),
            counter: 10000, // Start with a non-zero counter for interesting Base62 strings
        }
    }

    /// Generates a short URL for the given long URL
    pub fn shorten(&mut self, long_url: String) -> String {
        todo!("Generate a unique ID, encode it in Base62, and store the mapping")
    }

    /// Returns the long URL for the given short URL
    pub fn resolve(&self, short_url: &str) -> Option<String> {
        todo!("Resolve the short URL from the database")
    }

    /// Helper function to encode a number to Base62
    fn encode_base62(mut num: u64) -> String {
        todo!("Implement the Base62 encoding algorithm")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shorten_and_resolve() {
        let mut shortener = TinyUrl::new();
        let long_url = "https://www.rust-lang.org/".to_string();
        
        let short_url = shortener.shorten(long_url.clone());
        assert!(!short_url.is_empty());
        assert_eq!(shortener.resolve(&short_url), Some(long_url));
    }

    #[test]
    fn test_multiple_urls() {
        let mut shortener = TinyUrl::new();
        let url1 = "https://google.com".to_string();
        let url2 = "https://github.com".to_string();
        
        let s1 = shortener.shorten(url1.clone());
        let s2 = shortener.shorten(url2.clone());
        
        assert_ne!(s1, s2);
        assert_eq!(shortener.resolve(&s1), Some(url1));
        assert_eq!(shortener.resolve(&s2), Some(url2));
    }

    #[test]
    fn test_nonexistent() {
        let shortener = TinyUrl::new();
        assert_eq!(shortener.resolve("nonexistent"), None);
    }
}
