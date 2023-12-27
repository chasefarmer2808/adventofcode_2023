use crate::utils::read_lines;

/*
Trying a recursive solution.
Given list of integers, generate a new list of integers by taking running difference of each integer.
Recurse with new list.
If list is all zeros, return

fn get_next_value(list) -> int
    if list is all zeros
        return 0

    new_list = []
    for item in list
        new_list.push(next_item - item)

    return list[last] + get_next_value(new_list)
 */

pub fn part_one(input_file: &str) -> i32 {
    let readings = parse(input_file);

    readings.iter()
        .map(|reading| get_next_value(reading))
        .sum()
}

pub fn part_two(input_file: &str) -> i32 {
    let readings = parse(input_file);

    readings.iter()
        .map(|reading| get_prev_value(reading))
        .sum()
}

fn parse(input_file: &str) -> Vec<Vec<i32>> {
    let lines = read_lines(input_file);
    lines.iter()
        .map(|line| line.split_whitespace()
            .map(|num_str| num_str.parse().unwrap())
            .collect())
        .collect()
}

fn get_next_value(sequence: &Vec<i32>) -> i32 {
    if sequence.iter().all(|n| *n == 0) {
        return 0;
    }

    let mut new_list: Vec<i32> = Vec::new();
    for (i, n) in sequence[..sequence.len() - 1].iter().enumerate() {
        new_list.push(sequence[i+1] - *n);
    }

    return sequence.last().unwrap() + get_next_value(&new_list);
}

fn get_prev_value(sequence: &Vec<i32>) -> i32 {
    if sequence.iter().all(|n| *n == 0) {
        return 0;
    }

    let mut new_list: Vec<i32> = Vec::new();
    for (i, n) in sequence[..sequence.len() - 1].iter().enumerate() {
        new_list.push(sequence[i+1] - *n);
    }

    return sequence.first().unwrap() - get_prev_value(&new_list);
}