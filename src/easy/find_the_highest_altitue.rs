//! # 1732. Find the Highest Altitude
//!
//! There is a biker going on a road trip. The road trip consists of `n + 1` points
//! at different altitudes. The biker starts his trip on point `0` with altitude equal `0`.
//!
//! You are given an integer array `gain` of length `n` where `gain[i]` is the
//! **net gain in altitude** between points `i` and `i + 1` for all `(0 <= i < n)`.
//! Return the **highest altitude** of a point.
//!
//! ### Myanmar Explanation
//! ခရီးသွား biker တစ်ယောက်က altitude 0 ကနေ ခရီးစထွက်ပါတယ်။
//! `gain` array ထဲက ကိန်းတွေက point တစ်ခုနဲ့တစ်ခုကြား အတက်အကျ (altitude difference) တွေဖြစ်ပါတယ်။
//! ခရီးစဉ်တစ်လျှောက် ရောက်ရှိခဲ့သမျှ အမှတ်တွေထဲမှာ "အမြင့်ဆုံး altitude" ကို ရှာပေးရမှာပါ။
//! ဥပမာ - [0, -5, -4, 1, 1, -6] ဆိုရင် အဖြေက 1 ဖြစ်ပါတယ်။
//!
//! ### Constraints:
//! * `n == gain.length`
//! * `1 <= n <= 100`
//! * `-100 <= gain[i] <= 100`

struct Solution;

impl Solution {
    // pub fn largest_altitude(gain: Vec<i32>) -> i32 {
    //     let mut current_altitude =0;
    //     let mut largest_altitude =0;

    //     for g in gain{
    //         current_altitude += g;
    //         if current_altitude > largest_altitude{
    //             largest_altitude = current_altitude;
    //         }
    //     }

    //     largest_altitude
    // }
    // pub fn largest_altitude(gain: Vec<i32>) -> i32 {
    //     gain.iter()
    //         .scan(0, |ca, g| {
    //             *ca += g;
    //             Some(*ca)
    //         })
    //         .max()
    //         .unwrap_or(0)
    //         .max(0)
    // }
    pub fn largest_altitude(gain: Vec<i32>) -> i32 {
        #[inline]
        fn f((cur,max):(i32,i32), gain: &i32)-> (i32,i32){
            (cur+gain, max.max(cur+gain))
        }
        gain.iter().fold((0, 0), f).1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let gain = vec![-5, 1, 5, 0, -7];
        let result = Solution::largest_altitude(gain);
        assert_eq!(result, 1);
    }

    #[test]
    fn test_example_2() {
        let gain = vec![-4, -3, -2, -1, 4, 3, 2];
        let result = Solution::largest_altitude(gain);
        assert_eq!(result, 0);
    }

    #[test]
    fn test_all_positive() {
        let gain = vec![1, 2, 3, 4, 5];
        let result = Solution::largest_altitude(gain);
        assert_eq!(result, 15);
    }

    #[test]
    fn test_all_negative() {
        let gain = vec![-1, -2, -3, -4];
        let result = Solution::largest_altitude(gain);
        assert_eq!(result, 0); // Starts at 0, goes down, so 0 is highest
    }

    #[test]
    fn test_single_element_up() {
        let gain = vec![10];
        let result = Solution::largest_altitude(gain);
        assert_eq!(result, 10);
    }

    #[test]
    fn test_single_element_down() {
        let gain = vec![-10];
        let result = Solution::largest_altitude(gain);
        assert_eq!(result, 0);
    }
    #[test]
    fn test_case_1() {
        let gain = vec![-5, 1, 5, 0, -7];
        let result = Solution::largest_altitude(gain);
        assert_eq!(result, 1);
    }
    #[test]
    fn test_fluctuating_large_values() {
        let gain = vec![100, -100, 100, -100];
        let result = Solution::largest_altitude(gain);
        assert_eq!(result, 100);
    }
}
