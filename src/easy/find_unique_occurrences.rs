//! # 1207. Unique Number of Occurrences
//!
//! Given an array of integers `arr`, return `true` if the number of occurrences
//! of each value in the array is **unique** or `false` otherwise.
//!
//! ### Example 1:
//! **Input:** `arr = [1,2,2,1,1,3]`
//! **Output:** `true`
//! **Explanation:** The value 1 has 3 occurrences, 2 has 2 and 3 has 1.
//! No two values have the same number of occurrences.
//!
//! ### Example 2:
//! **Input:** `arr = [1,2]`
//! **Output:** `false`
//!
//! ### Example 3:
//! **Input:** `arr = [-3,0,1,-3,1,1,1,-3,10,0]`
//! **Output:** `true`
//!
//! ### Constraints:
//! * `1 <= arr.length <= 1000`
//! * `-1000 <= arr[i] <= 1000`
//!
//! ---
//!
//! ### Myanmar Explanation
//! ဒီပုစ္ဆာက ပေးထားတဲ့ array ထဲမှာ ပါတဲ့ ကိန်းဂဏန်းတစ်ခုချင်းစီရဲ့ အကြိမ်ရေ (Occurrences) တွေဟာ တစ်ခုနဲ့တစ်ခု မတူဘဲ သီးသန့်စီ (Unique) ဖြစ်နေသလားဆိုတာကို စစ်ခိုင်းတာပါ။
//!
//! **တွက်ချက်ပုံအဆင့်ဆင့်:**
//! ၁။ Array ထဲမှာပါတဲ့ ကိန်းတစ်ခုချင်းစီက ဘယ်နှစ်ကြိမ်စီပါလဲဆိုတာကို အရင်ရေတွက်ပါ။ (ဥပမာ - `1` က ၃ ကြိမ်၊ `2` က ၂ ကြိမ် စသဖြင့်)။
//! ၂။ အဲ့ဒီ ရလာတဲ့ အကြိမ်ရေ (Count values) တွေထဲမှာ ထပ်နေတာ ရှိမရှိ ပြန်စစ်ပါ။
//! ၃။ အကြိမ်ရေတွေအားလုံး မတူညီကြဘူးဆိုရင် `true` လို့ပြန်ပေးပြီး၊ တူညီတဲ့အကြိမ်ရေရှိနေရင် `false` လို့ပြန်ပေးရပါမယ်။

use std::collections::{HashMap, HashSet};

struct Solution;

impl Solution {
    pub fn unique_occurrences(arr: Vec<i32>) -> bool {
        let mut hashmap: HashMap<i32, i32> = HashMap::new();

        for n in arr {
            let _ = *hashmap.entry(n).or_insert(0) += 1;
        }
        let occur: Vec<i32> = hashmap.into_values().collect();

        // occur.len() == occur.iter().collect::<HashSet<_>>().len()
        let mut seen = HashSet::new();
        occur.iter().all(|x| seen.insert(x))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        assert_eq!(Solution::unique_occurrences(vec![1, 2, 2, 1, 1, 3]), true);
    }

    #[test]
    fn test_example_2() {
        assert_eq!(Solution::unique_occurrences(vec![1, 2]), false);
    }

    #[test]
    fn test_example_3() {
        assert_eq!(
            Solution::unique_occurrences(vec![-3, 0, 1, -3, 1, 1, 1, -3, 10, 0]),
            true
        );
    }

    #[test]
    fn test_all_same_elements() {
        // Only one value exists, its occurrence count (5) is unique.
        assert_eq!(Solution::unique_occurrences(vec![5, 5, 5, 5, 5]), true);
    }

    #[test]
    fn test_two_different_elements_same_frequency() {
        // '10' appears twice, '20' appears twice. Frequencies (2, 2) are not unique.
        assert_eq!(Solution::unique_occurrences(vec![10, 10, 20, 20]), false);
    }

    #[test]
    fn test_empty_constraints_minimum() {
        assert_eq!(Solution::unique_occurrences(vec![1]), true);
    }

    #[test]
    fn test_large_range() {
        // 1 occurs 1 time, 2 occurs 2 times, ... 4 occurs 4 times.
        let mut arr = Vec::new();
        for i in 1..=4 {
            for _ in 0..i {
                arr.push(i);
            }
        }
        assert_eq!(Solution::unique_occurrences(arr), true);
    }
}
