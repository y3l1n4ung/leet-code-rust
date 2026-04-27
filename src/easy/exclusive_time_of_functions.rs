//! # Exclusive Time of Functions
//!
//! On a single-threaded CPU, we execute a program containing `n` functions. 
//! Each function has a unique ID from `0` to `n-1`.
//!
//! A function's **exclusive time** is the sum of execution times for all function calls 
//! in the program, where the function maintains control of the CPU.
//!
//! Example 1:
//! Input: n = 2, logs = ["0:start:0","1:start:2","1:end:5","0:end:6"]
//! Output: [3, 4]
//! Explanation:
//! Function 0 starts at time 0, then function 1 starts at time 2 and ends at time 5.
//! Function 0 resumes at time 6 and ends.
//! So function 0 spent 2 units (0-1) + 1 unit (6) = 3 units.
//! Function 1 spent 4 units (2-5).

struct Solution;

impl Solution {
    pub fn exclusive_time(n: i32, logs: Vec<String>) -> Vec<i32> {
        // TODO: Implement the logic here
        let mut result= vec![n;0];
        let mut stack : Vec<usize> = Vec::new();
        let mut prev_time = 0;

        for log in logs{
            let parts: Vec<&str> = log.split(":").collect();
            let id = parts[0].parse::<usize>().unwrap();
            let status = parts[1];
            let time = parts[2].parse::<i32>().unwrap();

            if status == "start" {
                if let Some(&partent_id) = stack.last() {
                    result[partent_id] += time - prev_time;
                }
                stack.push(id);
                prev_time = time;
            }else {
                if let Some(curr_id) = stack.pop(){
                    result[curr_id] += time - prev_time +1;
                } 
                prev_time = time+1
            }
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let n = 2;
        let logs = vec![
            "0:start:0".to_string(),
            "1:start:2".to_string(),
            "1:end:5".to_string(),
            "0:end:6".to_string(),
        ];
        assert_eq!(Solution::exclusive_time(n, logs), vec![3, 4]);
    }

    #[test]
    fn test_nested_multiple() {
        // Function တစ်ခုထဲမှာ အခြား function တွေ အဆင့်ဆင့်ပွင့်လာတဲ့ case
        let n = 1;
        let logs = vec![
            "0:start:0".to_string(),
            "0:start:2".to_string(),
            "0:end:5".to_string(),
            "0:end:6".to_string(),
        ];
        // Total time is 7 (0 to 6). Function 0 occupies everything.
        assert_eq!(Solution::exclusive_time(n, logs), vec![7]);
    }

    #[test]
    fn test_complex_interleaving() {
        // ပိုရှုပ်ထွေးတဲ့ sequence
        let n = 3;
        let logs = vec![
            "0:start:0".to_string(),
            "1:start:2".to_string(),
            "2:start:3".to_string(),
            "2:end:4".to_string(),
            "1:end:5".to_string(),
            "0:end:6".to_string(),
        ];
        // Func 2: 3 to 4 = 2 units
        // Func 1: 2 to 5 (but minus func 2) = 4 - 2 = 2 units
        // Func 0: 0 to 6 (but minus func 1 & 2) = 7 - 4 = 3 units
        assert_eq!(Solution::exclusive_time(n, logs), vec![3, 2, 2]);
    }

    #[test]
    fn test_non_overlapping() {
        // တစ်ခုပြီးမှ တစ်ခုလာတဲ့ (Sequential) case
        let n = 2;
        let logs = vec![
            "0:start:0".to_string(),
            "0:end:2".to_string(),
            "1:start:3".to_string(),
            "1:end:5".to_string(),
        ];
        assert_eq!(Solution::exclusive_time(n, logs), vec![3, 3]);
    }
}