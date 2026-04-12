/// [208] Implement Trie (Prefix Tree)
/// Difficulty: Medium
/// Topics: Hash Table, String, Design, Trie
/// Tags: Blind75, NeetCode150
///
/// A trie (pronounced as "try") or prefix tree is a tree data structure used to efficiently store and retrieve keys in a dataset of strings. There are various applications of this data structure, such as autocomplete and spellchecker.
/// Implement the Trie class:
/// - Trie() Initializes the trie object.
/// - void insert(String word) Inserts the string word into the trie.
/// - boolean search(String word) Returns true if the string word is in the trie (i.e., was inserted before), and false otherwise.
/// - boolean startsWith(String prefix) Returns true if there is a previously inserted string word that has the prefix prefix, and false otherwise.
///
/// Link: https://leetcode.com/problems/implement-trie-prefix-tree/

struct Trie {
    todo: ()
}

impl Trie {
    pub fn new() -> Self {
        todo!()
    }
    
    pub fn insert(&self, word: String) {
        todo!()
    }
    
    pub fn search(&self, word: String) -> bool {
        todo!()
    }
    
    pub fn starts_with(&self, prefix: String) -> bool {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let trie = Trie::new();
        trie.insert("apple".to_string());
        assert_eq!(trie.search("apple".to_string()), true);
        assert_eq!(trie.search("app".to_string()), false);
        assert_eq!(trie.starts_with("app".to_string()), true);
        trie.insert("app".to_string());
        assert_eq!(trie.search("app".to_string()), true);
    }
}
