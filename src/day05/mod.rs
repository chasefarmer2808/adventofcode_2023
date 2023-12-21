use std::hash::Hash;
use std::ops::{ Range};
use crate::utils::read_lines;

pub fn part_one(input_file: &str) -> i64 {
    let almanac_lines = read_lines(input_file);
    let mut seeds: Vec<i64> = Vec::new();
    let mut min = i64::MAX;
    let mut mappings: Vec<Mapping> = Vec::new();

    // Parse Almanac
    for (i, line) in almanac_lines.iter().enumerate() {
        if line.contains("seeds:") {
            seeds = parse_seeds(&line);
            continue;
        }

        if line.contains("seed-to-soil map:") {
            let mapping_nums: Vec<Vec<i64>> = num_strings_to_num_vecs(lines_to_num_strings(&almanac_lines[i+1..]));

            let mut mapping = Mapping::default();
            for mapping_num in mapping_nums {
                mapping.maps.push(create_mapping(mapping_num[0], mapping_num[1], mapping_num[2]))
            }
            mappings.push(mapping);
            continue;
        }

        if line.contains("soil-to-fertilizer map:") {
            let mapping_nums: Vec<Vec<i64>> = num_strings_to_num_vecs(lines_to_num_strings(&almanac_lines[i+1..]));

            let mut mapping = Mapping::default();
            for mapping_num in mapping_nums {
                mapping.maps.push(create_mapping(mapping_num[0], mapping_num[1], mapping_num[2]))
            }
            mappings.push(mapping);
            continue;
        }

        if line.contains("fertilizer-to-water map:") {
            let mapping_nums: Vec<Vec<i64>> = num_strings_to_num_vecs(lines_to_num_strings(&almanac_lines[i+1..]));

            let mut mapping = Mapping::default();
            for mapping_num in mapping_nums {
                mapping.maps.push(create_mapping(mapping_num[0], mapping_num[1], mapping_num[2]))
            }
            mappings.push(mapping);
            continue;
        }

        if line.contains("water-to-light map:") {
            let mapping_nums: Vec<Vec<i64>> = num_strings_to_num_vecs(lines_to_num_strings(&almanac_lines[i+1..]));

            let mut mapping = Mapping::default();
            for mapping_num in mapping_nums {
                mapping.maps.push(create_mapping(mapping_num[0], mapping_num[1], mapping_num[2]))
            }
            mappings.push(mapping);
            continue;
        }

        if line.contains("light-to-temperature map:") {
            let mapping_nums: Vec<Vec<i64>> = num_strings_to_num_vecs(lines_to_num_strings(&almanac_lines[i+1..]));

            let mut mapping = Mapping::default();
            for mapping_num in mapping_nums {
                mapping.maps.push(create_mapping(mapping_num[0], mapping_num[1], mapping_num[2]))
            }
            mappings.push(mapping);
            continue;
        }

        if line.contains("temperature-to-humidity map:") {
            let mapping_nums: Vec<Vec<i64>> = num_strings_to_num_vecs(lines_to_num_strings(&almanac_lines[i+1..]));

            let mut mapping = Mapping::default();
            for mapping_num in mapping_nums {
                mapping.maps.push(create_mapping(mapping_num[0], mapping_num[1], mapping_num[2]))
            }
            mappings.push(mapping);
            continue;
        }

        if line.contains("humidity-to-location map:") {
            let mapping_nums: Vec<Vec<i64>> = num_strings_to_num_vecs(lines_to_num_strings(&almanac_lines[i+1..]));

            let mut mapping = Mapping::default();
            for mapping_num in mapping_nums {
                mapping.maps.push(create_mapping(mapping_num[0], mapping_num[1], mapping_num[2]))
            }
            mappings.push(mapping);
            continue;
        }
    }

    // Gather locations.
    for seed in &seeds {
        println!("parsing seed {:?}", seed);
        let mut cur = *seed;
        for map in &mappings {
            cur = map.apply_map(cur);
        }
        min = min.min(cur);
    }

    return min;
}

fn parse_seeds(seed_line: &str) -> Vec<i64> {
    seed_line
        .split_whitespace()
        .filter(|s| s.parse::<i64>().is_ok())
        .map(|s| s.parse::<i64>().unwrap())
        .collect::<Vec<i64>>()
}

fn lines_to_num_strings(num_lines: &[String]) -> Vec<&String> {
    num_lines
        .iter()
        .take_while(|line| !line.is_empty())
        .collect()
}

fn num_strings_to_num_vecs(num_strings: Vec<&String>) -> Vec<Vec<i64>> {
    num_strings
        .iter()
        .map(|s| s.split_whitespace()
            .map(|ns| ns.parse::<i64>().unwrap())
            .collect::<Vec<i64>>())
        .collect::<Vec<Vec<i64>>>()
}

fn create_mapping(dest: i64, src: i64, r: i64) -> SingleMap {
    SingleMap {range: src..src+r, delta: dest - src}
}

#[derive(Debug, Default)]
struct Mapping {
    maps: Vec<SingleMap>
}

impl Mapping {
    fn apply_map(&self, val: i64) -> i64 {
        for map in &self.maps {
            if map.range.contains(&val) {
                return val + map.delta;
            }
        }
        val
    }
}

#[derive(Debug, Default)]
struct SingleMap {
    range: Range<i64>,
    delta: i64
}