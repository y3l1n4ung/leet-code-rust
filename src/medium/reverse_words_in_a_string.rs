//! # 151. Reverse Words in a String
//!
//! **Difficulty:** Medium
//!
//! ## Problem Description
//! Given an input string `s`, reverse the order of the **words**.
//! A **word** is defined as a sequence of non-space characters. The words in `s`
//! will be separated by at least one space.
//!
//! Return a string of the words in reverse order concatenated by a single space.
//!
//! **Note** that `s` may contain leading or trailing spaces or multiple spaces
//! between two words. The returned string should only have a single space
//! separating the words. Do not include any extra spaces.
//!
//! ## Examples
//! ### Example 1:
//! - **Input:** `s = "the sky is blue"`
//! - **Output:** `"blue is sky the"`
//!
//! ### Example 2:
//! - **Input:** `s = "  hello world  "`
//! - **Output:** `"world hello"`
//! - **Explanation:** Your reversed string should not contain leading or trailing spaces.
//!
//! ### Example 3:
//! - **Input:** `s = "a good   example"`
//! - **Output:** `"example good a"`
//! - **Explanation:** You need to reduce multiple spaces between two words to a single space in the reversed string.
//!
//! ## Constraints:
//! - `1 <= s.length <= 10^4`
//! - `s` contains English letters (upper-case and lower-case), digits, and spaces ' '.
//! - There is **at least one** word in `s`.

struct Solution;

impl Solution {
    pub fn reverse_words(s: String) -> String {
        let mut words: Vec<&str> = s.split(" ").collect();
        let mut count = words.len() - 1;
        
        loop {
            if words[count].is_empty(){
                words.remove(count);
            }
            if count==0{
             break;   
            }
            count-=1;
            
         
            
        }

        let (mut left, mut right) = (0, words.len() - 1);

        while right > left {
          

         let tmp = words[left];
                words[left] = words[right];
                words[right] = tmp;
                left += 1;
                right -= 1;
        }

        words.join(" ")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        assert_eq!(
            Solution::reverse_words("the sky is blue".to_string()),
            "blue is sky the".to_string()
        );
    }

    #[test]
    fn test_example_2() {
        assert_eq!(
            Solution::reverse_words("  hello world  ".to_string()),
            "world hello".to_string()
        );
    }

    #[test]
    fn test_example_3() {
        assert_eq!(
            Solution::reverse_words("a good   example".to_string()),
            "example good a".to_string()
        );
    }
    #[test]
    fn test_example_4(){
        assert_eq!(
            Solution::reverse_words("a      Tt98F8cl       2ZqzzN      9ScQuGTD       4zF18c   U5wrw62z    KV1 q     WK1Asi       S4hiKPi2Y  l      XZCvURMnY  I3   llmjK3AVT F    eaiDa     N6Rc 8QOxU P0ieWh ta".to_string()),
            "ta P0ieWh 8QOxU N6Rc eaiDa F llmjK3AVT I3 XZCvURMnY l S4hiKPi2Y WK1Asi q KV1 U5wrw62z 4zF18c 9ScQuGTD 2ZqzzN Tt98F8cl a".to_string()
        );
    }
    #[test]
    fn test_single_word() {
        assert_eq!(
            Solution::reverse_words("  Bob  ".to_string()),
            "Bob".to_string()
        );
    }

    #[test]
    fn test_max_constraints_spaces() {
        assert_eq!(
            Solution::reverse_words("EPIC      SPACES".to_string()),
            "SPACES EPIC".to_string()
        );
    }
}
