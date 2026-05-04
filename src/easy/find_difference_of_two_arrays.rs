//! # 2215. Find the Difference of Two Arrays
//! 
//! Given two **0-indexed** integer arrays `nums1` and `nums2`, return a list `answer` of size `2` where:
//! * `answer[0]` is a list of all **distinct** integers in `nums1` which are **not** present in `nums2`.
//! * `answer[1]` is a list of all **distinct** integers in `nums2` which are **not** present in `nums1`.
//! 
//! Note that the integers in the lists may be returned in **any** order.
//! 
//! ### Example 1:
//! **Input:** `nums1 = [1,2,3], nums2 = [2,4,6]`
//! **Output:** `[[1,3],[4,6]]`
//! **Explanation:**
//! For `nums1`, `nums1[1] = 2` is present at index 0 of `nums2`, whereas `nums1[0] = 1` and `nums1[2] = 3` are not present in `nums2`. Therefore, `answer[0] = [1,3]`.
//! For `nums2`, `nums2[0] = 2` is present at index 1 of `nums1`, whereas `nums2[1] = 4` and `nums2[2] = 6` are not present in `nums1`. Therefore, `answer[1] = [4,6]`.
//! 
//! ### Example 2:
//! **Input:** `nums1 = [1,2,3,3], nums2 = [1,1,2,2]`
//! **Output:** `[[3],[]]`
//! **Explanation:**
//! For `nums1`, `nums1[2]` and `nums1[3]` are not present in `nums2`. Since `nums1[2] == nums1[3]`, their value is only included once and `answer[0] = [3]`.
//! Every integer in `nums2` is present in `nums1`. Therefore, `answer[1] = []`.
//! 
//! ### Constraints:
//! * `1 <= nums1.length, nums2.length <= 1000`
//! * `-1000 <= nums1[i], nums2[i] <= 1000`
//! 
//! ---
//! 
//! ### Myanmar Explanation
//! ဒီပုစ္ဆာက ပေးထားတဲ့ array နှစ်ခု (`nums1` နဲ့ `nums2`) ကြားက မတူညီတဲ့ ကိန်းဂဏန်းတွေကို ရှာခိုင်းတာပါ။
//! 
//! **ရလဒ်မှာ list နှစ်ခု ပါရပါမယ်:**
//! ၁။ `answer[0]` မှာ `nums1` ထဲမှာပဲရှိပြီး `nums2` ထဲမှာ **မပါတဲ့** ကိန်းတွေကို ထည့်ရပါမယ်။
//! ၂။ `answer[1]` မှာ `nums2` ထဲမှာပဲရှိပြီး `nums1` ထဲမှာ **မပါတဲ့** ကိန်းတွေကို ထည့်ရပါမယ်။
//! 
//! **သတိပြုရန်:**
//! *   ကိန်းတစ်ခုတည်း ထပ်နေရင် (Distinct value) တစ်ခါပဲ ထည့်ရပါမယ်။
//! *   အစဉ်လိုက် (Order) ဖြစ်နေစရာ မလိုပါဘူး။

use std::collections::{HashMap, HashSet};

struct Solution;

impl Solution {
    pub fn find_difference(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<Vec<i32>> {
        // [1,2,3] [2,4,5]
        // hashmap -> 1,2-2,3,4,5
        // lv ,rv = [],[] < - hashset
        // loop lv,rv -> check with hashmap if  value is > than 1 remove else skip
        // [lv,rv]
        let mut hashmap: HashMap<i32,i32>  = HashMap::new();

        let mut ls: HashSet<i32> = HashSet::from_iter(nums1); 
        let mut rs: HashSet<i32> = HashSet::from_iter(nums2);

        for i in &ls {
            *hashmap.entry(*i).or_insert(0) +=1;
        }
        for i in &rs {
            *hashmap.entry(*i).or_insert(0) +=1;
        }

        ls.retain(|i|{
            match hashmap.get(i) {
                Some(&val) => val <= 1,
                None => true,
            }
        });
        rs.retain(|i|{
            match hashmap.get(i) {
                Some(&val) => val <= 1,
                None => true,
            }
        });
        vec![ls.into_iter().collect(),rs.into_iter().collect()]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let nums1 = vec![1, 2, 3];
        let nums2 = vec![2, 4, 6];
        let mut result = Solution::find_difference(nums1, nums2);
        
        // Sorting results for consistent assertion
        result[0].sort();
        result[1].sort();
        assert_eq!(result, vec![vec![1, 3], vec![4, 6]]);
    }

    #[test]
    fn test_example_2() {
        let nums1 = vec![1, 2, 3, 3];
        let nums2 = vec![1, 1, 2, 2];
        let mut result = Solution::find_difference(nums1, nums2);
        
        result[0].sort();
        result[1].sort();
        assert_eq!(result, vec![vec![3], vec![]]);
    }

    #[test]
    fn test_identical_arrays() {
        let nums1 = vec![1, 2, 3];
        let nums2 = vec![1, 2, 3];
        let result = Solution::find_difference(nums1, nums2);
        assert_eq!(result, vec![vec![], vec![]]);
    }

    #[test]
    fn test_completely_different_arrays() {
        let nums1 = vec![1, 2];
        let nums2 = vec![3, 4];
        let mut result = Solution::find_difference(nums1, nums2);
        
        result[0].sort();
        result[1].sort();
        assert_eq!(result, vec![vec![1, 2], vec![3, 4]]);
    }

    #[test]
    fn test_with_negatives() {
        let nums1 = vec![-1, 0, 1];
        let nums2 = vec![1, 2, 3];
        let mut result = Solution::find_difference(nums1, nums2);
        
        result[0].sort();
        result[1].sort();
        assert_eq!(result, vec![vec![-1, 0], vec![2, 3]]);
    }
}