use std::collections::HashSet;
use std::env;
use std::fs;
use std::io;
use std::io::BufRead;
use std::iter::FromIterator;
use std::time::Instant;

// PART 1: 144381670
// PART 2: 20532569
fn main() {
    let timer = Instant::now();
    let filename = env::args().nth(1).unwrap_or(String::from("input.txt"));
    let lines = read_file_to_vec(filename);
    let (part_1_answer, part_2_answer) = answers(&lines);

    println!("PART 1: {}", part_1_answer);
    println!("PART 2: {}", part_2_answer);
    println!("Execution completed in {}ms", timer.elapsed().as_millis())
}

fn read_file_to_vec(filename: String) -> Vec<u64> {
    println!("Reading file {}", filename);
    let file = fs::File::open(filename).unwrap();
    let reader = io::BufReader::new(file);
    let mut parsed_lines: Vec<u64> = Vec::new();
    for (_, line) in reader.lines().enumerate() {
        parsed_lines.push(line.unwrap().parse().unwrap());
    }
    parsed_lines
}

// Returns the invalid number
fn run_part1(numbers: &Vec<u64>, preamble_length: usize) -> u64 {
    for (i, number) in numbers.iter().enumerate() {
        if i < preamble_length {
            continue;
        }
        let last_5_values = numbers[i.checked_sub(preamble_length).unwrap()..i]
            .iter()
            .map(|x| x.clone())
            .collect();
        if !has_summing_pair(&last_5_values, number.clone()) {
            return number.clone();
        }
    }
    0
}

fn run_part2(numbers: &Vec<u64>, sum_to_number: u64) -> u64 {
    let mut current_window: Vec<u64> = Vec::new();
    for (_, number) in numbers.iter().enumerate() {
        current_window.push(number.clone());
        let mut current_sum: u64 = current_window.iter().sum();
        while current_sum > sum_to_number {
            let removed = current_window.drain(..1).next().unwrap_or(0);
            current_sum -= removed;
        }
        if current_sum == sum_to_number {
            println!(
                "Found {} numbers that sum to {}",
                current_window.len(),
                sum_to_number
            );
            break;
        }
    }
    let min = current_window.iter().min().cloned().unwrap_or(0);
    let max = current_window.iter().max().cloned().unwrap_or(0);
    min + max
}

fn has_summing_pair(values: &Vec<u64>, sum_to_value: u64) -> bool {
    let value_set: HashSet<u64> = HashSet::from_iter(values.iter().cloned());
    for e1 in values.iter() {
        let e2 = sum_to_value.checked_sub(e1.clone());
        if !e2.is_none() && value_set.contains(&e2.unwrap()) {
            return true;
        }
    }
    false
}

#[allow(unused)]
fn answers(lines: &Vec<u64>) -> (u64, u64) {
    (run_part1(lines, 25), run_part2(lines, 144381670))
}

#[test]
fn test() {
    let lines = read_file_to_vec(String::from("sample_input.txt"));
    let part_1_answer = run_part1(&lines, 5);
    println!("PART1: {}", part_1_answer);
    assert_eq!(part_1_answer, 127);
}
#[test]
fn test_part2() {
    let lines = read_file_to_vec(String::from("sample_input.txt"));
    let answer = run_part2(&lines, 127);
    println!("PART2: {}", answer);
    assert_eq!(answer, 62);
}
