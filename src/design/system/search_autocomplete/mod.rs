/// [System Design] Search Autocomplete
/// Topic: Data Structures, Algorithms
/// Tags: Trie, TopK, Suggestion
///
/// Link: https://bytebytego.com/courses/system-design-interview/design-search-autocomplete-system

use std::collections::HashMap;

#[derive(Default)]
struct TrieNode {
    children: HashMap<char, TrieNode>,
    /// Maps full search terms to their frequencies (only at the end of a word)
    frequencies: HashMap<String, u32>,
}

pub struct AutocompleteSystem {
    root: TrieNode,
}

impl AutocompleteSystem {
    pub fn new() -> Self {
        Self {
            root: TrieNode::default(),
        }
    }

    /// Inserts a search term with its frequency into the Trie
    pub fn insert(&mut self, term: String, frequency: u32) {
        todo!("Implement the iterative or recursive Trie insertion logic")
    }

    /// Returns the top k suggestions for the given prefix
    pub fn get_suggestions(&self, prefix: &str, k: usize) -> Vec<String> {
        todo!("Traverse to the prefix node, collect all reachable terms, and return the top k by frequency")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_autocomplete_basic() {
        let mut system = AutocompleteSystem::new();
        system.insert("apple".to_string(), 10);
        system.insert("apply".to_string(), 5);
        system.insert("app".to_string(), 15);
        system.insert("banana".to_string(), 20);

        let suggestions = system.get_suggestions("app", 3);
        assert_eq!(suggestions.len(), 3);
        // Order should be "app" (15), "apple" (10), "apply" (5)
        assert_eq!(suggestions[0], "app");
        assert_eq!(suggestions[1], "apple");
        assert_eq!(suggestions[2], "apply");
    }

    #[test]
    fn test_autocomplete_prefix_not_found() {
        let mut system = AutocompleteSystem::new();
        system.insert("rust".to_string(), 100);
        
        let suggestions = system.get_suggestions("go", 5);
        assert!(suggestions.is_empty());
    }

    #[test]
    fn test_autocomplete_k_limit() {
        let mut system = AutocompleteSystem::new();
        system.insert("a1".to_string(), 10);
        system.insert("a2".to_string(), 20);
        system.insert("a3".to_string(), 30);
        
        let suggestions = system.get_suggestions("a", 2);
        assert_eq!(suggestions.len(), 2);
        assert_eq!(suggestions[0], "a3");
        assert_eq!(suggestions[1], "a2");
    }
}
