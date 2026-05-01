/// [286] Walls and Gates
/// Difficulty: Medium
/// Topics: Array, Breadth-First Search, Matrix
/// Tags: NeetCode150
///
/// You are given an m x n grid rooms initialized with these three possible values:
/// - -1: A wall or an obstacle.
/// - 0: A gate.
/// - INF: Infinity means an empty room.
/// Fill each empty room with the distance to its nearest gate. If it is impossible to reach a gate, it should be filled with INF.
///
/// Link: https://leetcode.com/problems/walls-and-gates/ (Premium)
/// Free Link: https://www.lintcode.com/problem/663/

struct Solution;

impl Solution {
    pub fn walls_and_gates(rooms: &mut Vec<Vec<i32>>) {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        const INF: i32 = 2147483647;
        let mut rooms = vec![
            vec![INF, -1, 0, INF],
            vec![INF, INF, INF, -1],
            vec![INF, -1, INF, -1],
            vec![0, -1, INF, INF],
        ];
        let expected = vec![
            vec![3, -1, 0, 1],
            vec![2, 2, 1, -1],
            vec![1, -1, 2, -1],
            vec![0, -1, 3, 4],
        ];
        Solution::walls_and_gates(&mut rooms);
        assert_eq!(rooms, expected);
    }
}
