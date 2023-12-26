use std::collections::HashMap;
use crate::utils::read_lines;
use regex::Regex;

pub fn part_one(input_file: &str) -> u64 {
    let lines = read_lines(input_file);
    let moves = &lines[0];
    let node_map = parse_nodes(&lines[2..]);
    let mut steps = 0;

    let mut cur_move_index = 0;
    let mut cur_node = "AAA";
    let mut cur_branches = node_map.get(cur_node).unwrap();
    while cur_node != "ZZZ" {
        let cur_move: char = moves.chars().nth(cur_move_index).unwrap();

        match cur_move {
            'L' => cur_node = &*cur_branches.0,
            'R' => cur_node = &*cur_branches.1,
            _ => panic!("Code bug")
        }

        cur_move_index += 1;
        steps += 1;
        cur_branches = node_map.get(cur_node).unwrap();

        if cur_move_index >= moves.len() {
            cur_move_index = 0
        }

        if cur_node == "ZZZ" {
            break;
        }
    }

    return steps;
}

pub fn part_two(input_file: &str) -> u64 {
    let lines = read_lines(input_file);
    let moves = &lines[0];
    let node_map = parse_nodes(&lines[2..]);
    let mut step_counts: Vec<u64> = Vec::new();
    let mut steps = 0;

    let mut cur_nodes: Vec<&str> = node_map.keys()
        .filter(|key| key.chars().last().unwrap() == 'A')
        .map(|s| s.as_str())
        .collect();

    for cur_node in &mut cur_nodes {
        let mut cur_steps = 0;
        let mut cur_move_index = 0;
        let mut cur_branches = node_map.get(*cur_node).unwrap();

        while !cur_node.ends_with("Z") {
            let cur_move: char = moves.chars().nth(cur_move_index).unwrap();

            *cur_node = match cur_move {
                'L' => &*cur_branches.0,
                'R' => &*cur_branches.1,
                _ => panic!("Code bug")
            };

            cur_move_index += 1;
            cur_steps += 1;
            cur_branches = node_map.get(*cur_node).unwrap();

            if cur_move_index >= moves.len() {
                cur_move_index = 0
            }
        }

        step_counts.push(cur_steps);
    }

    return lcm_of_list(&step_counts);
}

fn parse_nodes(nodes: &[String]) -> HashMap<String, (String, String)> {
    let parse_regex = Regex::new(r"[A-Z]{3}").unwrap();
    let mut node_map: HashMap<String, (String, String)> = HashMap::new();

    for node in nodes {
        let matches: Vec<&str> = parse_regex.find_iter(node).map(|m| m.as_str()).collect::<Vec<&str>>();
        let node_label = matches[0];
        let left = matches[1];
        let right = matches[2];

        node_map.insert(String::from(node_label), (String::from(left), String::from(right)));
    }

    return node_map.clone();
}

fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn lcm(a: u64, b: u64) -> u64 {
    (a * b) / gcd(a, b)
}

fn lcm_of_list(numbers: &Vec<u64>) -> u64 {
    numbers.iter().cloned().fold(1, |acc, x| lcm(acc, x))
}