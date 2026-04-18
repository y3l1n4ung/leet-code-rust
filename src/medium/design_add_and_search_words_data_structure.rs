/// [211] Design Add and Search Words Data Structure
/// Difficulty: Medium
/// Topics: String, Depth-First Search, Design, Trie
/// Tags: NeetCode150
///
/// Design a data structure that supports adding new words and finding if a string matches any previously added string.
/// Implement the WordDictionary class:
/// - WordDictionary() Initializes the object.
/// - void addWord(word) Adds word to the data structure, it can be matched later.
/// - bool search(word) Returns true if there is any string in the data structure that matches word or false otherwise. word may contain dots '.' where dots can be matched with any letter.
///
/// Link: https://leetcode.com/problems/design-add-and-search-words-data-structure/

struct WordDictionary {
    todo: (),
}

impl WordDictionary {
    pub fn new() -> Self {
        todo!()
    }

    pub fn add_word(&self, word: String) {
        todo!()
    }

    pub fn search(&self, word: String) -> bool {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let dict = WordDictionary::new();
        dict.add_word("bad".to_string());
        dict.add_word("dad".to_string());
        dict.add_word("mad".to_string());
        assert_eq!(dict.search("pad".to_string()), false);
        assert_eq!(dict.search("bad".to_string()), true);
        assert_eq!(dict.search(".ad".to_string()), true);
        assert_eq!(dict.search("b..".to_string()), true);
    }
}
