//! # 119. Pascal's Triangle II
//! 
//! Given an integer `row_index`, return the `row_index-th` (0-indexed) row of the Pascal's triangle.
//! 
//! ### Constraints:
//! - 0 <= rowIndex <= 33
//! - Follow-up: Could you optimize your algorithm to use only O(k) extra space?

struct Solution;

impl Solution {
    pub fn get_row(row_index: i32) -> Vec<i32> {
        // Space complexity O(k) ဖြစ်ဖို့အတွက် vector တစ်ခုတည်းကိုပဲ ပြန်သုံးပါမယ်
        let n = row_index as usize;
        let mut row = vec![1; n + 1];

        for i in 1..n  {
            for j in (1..=i).rev(){ 
                
                // i=1 တုန်းက: [1, x, x, x]
                // i=2 တုန်းက: [1, 2, 1, x]
                // i=3 တုန်းက: [1, 3, 3, 1] ဖြစ်သွားမယ်
                // rev မလုပ်ရင် // i=2 တုန်းက: [1, 2, 1, x]
                // ကို i=3 တွင် update လုပ်ပါက [2] တန်ဖိုး [3] ပြောင်းပီး နောက်တစ်ကြိမ်ပေါင်းပါက [4] ဖြစ်သွားပီး [1,3,4,1] ဖြစ်သွားမည်

                row[j] = row[j] + row[j-1];
            }

        
        }
        row
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_row_3() {
        // Input: 3 -> Output: [1, 3, 3, 1]
        assert_eq!(Solution::get_row(3), vec![1, 3, 3, 1]);
    }

    #[test]
    fn test_row_0() {
        // Input: 0 -> Output: [1]
        assert_eq!(Solution::get_row(0), vec![1]);
    }

    #[test]
    fn test_row_1() {
        // Input: 1 -> Output: [1, 1]
        assert_eq!(Solution::get_row(1), vec![1, 1]);
    }
}