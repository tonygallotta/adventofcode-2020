use std::env;
use std::fs;
use std::io;
use std::io::BufRead;

fn main() {
    let filename = env::args().nth(1).unwrap_or(String::from("input.txt"));
    let parsed_lines = read_file_to_vec(filename);
    let part_1_answer = part1(&parsed_lines, 31, 3, 1);
    println!("PART 1: {}", part_1_answer);
    let part_2_answer = part_1_answer
        * part1(&parsed_lines, 31, 1, 1)
        * part1(&parsed_lines, 31, 5, 1)
        * part1(&parsed_lines, 31, 7, 1)
        * part1(&parsed_lines, 31, 1, 2);
    println!("PART 2: {}", part_2_answer);
}

fn read_file_to_vec(filename: String) -> Vec<Vec<char>> {
    println!("Reading file {}", filename);
    let file = fs::File::open(filename).unwrap();
    let reader = io::BufReader::new(file);
    let mut parsed_lines : Vec<Vec<char>> = Vec::new();
    for (_, line) in reader.lines().enumerate() {
        let unwrapped = line.unwrap();
        parsed_lines.push(unwrapped.chars().collect());
    }
    return parsed_lines;
}

fn part1(lines: &Vec<Vec<char>>, line_length: usize, right: usize, down: usize) -> u32 {
    let mut row :usize = 0;
    let mut column :usize = 0;
    let row_count = lines.len();
    let mut trees_hit = 0;
    while row < row_count {
        row += down;
        column = (column + right) % line_length;
        if row < row_count && lines[row][column] == '#' {
            trees_hit += 1;
        }
    }
    return trees_hit;
}

#[test]
fn test() {
    let sample_lines = read_file_to_vec(String::from("sample_input.txt"));
    assert_eq!(7, part1(&sample_lines, 66, 3, 1));
}

