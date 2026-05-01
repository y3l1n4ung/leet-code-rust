/// [1206] Design Skiplist
/// Difficulty: Hard
/// Topics: Linked List, Design
/// Tags: RustMastery
///
/// Design a Skiplist without using any built-in libraries.
///
/// Link: https://leetcode.com/problems/design-skiplist/

pub struct Skiplist {
    // Add your internal state here
}

impl Skiplist {
    pub fn new() -> Self {
        todo!("Initialize the skiplist")
    }

    pub fn search(&self, target: i32) -> bool {
        todo!("Search for a target in the skiplist")
    }

    pub fn add(&mut self, num: i32) {
        todo!("Add a number to the skiplist")
    }

    pub fn erase(&mut self, num: i32) -> bool {
        todo!("Erase a number from the skiplist")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_skiplist_basic() {
        let mut skiplist = Skiplist::new();
        skiplist.add(1);
        skiplist.add(2);
        skiplist.add(3);
        assert_eq!(skiplist.search(0), false);

        skiplist.add(4);
        assert_eq!(skiplist.search(1), true);

        assert_eq!(skiplist.erase(0), false);
        assert_eq!(skiplist.erase(1), true);
        assert_eq!(skiplist.search(1), false);
    }
}
