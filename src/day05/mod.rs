use std::collections::HashMap;
use std::hash::Hash;
use crate::utils::read_lines;

pub fn part_one(input_file: &str) -> u32 {
    let almanac_lines = read_lines(input_file);
    let mut seeds: Vec<u32> = Vec::new();

    // Parse Almanac
    for (i, line) in almanac_lines.iter().enumerate() {
        if line.contains("seeds:") {
            seeds = parse_seeds(&line);
            continue;
        }

        if line.contains("seed-to-soil map:") {
            let seed_to_soil_nums_strings: Vec<&String> = lines_to_num_strings(&almanac_lines[i+1..]);
                // .iter()
                // .take_while(|line| !line.is_empty())
                // .collect();

            let seed_to_soil_nums: Vec<Vec<u32>> = num_strings_to_num_vecs(seed_to_soil_nums_strings);
                // .iter()
                // .map(|s| s.split_whitespace()
                //     .map(|ns| ns.parse::<u32>().unwrap())
                //     .collect::<Vec<u32>>())
                // .collect::<Vec<Vec<u32>>>();


            println!("{:?}", seed_to_soil_nums)
        }
    }

    let almanac = Almanac {
        seeds,
        seed_to_soil: Default::default(),
        soil_to_fert: Default::default(),
        fert_to_water: Default::default(),
        water_to_light: Default::default(),
        light_to_temp: Default::default(),
        temp_to_humid: Default::default(),
        humid_to_loc: Default::default()
    };

    // TODO: Gather locations.

    return 0;
}

fn parse_seeds(seed_line: &str) -> Vec<u32> {
    seed_line
        .split_whitespace()
        .filter(|s| s.parse::<u32>().is_ok())
        .map(|s| s.parse::<u32>().unwrap())
        .collect::<Vec<u32>>()
}

fn lines_to_num_strings(num_lines: &[String]) -> Vec<&String> {
    num_lines
        .iter()
        .take_while(|line| !line.is_empty())
        .collect()
}

fn num_strings_to_num_vecs(num_strings: Vec<&String>) -> Vec<Vec<u32>> {
    num_strings
        .iter()
        .map(|s| s.split_whitespace()
            .map(|ns| ns.parse::<u32>().unwrap())
            .collect::<Vec<u32>>())
        .collect::<Vec<Vec<u32>>>()
}

#[derive(Debug)]
struct Almanac {
    seeds: Vec<u32>,
    seed_to_soil: HashMap<u32, u32>,
    soil_to_fert: HashMap<u32, u32>,
    fert_to_water: HashMap<u32, u32>,
    water_to_light: HashMap<u32, u32>,
    light_to_temp: HashMap<u32, u32>,
    temp_to_humid: HashMap<u32, u32>,
    humid_to_loc: HashMap<u32, u32>
}