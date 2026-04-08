# Thinking: Underground System

## Problem Understanding
Calculate the average travel time between two stations based on check-in and check-out events.
- `check_in(id, station, time)`: Record a passenger starting their journey.
- `check_out(id, station, time)`: Record a passenger ending their journey and calculate the duration.
- `get_average_time(start, end)`: Return the average of all durations for this specific route.

## Data Structure Design
1. **Ongoing Journeys**: Use a `HashMap<id, (station, start_time)>` to store passengers currently in the system.
2. **Completed Route Statistics**: Use a `HashMap<(start_station, end_station), (total_time, count)>` to store aggregated travel data.

## Scalability Consideration
- **Memory**: Journeys are removed upon check-out, so memory scales with concurrent passengers.
- **Aggregation**: Storing `total_time` and `count` allows O(1) calculation for the average instead of storing every individual duration.
