use regex::Regex;
use std::env;
use std::fs;
use std::io;
use std::io::BufRead;
use std::time::Instant;

// PART 1: 636
// PART 2: 26841
fn main() {
    let timer = Instant::now();
    let filename = env::args()
        .nth(1)
        .unwrap_or_else(|| String::from("input.txt"));
    let lines = read_file_to_vec(filename);
    let (part_1_answer, part_2_answer) = answers(&lines);

    println!("PART 1: {}", part_1_answer);
    println!("PART 2: {}", part_2_answer);
    println!("Execution completed in {}ms", timer.elapsed().as_millis())
}

fn read_file_to_vec(filename: String) -> Vec<String> {
    println!("Reading file {}", filename);
    let file = fs::File::open(filename).unwrap();
    let reader = io::BufReader::new(file);
    let mut parsed_lines = Vec::new();
    for (_, line) in reader.lines().enumerate() {
        parsed_lines.push(line.unwrap());
    }
    parsed_lines
}

fn answers(lines: &Vec<String>) -> (u64, u64) {
    (run_part1(lines), run_part2(&lines))
}

fn run_part1(lines: &Vec<String>) -> u64 {
    let directions = vec!['E', 'S', 'W', 'N'];
    let mut face = 'E';
    let mut east_west_position: i32 = 0;
    let mut north_south_position: i32 = 0;
    let line_regex = Regex::new(r"^(?P<action>\w)(?P<value>\d+)$").unwrap();
    for line in lines {
        let captures = line_regex.captures(line).unwrap();
        let action = captures
            .name("action")
            .unwrap()
            .as_str()
            .parse::<char>()
            .unwrap();
        let value = captures
            .name("value")
            .unwrap()
            .as_str()
            .parse::<i32>()
            .unwrap();
        // println!("action = {}, value = {}", action, value);
        match action {
            'F' => match face {
                'E' => east_west_position += value,
                'W' => east_west_position -= value,
                'N' => north_south_position += value,
                'S' => north_south_position -= value,
                _ => {}
            },
            'N' => north_south_position += value,
            'S' => north_south_position -= value,
            'E' => east_west_position += value,
            'W' => east_west_position -= value,
            'R' => {
                let rotation = value / 90;
                let mut current_face_index = directions.iter().position(|d| d == &face).unwrap();
                current_face_index = add(current_face_index, rotation).unwrap() % 4;
                face = directions[current_face_index];
            }
            'L' => {
                let rotation = value / 90;
                let mut current_face_index = directions.iter().position(|d| d == &face).unwrap();
                current_face_index = add(current_face_index, 4 - rotation).unwrap() % 4;
                face = directions[current_face_index];
            }
            _ => {}
        }
    }
    let manhattan_distance = east_west_position.abs() + north_south_position.abs();
    manhattan_distance as u64
}

fn run_part2(lines: &Vec<String>) -> u64 {
    let directions = vec!['E', 'S', 'W', 'N'];
    let mut waypoint_face = ('E', 'N');
    let mut waypoint = (10, 1);
    let mut east_west_position: i32 = 0;
    let mut north_south_position: i32 = 0;
    let line_regex = Regex::new(r"^(?P<action>\w)(?P<value>\d+)$").unwrap();
    for line in lines {
        let captures = line_regex.captures(line).unwrap();
        let action = captures
            .name("action")
            .unwrap()
            .as_str()
            .parse::<char>()
            .unwrap();
        let value = captures
            .name("value")
            .unwrap()
            .as_str()
            .parse::<i32>()
            .unwrap();
        // println!("action = {}, value = {}", action, value);
        match action {
            'F' => {
                // println!("Now facing {}, {}", waypoint_face.0, waypoint_face.1);
                match waypoint_face {
                    ('E', 'N') => {
                        east_west_position += waypoint.0 * value;
                        north_south_position += waypoint.1 * value;
                    }
                    ('W', 'N') => {
                        east_west_position -= waypoint.0 * value;
                        north_south_position += waypoint.1 * value;
                    }
                    ('W', 'S') => {
                        east_west_position -= waypoint.0 * value;
                        north_south_position -= waypoint.1 * value;
                    }
                    ('E', 'S') => {
                        east_west_position += waypoint.0 * value;
                        north_south_position -= waypoint.1 * value;
                    }
                    ('N', 'E') => {
                        east_west_position += waypoint.1 * value;
                        north_south_position += waypoint.0 * value;
                    }
                    ('N', 'W') => {
                        east_west_position -= waypoint.1 * value;
                        north_south_position += waypoint.0 * value;
                    }
                    ('S', 'W') => {
                        east_west_position -= waypoint.1 * value;
                        north_south_position -= waypoint.0 * value;
                    }
                    ('S', 'E') => {
                        east_west_position += waypoint.1 * value;
                        north_south_position -= waypoint.0 * value;
                    }
                    _ => {}
                };
            }
            'N' => match waypoint_face.0 {
                'E' | 'W' => match waypoint_face.1 {
                    'N' => waypoint.1 += value,
                    'S' => waypoint.1 -= value,
                    _ => {}
                },
                'N' => waypoint.0 += value,
                'S' => waypoint.0 -= value,
                _ => {}
            },
            'S' => match waypoint_face.0 {
                'E' | 'W' => match waypoint_face.1 {
                    'N' => waypoint.1 -= value,
                    'S' => waypoint.1 += value,
                    _ => {}
                },
                'N' => waypoint.0 -= value,
                'S' => waypoint.0 += value,
                _ => {}
            },
            'E' => match waypoint_face.0 {
                'N' | 'S' => match waypoint_face.1 {
                    'E' => waypoint.1 += value,
                    'W' => waypoint.1 -= value,
                    _ => {}
                },
                'E' => waypoint.0 += value,
                'W' => waypoint.0 -= value,
                _ => {}
            },
            'W' => match waypoint_face.0 {
                'N' | 'S' => match waypoint_face.1 {
                    'E' => waypoint.1 -= value,
                    'W' => waypoint.1 += value,
                    _ => {}
                },
                'E' => waypoint.0 -= value,
                'W' => waypoint.0 += value,
                _ => {}
            },
            'R' => {
                let rotation = value / 90;
                let mut current_face_index = (
                    directions
                        .iter()
                        .position(|d| d == &waypoint_face.0)
                        .unwrap(),
                    directions
                        .iter()
                        .position(|d| d == &waypoint_face.1)
                        .unwrap(),
                );
                current_face_index = (
                    add(current_face_index.0, rotation).unwrap() % 4,
                    add(current_face_index.1, rotation).unwrap() % 4,
                );
                waypoint_face = (
                    directions[current_face_index.0],
                    directions[current_face_index.1],
                );
            }
            'L' => {
                let rotation = value / 90;
                let mut current_face_index = (
                    directions
                        .iter()
                        .position(|d| d == &waypoint_face.0)
                        .unwrap(),
                    directions
                        .iter()
                        .position(|d| d == &waypoint_face.1)
                        .unwrap(),
                );
                current_face_index = (
                    add(current_face_index.0, 4 - rotation).unwrap() % 4,
                    add(current_face_index.1, 4 - rotation).unwrap() % 4,
                );
                waypoint_face = (
                    directions[current_face_index.0],
                    directions[current_face_index.1],
                );
            }
            _ => {}
        }
    }
    let manhattan_distance = east_west_position.abs() + north_south_position.abs();
    manhattan_distance as u64
}

fn add(u: usize, i: i32) -> Option<usize> {
    if i.is_negative() {
        u.checked_sub(i.wrapping_abs() as u32 as usize)
    } else {
        u.checked_add(i as usize)
    }
}

#[test]
fn test_part1() {
    let lines = read_file_to_vec(String::from("sample_input.txt"));
    assert_eq!(run_part1(&lines), 25);
}
#[test]
fn test_part2() {
    let lines = read_file_to_vec(String::from("sample_input.txt"));
    assert_eq!(run_part2(&lines), 286);
}
