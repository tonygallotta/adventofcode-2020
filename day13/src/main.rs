use math::round;
use std::env;
use std::fs;
use std::io;
use std::io::BufRead;
use std::time::Instant;

// PART 1: 1835
// PART 2: 247086664214628
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
    let earliest_departure_time = lines.get(0).unwrap().parse::<u64>().unwrap();
    let bus_numbers: Vec<u64> = lines
        .get(1)
        .unwrap()
        .split(",")
        .filter(|v| *v != "x")
        .map(|v| v.parse::<u64>().unwrap())
        .collect();
    let mut min_wait_time: u64 = 10000000000;
    let mut best_bus = 0;
    for bus_number in bus_numbers {
        let multiplier: u64 =
            round::ceil(earliest_departure_time as f64 / bus_number as f64, 0) as u64;
        let wait_time: u64 = (bus_number * multiplier) - earliest_departure_time;
        if wait_time < min_wait_time {
            min_wait_time = wait_time;
            best_bus = bus_number;
        }
    }
    min_wait_time * best_bus
}

fn run_part2(lines: &Vec<String>) -> u64 {
    let bus_numbers: Vec<u64> = lines
        .get(1)
        .unwrap()
        .split(",")
        .map(|v| {
            if v == "x" {
                0
            } else {
                v.parse::<u64>().unwrap()
            }
        })
        .collect();
    let max_bus_number = 12_088_973;
    let max_bus_number_position: u64 = 48;
    let mut timestamp: u64 = 12_088_973;
    loop {
        while (timestamp - 31) % 79_066_847 != 0 {
            timestamp += max_bus_number;
        }
        let mut works = true;
        for (i, bus_number) in bus_numbers.iter().enumerate() {
            if *bus_number != 0 {
                let required_timestamp =
                    timestamp.checked_sub(max_bus_number_position).unwrap() + i as u64;
                if required_timestamp % bus_number != 0 {
                    works = false;
                    break;
                }
            }
        }
        if works {
            return timestamp.checked_sub(max_bus_number_position).unwrap();
        }
        timestamp += max_bus_number;
    }
}

#[allow(dead_code)]
fn add(u: usize, i: i32) -> Option<usize> {
    if i.is_negative() {
        u.checked_sub(i.wrapping_abs() as u32 as usize)
    } else {
        u.checked_add(i as usize)
    }
}

#[test]
fn test_part1() {
    let mut lines = Vec::new();
    lines.push(String::from("939"));
    lines.push(String::from("7,13,x,x,59,x,31,19"));
    assert_eq!(run_part1(&lines), 295);
}

#[test]
fn test_part2() {
    // let mut lines = Vec::new();
    // lines.push(String::from("939"));
    // lines.push(String::from("7,13,x,x,59,x,31,19"));
    // assert_eq!(run_part2(&lines), 1068781);
    //
    // assert_eq!(
    //     run_part2(&vec![String::from(""), String::from("1789,37,47,1889")]),
    //     1202161486
    // );
    println!("PART 2 {}", run_part2(&vec![String::from(""), String::from("17,x,x,x,x,x,x,41,x,x,x,37,x,x,x,x,x,367,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,19,x,x,x,23,x,x,x,x,x,29,x,613,x,x,x,x,x,x,x,x,x,x,x,x,13")]));
}
