/// [LeetCode 1396] Design Underground System
/// Difficulty: Medium
/// Topics: Hash Table, String, Design
///
/// Link: https://leetcode.com/problems/design-underground-system/

use std::collections::HashMap;

pub struct UndergroundSystem {
    check_ins: HashMap<i32, (String, i32)>, // id -> (station, time)
    travel_times: HashMap<(String, String), (i32, i32)>, // (start, end) -> (total_time, count)
}

impl UndergroundSystem {
    pub fn new() -> Self {
        Self {
            check_ins: HashMap::new(),
            travel_times: HashMap::new(),
        }
    }

    pub fn check_in(&mut self, id: i32, station_name: String, t: i32) {
        todo!("Record the check-in event")
    }

    pub fn check_out(&mut self, id: i32, station_name: String, t: i32) {
        todo!("Process the check-out event and update travel time statistics")
    }

    pub fn get_average_time(&self, start_station: String, end_station: String) -> f64 {
        todo!("Calculate the average travel time between two stations")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_underground_system_basic() {
        let mut system = UndergroundSystem::new();
        system.check_in(45, "Leyton".to_string(), 3);
        system.check_in(32, "Paradise".to_string(), 8);
        system.check_in(27, "Leyton".to_string(), 10);
        
        system.check_out(45, "Waterloo".to_string(), 15);
        system.check_out(27, "Waterloo".to_string(), 20);
        system.check_out(32, "Cambridge".to_string(), 22);
        
        assert_eq!(system.get_average_time("Paradise".to_string(), "Cambridge".to_string()), 14.0);
        assert_eq!(system.get_average_time("Leyton".to_string(), "Waterloo".to_string()), 11.0);
    }
}
