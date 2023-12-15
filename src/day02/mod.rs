/*
Bag contains 12 red cubes, 13 green cubes, and 14 blue cubes.  Need to decide which games would
have been possible given this configuration, and then sum up the game ids.

Brute force: n*m time
for each game
    for each set
        if set impossible
            go to next game
    after each set, all game is possible so add game id to running sum.

 Can we look at entire line at once?
 for each game
    get max red match
    get max blue match
    get max green match

    if max red > red config or max blue > blue config or max green > green config
        continue

    Otherwise, game is possible so add game id to running sum.
 */

use regex::Regex;
use crate::utils::read_lines;

pub fn part_one(input_file: &str, red_config: u32, green_config: u32, blue_config: u32) -> u32 {
    let games = read_lines(input_file);
    let mut sum: u32 = 0;
    let red_reg = Regex::new(r"[0-9]* red").unwrap();
    let blue_reg = Regex::new(r"[0-9]* blue").unwrap();
    let green_reg = Regex::new(r"[0-9]* green").unwrap();

    for (i, game) in games.iter().enumerate() {
        let max_red = red_reg.find_iter(game)
            .map(|item| item.as_str().split(' ').nth(0).unwrap().parse::<u32>().unwrap())
            .max().unwrap();

        if (max_red > red_config) {
            continue;
        }

        let max_green = green_reg.find_iter(game)
            .map(|item| item.as_str().split(' ').nth(0).unwrap().parse::<u32>().unwrap())
            .max().unwrap();

        if (max_green > green_config) {
            continue;
        }

        let max_blue = blue_reg.find_iter(game)
            .map(|item| item.as_str().split(' ').nth(0).unwrap().parse::<u32>().unwrap())
            .max().unwrap();

        if (max_blue > blue_config) {
            continue;
        }

        sum += i as u32 + 1;
    }

    return sum;
}

pub fn part_two(input_file: &str) -> u32 {
    let games = read_lines(input_file);
    let mut sum: u32 = 0;
    let red_reg = Regex::new(r"[0-9]* red").unwrap();
    let blue_reg = Regex::new(r"[0-9]* blue").unwrap();
    let green_reg = Regex::new(r"[0-9]* green").unwrap();

    for (i, game) in games.iter().enumerate() {
        let max_red = red_reg.find_iter(game)
            .map(|item| item.as_str().split(' ').nth(0).unwrap().parse::<u32>().unwrap())
            .max().unwrap();

        let max_green = green_reg.find_iter(game)
            .map(|item| item.as_str().split(' ').nth(0).unwrap().parse::<u32>().unwrap())
            .max().unwrap();

        let max_blue = blue_reg.find_iter(game)
            .map(|item| item.as_str().split(' ').nth(0).unwrap().parse::<u32>().unwrap())
            .max().unwrap();

        sum += max_red * max_green * max_blue;
    }

    return sum;
}