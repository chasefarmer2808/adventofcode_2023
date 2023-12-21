use std::collections::HashMap;
use std::hash::Hash;
use crate::utils::read_lines;

pub fn part_one(input_file: &str) -> u64 {
    let almanac_lines = read_lines(input_file);
    let mut seeds: Vec<u64> = Vec::new();
    let mut locs: Vec<u64> = Vec::new();
    let mut seed_to_soil_map: HashMap<u64, u64> = HashMap::new();
    let mut soil_to_fert_map: HashMap<u64, u64> = HashMap::new();
    let mut fert_to_water_map: HashMap<u64, u64> = HashMap::new();
    let mut water_to_light_map: HashMap<u64, u64> = HashMap::new();
    let mut light_to_temp_map: HashMap<u64, u64> = HashMap::new();
    let mut temp_to_humid_map: HashMap<u64, u64> = HashMap::new();
    let mut humid_to_loc_map: HashMap<u64, u64> = HashMap::new();

    // Parse Almanac
    for (i, line) in almanac_lines.iter().enumerate() {
        if line.contains("seeds:") {
            seeds = parse_seeds(&line);
            continue;
        }

        if line.contains("seed-to-soil map:") {
            let seed_to_soil_nums_strings: Vec<&String> = lines_to_num_strings(&almanac_lines[i+1..]);
            let seed_to_soil_nums: Vec<Vec<u64>> = num_strings_to_num_vecs(seed_to_soil_nums_strings);

            parse_mappings(&seed_to_soil_nums, &mut seed_to_soil_map);
            continue;
        }

        if line.contains("soil-to-fertilizer map:") {
            let soil_to_fert_nums_strings: Vec<&String> = lines_to_num_strings(&almanac_lines[i+1..]);
            let soil_to_fert_nums: Vec<Vec<u64>> = num_strings_to_num_vecs(soil_to_fert_nums_strings);

            parse_mappings(&soil_to_fert_nums, &mut soil_to_fert_map);
            continue;
        }

        if line.contains("fertilizer-to-water map:") {
            let fert_to_water_nums_strings: Vec<&String> = lines_to_num_strings(&almanac_lines[i+1..]);
            let fert_to_water_nums: Vec<Vec<u64>> = num_strings_to_num_vecs(fert_to_water_nums_strings);

            parse_mappings(&fert_to_water_nums, &mut fert_to_water_map);
            continue;
        }

        if line.contains("water-to-light map:") {
            let nums: Vec<Vec<u64>> = num_strings_to_num_vecs(lines_to_num_strings(&almanac_lines[i+1..]));

            parse_mappings(&nums, &mut water_to_light_map);
            continue;
        }

        if line.contains("light-to-temperature map:") {
            let nums: Vec<Vec<u64>> = num_strings_to_num_vecs(lines_to_num_strings(&almanac_lines[i+1..]));

            parse_mappings(&nums, &mut light_to_temp_map);
            continue;
        }

        if line.contains("temperature-to-humidity map:") {
            let nums: Vec<Vec<u64>> = num_strings_to_num_vecs(lines_to_num_strings(&almanac_lines[i+1..]));

            parse_mappings(&nums, &mut temp_to_humid_map);
            continue;
        }

        if line.contains("humidity-to-location map:") {
            let nums: Vec<Vec<u64>> = num_strings_to_num_vecs(lines_to_num_strings(&almanac_lines[i+1..]));

            parse_mappings(&nums, &mut humid_to_loc_map);
            continue;
        }
    }

    // Gather locations.
    for seed in &seeds {
        let soil = get_mapping(&seed_to_soil_map, seed);
        let fert = get_mapping(&soil_to_fert_map, &soil);
        let water = get_mapping(&fert_to_water_map, &fert);
        let light = get_mapping(&water_to_light_map, &water);
        let temp = get_mapping(&light_to_temp_map, &light);
        let humid = get_mapping(&temp_to_humid_map, &temp);
        let loc = get_mapping(&humid_to_loc_map, &humid);

        locs.push(loc);
    }

    return locs.iter().min().unwrap().clone();
}

fn parse_seeds(seed_line: &str) -> Vec<u64> {
    seed_line
        .split_whitespace()
        .filter(|s| s.parse::<u64>().is_ok())
        .map(|s| s.parse::<u64>().unwrap())
        .collect::<Vec<u64>>()
}

fn lines_to_num_strings(num_lines: &[String]) -> Vec<&String> {
    num_lines
        .iter()
        .take_while(|line| !line.is_empty())
        .collect()
}

fn num_strings_to_num_vecs(num_strings: Vec<&String>) -> Vec<Vec<u64>> {
    num_strings
        .iter()
        .map(|s| s.split_whitespace()
            .map(|ns| ns.parse::<u64>().unwrap())
            .collect::<Vec<u64>>())
        .collect::<Vec<Vec<u64>>>()
}

fn parse_mappings(mappings: &Vec<Vec<u64>>, target_map: &mut HashMap<u64, u64>) {
    for mapping in mappings {
        if let [dest, src, r] = mapping[..] {
            let mut j = 0;

            for n in src..src+r {
                target_map.insert(n, dest + j);
                j += 1;
            }
        }
    }
}

fn get_mapping(mapping: &HashMap<u64, u64>, target: &u64) -> u64 {
    match mapping.contains_key(target) {
        true => mapping.get(target),
        false => Some(target)
    }.unwrap().clone()
}

#[derive(Debug)]
struct Almanac {
    seeds: Vec<u64>,
    seed_to_soil: HashMap<u64, u64>,
    soil_to_fert: HashMap<u64, u64>,
    fert_to_water: HashMap<u64, u64>,
    water_to_light: HashMap<u64, u64>,
    light_to_temp: HashMap<u64, u64>,
    temp_to_humid: HashMap<u64, u64>,
    humid_to_loc: HashMap<u64, u64>
}