use crate::utils::{print_matrix, read_lines};

pub fn part_one(input_file: &str) -> u32 {
    let mut engin_map = create_engin_map_matrix(read_lines(input_file));
    print_matrix(engin_map);

    return 0;
}

fn create_engin_map_matrix(map_lines: Vec<String>) -> Vec<Vec<char>> {
    let mut engin_map: Vec<Vec<char>> = Vec::new();

    for line in map_lines {
        let mut map_row: Vec<char> = Vec::new();

        for c in line.chars() {
            map_row.push(c);
        }

        engin_map.push(map_row);
    }

    return engin_map;
}