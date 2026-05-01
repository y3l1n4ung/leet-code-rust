// Climbing Stairs Practice 🦀

pub struct Solution;

impl Solution {
    /// Practice: Climbing Stairs
    /// Calculates the number of ways to reach the n-th step.
    pub fn climb_stairs(n: i32) -> i32 {
        if n <= 2 {
            return n;
        }

        let mut first = 1;
        let mut second = 2;

        for _ in 3..=n {
            let third = first + second;
            first = second;
            second = third;
        }

        second
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_climb_stairs() {
        assert_eq!(Solution::climb_stairs(2), 2);
        assert_eq!(Solution::climb_stairs(3), 3);
        assert_eq!(Solution::climb_stairs(5), 8);
    }
}
