/*
Need to get the margin of error for all races by finding the number of winning scenarios per race and
multiplying them all together.

get winning scenario count for a race: go through all combinations of time holding button up to the given
record time, starting at holding button for 1ms.

for hold_time in 1 to record_time
    distance = hold_time * record_time - hold_time

    if distance > record_distance
        increment win count

 */

use crate::utils::read_lines;

pub fn part_one(input_file: &str) -> u64 {
    let mut margin_of_error = 1;
    let race_data = read_lines(input_file);
    let times: Vec<u64> = race_data[0]
        .split(':')
        .last().unwrap()
        .split_whitespace()
        .map(|s| s.parse::<u64>().unwrap())
        .collect();
    let distances: Vec<u64> = race_data[1]
        .split(':')
        .last().unwrap()
        .split_whitespace()
        .map(|s| s.parse::<u64>().unwrap())
        .collect();
    let mut races: Vec<Race> = Vec::new();

    for (i, time) in times.iter().enumerate() {
        races.push(Race { record_time: *time, record_distance: distances[i] })
    }

    for race in races {
        margin_of_error *= race.count_winning_scenarios();
    }

    return margin_of_error;
}

pub fn part_two(input_file: &str) -> u64 {
    let mut margin_of_error = 1;
    let race_data = read_lines(input_file);
    let time = race_data[0]
        .split(':')
        .last().unwrap()
        .chars()
        .filter(|&c| !c.is_whitespace())
        .collect::<String>()
        .parse::<u64>().unwrap();
    let distance = race_data[1]
        .split(':')
        .last().unwrap()
        .chars()
        .filter(|&c| !c.is_whitespace())
        .collect::<String>()
        .parse::<u64>().unwrap();
    
    let race = Race {record_time: time, record_distance: distance };

    return race.count_winning_scenarios();
}

struct Race {
    record_time: u64,
    record_distance: u64
}

impl Race {
    fn count_winning_scenarios(&self) -> u64 {
        let mut count = 0;

        for hold_time in 1..self.record_time {
            let distance = hold_time * (self.record_time - hold_time);

            if distance > self.record_distance {
                count += 1;
            }
        }

        return count;
    }
}