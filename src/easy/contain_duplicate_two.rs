//! # 219. Contains Duplicate II
//!
//! Given an integer array `nums` and an integer `k`, return true if there are
//! two distinct indices `i` and `j` in the array such that:
//! 1. nums[i] == nums[j]
//! 2. abs(i - j) <= k
//!
//! Constraints:
//! - 1 <= nums.length <= 10^5
//! - -10^9 <= nums[i] <= 10^9
//! - 0 <= k <= 10^5

use std::collections::HashSet;

struct Solution;

impl Solution {
    pub fn contains_nearby_duplicate(nums: Vec<i32>, k: i32) -> bool {
        let k = k as usize;

        // ၁။ Window ထဲမှာရှိတဲ့ ဂဏန်းတွေကို သိမ်းထားဖို့ HashSet တစ်ခု တည်ဆောက်မယ်။
        let mut window = HashSet::with_capacity(k + 1);

        for i in 0..nums.len() {
            // ၂။ Window Maintenance (Cleanup):
            // အကယ်၍ လက်ရှိ index 'i' က 'k' ထက် ကြီးနေပြီဆိုရင်၊
            // Window ရဲ့ အနောက်မှာ ကျန်ခဲ့တဲ့ (အကွာအဝေး k ထက် ကျော်သွားတဲ့) ဂဏန်းကို ဖယ်ထုတ်ရမယ်။
            // ဥပမာ - k=3, i=4 ဆိုရင် index 0 က ဂဏန်းကို ဖယ်မယ် (4 - 3 - 1 = 0)။
            if i > k {
                window.remove(&nums[i - k - 1]);
            }

            // ၃။ Duplicate Check:
            // .insert() function က တန်ဖိုးအသစ်ဆိုရင် true ပြန်ပြီး၊
            // ရှိပြီးသား တန်ဖိုးဆိုရင်တော့ false ပြန်ပါတယ်။
            // false ပြန်လာပြီဆိုရင်တော့ k အကွာအဝေးအတွင်းမှာ duplicate တွေ့ပြီလို့ ဆိုလိုတာပါ။
            if !window.insert(nums[i]) {
                return true;
            }
        }

        // ၄။ Array တစ်ခုလုံး ပတ်လို့မှ duplicate မတွေ့ရင် false ပြန်မယ်။

        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        assert_eq!(
            Solution::contains_nearby_duplicate(vec![1, 2, 3, 1], 3),
            true
        );
    }

    #[test]
    fn test_example_2() {
        assert_eq!(
            Solution::contains_nearby_duplicate(vec![1, 0, 1, 1], 1),
            true
        );
    }

    #[test]
    fn test_example_3() {
        assert_eq!(
            Solution::contains_nearby_duplicate(vec![1, 2, 3, 1, 2, 3], 2),
            false
        );
    }

    #[test]
    fn test_k_zero() {
        assert_eq!(Solution::contains_nearby_duplicate(vec![1, 2, 1], 0), false);
    }

    #[test]
    fn test_empty_or_single() {
        assert_eq!(Solution::contains_nearby_duplicate(vec![], 0), false);
        assert_eq!(Solution::contains_nearby_duplicate(vec![1], 1), false);
    }
}
