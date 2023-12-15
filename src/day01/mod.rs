/*
Given lines of text containing letters and integers, get the first and last integer in the line, use
them to create a two-digit integer, and sum them all together.

For each line, use regex to filter out alpha chars.  Then take first and last chars of resulting string.
If resulting string has only one char, use it twice.
 */

use std::fs::{read_to_string};
use regex::Regex;

pub fn soln(input_file: &str) -> u32 {
    let calibrations = read_lines(input_file);
    let mut values: Vec<u32> = Vec::new();

    for cal in calibrations.iter() {
        let nums = parse_digits(cal);
        let value: u32 = match nums.len() {
            0 => 0,
            1 => to_two_digit_int(nums[0], nums[0]),
            _ => to_two_digit_int(nums[0], nums.last().unwrap().to_owned())
        };
        values.push(value);
    }

    return values.into_iter().reduce(|a, b| a + b).unwrap();
}

pub fn part_two(input_file: &str) {
    let reg = Regex::new(r"(?=(one|two|three|four|five|six|seven|eight|nine|[1-9]))").unwrap();
    let calibrations = read_lines(input_file);
    let mut values: Vec<u32> = Vec::new();

    for cal in calibrations.iter() {
        let nums: Vec<u32> = reg.find_iter(cal)
            .map(|digit| digit_str_to_int(digit.as_str()))
            .collect();
        println!("{:?}", nums)
    }
}

fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename)
        .unwrap()
        .lines()
        .map(String::from)
        .collect()
}

fn parse_digits(line: &str) -> Vec<u32> {
    line.chars().filter_map(|item| item.to_digit(10)).collect()
}

fn digit_str_to_int(digit: &str) -> u32 {
    match digit {
        "one" | "1" => 1,
        "two" | "2" => 2,
        "three" | "3" => 3,
        "four" | "4" => 4,
        "five" | "5" => 5,
        "six" | "6" => 6,
        "seven" | "7" => 7,
        "eight" | "8" => 8,
        "nine" | "9" => 9,
        _ => 0
    }
}

fn to_two_digit_int(a: u32, b: u32) -> u32 {
    let mut digits: String = a.to_string();
    digits.push(b.to_string().chars().nth(0).unwrap());
    return digits.parse().unwrap();
}
