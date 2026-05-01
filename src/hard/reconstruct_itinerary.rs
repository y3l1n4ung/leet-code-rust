/// [332] Reconstruct Itinerary
/// Difficulty: Hard
/// Topics: Depth-First Search, Graph, Eulerian Circuit
/// Tags: NeetCode150
///
/// You are given a list of airline tickets where tickets[i] = [fromi, toi] represent the departure and the arrival airports of one flight. Reconstruct the itinerary in order and return it.
/// All of the tickets belong to a man who departs from "JFK", thus, the itinerary must begin with "JFK". If there are multiple valid itineraries, you should return the itinerary that has the smallest lexical order when read as a single string.
/// For example, the itinerary ["JFK", "LGA"] has a smaller lexical order than ["JFK", "LGB"].
/// You may assume all tickets form at least one valid itinerary. You must use all the tickets once and only once.
///
/// Link: https://leetcode.com/problems/reconstruct-itinerary/

struct Solution;

impl Solution {
    pub fn find_itinerary(tickets: Vec<Vec<String>>) -> Vec<String> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let tickets = vec![
            vec!["MUC".to_string(), "LHR".to_string()],
            vec!["JFK".to_string(), "MUC".to_string()],
            vec!["SFO".to_string(), "SJC".to_string()],
            vec!["LHR".to_string(), "SFO".to_string()],
        ];
        assert_eq!(
            Solution::find_itinerary(tickets),
            vec![
                "JFK".to_string(),
                "MUC".to_string(),
                "LHR".to_string(),
                "SFO".to_string(),
                "SJC".to_string()
            ]
        );
    }

    #[test]
    fn test_2() {
        let tickets = vec![
            vec!["JFK".to_string(), "SFO".to_string()],
            vec!["JFK".to_string(), "ATL".to_string()],
            vec!["SFO".to_string(), "ATL".to_string()],
            vec!["ATL".to_string(), "JFK".to_string()],
            vec!["ATL".to_string(), "SFO".to_string()],
        ];
        assert_eq!(
            Solution::find_itinerary(tickets),
            vec![
                "JFK".to_string(),
                "ATL".to_string(),
                "JFK".to_string(),
                "SFO".to_string(),
                "ATL".to_string(),
                "SFO".to_string()
            ]
        );
    }
}
