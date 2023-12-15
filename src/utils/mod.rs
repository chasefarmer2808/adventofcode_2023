use std::fs::read_to_string;

pub fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename)
        .unwrap()
        .lines()
        .map(String::from)
        .collect()
}

pub fn print_matrix(matrix: Vec<Vec<char>>) {
    matrix.iter().for_each(|row| {
        row.iter().for_each(|col| print!("{}", col));
        println!();
    })
}