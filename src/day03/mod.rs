use crate::utils::{print_matrix, read_lines};

pub fn part_one(input_file: &str) -> u32 {
    let mut engin_map = create_engin_map_matrix(read_lines(input_file));
    let mut sum: u32 = 0;

    for (i, row) in engin_map.iter().enumerate() {
        let mut num_str = String::new();

        'outer: for (j, &c) in row.iter().enumerate() {
            while c.is_digit(10) {
                num_str.push(c);
                continue 'outer;
            }

            if num_str.is_empty() {
                continue;
            }

            let start: i32 = j as i32 - num_str.len() as i32;
            let num: u32 = num_str.parse().unwrap();
            num_str.clear();

            // At this point, I have a full number.  So start checking for adj. symbol.
            if is_symbol(&engin_map, i as i32, start - 1) || is_symbol(&engin_map, i as i32, j as i32) {
                sum += num;
                continue;
            }

            for k in start - 1 .. j as i32 + 1 {
                if is_symbol(&engin_map, i as i32 - 1, k) || is_symbol(&engin_map, i as i32 + 1, k) {
                    sum += num;
                    continue;
                }
            }
        }
    }

    return sum;
}

pub fn part_two(input_file: &str) -> u32 {
    let mut engin_map = create_engin_map_matrix(read_lines(input_file));
    let mut possible_gears: Vec<Vec<u32>> = Vec::new();
    let mut sum: u32 = 0;


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

fn is_symbol(mat: &Vec<Vec<char>>, i: i32, j: i32) -> bool {
    if !((i >= 0 && i < mat.len() as i32) && (j >= 0 && j < mat[0].len() as i32)) {
    // if i < 0 || i >= mat.len() as i32 || j < 0 || j >= mat[0].len() as i32 {i32
        return false;
    }

    return mat[i as usize][j as usize] != '.' && !mat[i as usize][j as usize].is_digit(10);
}