//! # Evaluate Reverse Polish Notation
//! 
//! You are given an array of strings `tokens` that represents an arithmetic expression in a Reverse Polish Notation.
//! Evaluate the expression. Return an integer that represents the value of the expression.
//!
//! Example 1:
//! Input: tokens = ["2","1","+","3","*"]
//! Output: 9
//! Explanation: ((2 + 1) * 3) = 9

struct Solution;

impl Solution {
    pub fn eval_rpn( tokens: Vec<String>) -> i32 {
        // နံပါတ်တွေ့ရင် Stack ထဲ ထည့်မယ်။ 
        // Operator (+, -, *, /) တွေ့ရင် Stack ထဲက နောက်ဆုံး ၂ လုံးကို ထုတ်ပြီး တွက်မယ်။
        let mut stack = Vec::new();
        for token in tokens{
            match token.as_str() {
                "+"=> {
                    let a = stack.pop().unwrap();
                    let b = stack.pop().unwrap();
                    stack.push(b + a);
                },
                "-" => {
                    let a = stack.pop().unwrap();
                    let b = stack.pop().unwrap();
                    stack.push(b - a );
                },
                "*" => {
                    let a = stack.pop().unwrap();
                    let b = stack.pop().unwrap();
                    stack.push(b * a);

                },
                 "/" => {
                    let a = stack.pop().unwrap();
                    let b = stack.pop().unwrap();
                    stack.push(b / a);

                },
                number_str => {
                    let n : i32= number_str.parse().unwrap();
                    stack.push(n);
                },
                
            }
            
        }
        stack.pop().unwrap_or(0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let tokens = vec![
            "2".to_string(), 
            "1".to_string(), 
            "+".to_string(), 
            "3".to_string(), 
            "*".to_string()
        ];
        assert_eq!(Solution::eval_rpn(tokens), 9);
    }

    #[test]
    fn test_example_2() {
        let tokens = vec![
            "4".to_string(), 
            "13".to_string(), 
            "5".to_string(), 
            "/".to_string(), 
            "+".to_string()
        ];
        assert_eq!(Solution::eval_rpn(tokens), 6);
    }
}