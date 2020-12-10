use std::collections::HashMap;
use std::env;
use std::fs;
use std::io;
use std::io::BufRead;
use std::time::Instant;

// PART 1: 2080
// PART 2: 6908379398144
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

fn run_part1(numbers: &Vec<u64>) -> u64 {
    let mut jolt_difference_counts: HashMap<u64, u64> = HashMap::new();
    let mut previous_number = 0;
    let mut sorted_numbers = numbers.clone();
    sorted_numbers.sort();
    jolt_difference_counts.insert(3, 1);
    for (i, number) in sorted_numbers.iter().enumerate() {
        if i == 0 {
            jolt_difference_counts.insert(number.clone(), 1);
        } else {
            let current_difference = number - previous_number;
            let new_count = *jolt_difference_counts
                .get(&current_difference)
                .unwrap_or(&0)
                + 1;
            jolt_difference_counts.insert(current_difference, new_count);
        }
        previous_number = *number;
    }
    jolt_difference_counts.get(&1).cloned().unwrap_or(0)
        * jolt_difference_counts.get(&3).cloned().unwrap_or(0)
}

fn run_part2(numbers: &Vec<u64>) -> u64 {
    let mut previous_number = 0;
    let mut sorted_numbers = numbers.clone();
    let mut run_length = 0;
    let mut total = 1;
    sorted_numbers.sort();
    for number in sorted_numbers {
        let current_difference = number - previous_number;
        previous_number = number;
        if current_difference == 1 {
            run_length += 1
        } else {
            total *= match run_length {
                4 => 7,
                3 => 4,
                2 => 2,
                _ => 1,
            };
            run_length = 0;
        }
        if current_difference == 2 {
            println!("Found a difference of 2: {} - {}", number, previous_number);
        }
    }
    total *= match run_length {
        4 => 7,
        3 => 4,
        2 => 2,
        _ => 1,
    };
    total
}

#[allow(unused)]
fn answers(lines: &Vec<u64>) -> (u64, u64) {
    (run_part1(lines), run_part2(&lines))
}

#[test]
fn test() {
    let lines = read_file_to_vec(String::from("sample_input_2.txt"));
    let part_1_answer = run_part1(&lines);
    // for (k, v) in part_1_answer {
    //     println!("{} = {}", k, v);
    // }
    assert_eq!(part_1_answer, 220)
}
#[test]
fn test_part2() {
    let lines = read_file_to_vec(String::from("sample_input_2.txt"));
    assert_eq!(run_part2(&lines), 19208);
}
