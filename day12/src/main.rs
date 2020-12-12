use regex::Regex;
use std::env;
use std::fmt;
use std::fs;
use std::io;
use std::io::BufRead;
use std::str::FromStr;
use std::time::Instant;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum Direction {
    E,
    S,
    N,
    W,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum Action {
    E,
    S,
    N,
    W,
    L,
    R,
    F,
}

impl FromStr for Action {
    type Err = ();

    fn from_str(s: &str) -> Result<Action, ()> {
        match s {
            "E" => Ok(Action::E),
            "S" => Ok(Action::S),
            "N" => Ok(Action::N),
            "W" => Ok(Action::W),
            "L" => Ok(Action::L),
            "R" => Ok(Action::R),
            "F" => Ok(Action::F),
            _ => Err(()),
        }
    }
}

impl Action {
    fn to_direction(&self) -> Option<Direction> {
        match self {
            Action::E => Option::Some(Direction::E),
            Action::S => Option::Some(Direction::S),
            Action::W => Option::Some(Direction::W),
            Action::N => Option::Some(Direction::N),
            _ => Option::None,
        }
    }
}

impl FromStr for Direction {
    type Err = ();

    fn from_str(s: &str) -> Result<Direction, ()> {
        match s {
            "E" => Ok(Direction::E),
            "S" => Ok(Direction::S),
            "N" => Ok(Direction::N),
            "W" => Ok(Direction::W),
            _ => Err(()),
        }
    }
}

impl fmt::Display for Direction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Direction::E => "E",
                Direction::N => "N",
                Direction::S => "S",
                Direction::W => "W",
            }
        )
    }
}
impl Direction {
    fn rotate(&self, degrees: i32) -> Direction {
        let directions: Vec<Direction> =
            vec![Direction::E, Direction::S, Direction::W, Direction::N];
        let rotation = degrees / 90;
        let mut current_face_index = directions.iter().position(|d| d == self).unwrap();
        current_face_index = add(current_face_index, 4 + rotation).unwrap() % 4;
        directions[current_face_index]
    }

    fn multiplier(&self) -> i32 {
        match self {
            Direction::S | Direction::W => -1,
            _ => 1,
        }
    }
}

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
    let mut face = Direction::E;
    let mut east_west_position: i32 = 0;
    let mut north_south_position: i32 = 0;
    let line_regex = Regex::new(r"^(?P<action>\w)(?P<value>\d+)$").unwrap();
    for line in lines {
        let captures = line_regex.captures(line).unwrap();
        let action = captures
            .name("action")
            .unwrap()
            .as_str()
            .parse::<Action>()
            .unwrap();
        let value = captures
            .name("value")
            .unwrap()
            .as_str()
            .parse::<i32>()
            .unwrap();
        match action {
            Action::F => match face {
                Direction::N | Direction::S => north_south_position += face.multiplier() * value,
                Direction::E | Direction::W => east_west_position += face.multiplier() * value,
            },
            Action::N | Action::S => {
                north_south_position += action.to_direction().unwrap().multiplier() * value
            }
            Action::E | Action::W => {
                east_west_position += action.to_direction().unwrap().multiplier() * value
            }
            Action::R => face = face.rotate(value),
            Action::L => face = face.rotate(-value),
        }
    }
    let manhattan_distance = east_west_position.abs() + north_south_position.abs();
    manhattan_distance as u64
}

fn run_part2(lines: &Vec<String>) -> u64 {
    // Always keep the E/W direction in position 0, and N/S in position 1 to simplify action processing
    let mut waypoint_face = (Direction::E, Direction::N);
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
            .parse::<Action>()
            .unwrap();
        let value = captures
            .name("value")
            .unwrap()
            .as_str()
            .parse::<i32>()
            .unwrap();
        match action {
            Action::F => {
                east_west_position += waypoint.0 * waypoint_face.0.multiplier() * value;
                north_south_position += waypoint.1 * waypoint_face.1.multiplier() * value;
            }
            Action::N | Action::S => {
                waypoint.1 += if waypoint_face.1 == action.to_direction().unwrap() {
                    1
                } else {
                    -1
                } * value;
            }
            Action::E | Action::W => {
                waypoint.0 += if waypoint_face.0 == action.to_direction().unwrap() {
                    1
                } else {
                    -1
                } * value;
            }
            Action::L | Action::R => {
                let multiplier = if action == Action::L { -1 } else { 1 };
                let new_face = (
                    waypoint_face.0.rotate(multiplier * value),
                    waypoint_face.1.rotate(multiplier * value),
                );
                if new_face.0 == Direction::N || new_face.0 == Direction::S {
                    waypoint = (waypoint.1, waypoint.0);
                    waypoint_face = (new_face.1, new_face.0);
                } else {
                    waypoint_face = new_face;
                }
            }
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
    let input = read_file_to_vec(String::from("input.txt"));
    assert_eq!(run_part2(&input), 26841);
}

#[test]
fn test_rotate() {
    assert_eq!(Direction::E.rotate(90), Direction::S);
    assert_eq!(Direction::E.rotate(-90), Direction::N);
    assert_eq!(Direction::E.rotate(180), Direction::W);
    assert_eq!(Direction::E.rotate(-270), Direction::S);
    assert_eq!(Direction::E.rotate(270), Direction::N);
}
