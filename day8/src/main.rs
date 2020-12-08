use std::collections::HashSet;
use std::env;
use std::fs;
use std::io;
use std::io::BufRead;
use std::iter::FromIterator;
use std::time::Instant;

// PART 1: 2014
// PART 2: 2251
fn main() {
    let timer = Instant::now();
    let filename = env::args().nth(1).unwrap_or(String::from("input.txt"));
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
    let mut parsed_lines: Vec<String> = Vec::new();
    for (_, line) in reader.lines().enumerate() {
        parsed_lines.push(line.unwrap());
    }
    parsed_lines
}

// Returns a tuple of the accumulator value, and a boolean indicating whether the program ran to completion.
fn run_program(lines: &Vec<String>) -> (i32, bool) {
    let mut accumulator: i32 = 0;
    let mut instruction_number: usize = 0;
    let mut executed_instructions: HashSet<usize> = HashSet::new();

    while !executed_instructions.contains(&instruction_number) && instruction_number < lines.len() {
        let instruction: &String = &lines[instruction_number];
        let parts: Vec<&str> = Vec::from_iter(instruction.split(" "));
        let op = parts[0];
        let arg: i32 = parts[1].parse().unwrap();
        executed_instructions.insert(instruction_number);
        match op {
            "acc" => {
                accumulator += arg;
                instruction_number += 1;
            }
            "jmp" => instruction_number = add(instruction_number, arg).unwrap(),
            _ => instruction_number += 1,
        }
    }
    (accumulator, instruction_number == lines.len())
}

fn run_part_2(lines: &Vec<String>) -> i32 {
    for (i, line) in lines.iter().enumerate() {
        if line.contains("nop") || line.contains("jmp") {
            let change_to_try = copy_with_line_change(lines, i);
            let run_result = run_program(&change_to_try);
            if run_result.1 {
                return run_result.0;
            }
        }
    }
    0
}

fn copy_with_line_change(lines: &Vec<String>, line_to_change: usize) -> Vec<String> {
    let original = lines[line_to_change].as_str();
    let was_jmp = original.contains("jmp");
    let mut new_lines = Vec::new();
    for (idx, line) in lines.iter().enumerate() {
        if idx == line_to_change {
            if was_jmp {
                new_lines.push(String::from(original.replace("jmp", "nop")));
            } else {
                new_lines.push(String::from(original.replace("nop", "jmp")));
            }
        } else {
            new_lines.push(line.clone());
        }
    }
    new_lines
}

fn add(u: usize, i: i32) -> Option<usize> {
    if i.is_negative() {
        u.checked_sub(i.wrapping_abs() as u32 as usize)
    } else {
        u.checked_add(i as usize)
    }
}

#[allow(unused)]
fn answers(lines: &Vec<String>) -> (i32, i32) {
    (run_program(lines).0, run_part_2(lines))
}

#[test]
fn test() {
    let lines = read_file_to_vec(String::from("sample_input.txt"));
    let (part_1_answer, _) = run_program(&lines);
    println!("PART1: {}", part_1_answer);
}

#[test]
fn test_part2() {
    let lines = read_file_to_vec(String::from("sample_input.txt"));
    let good_example = copy_with_line_change(&lines, 7);
    assert!(!run_program(&lines).1);
    assert!(run_program(&good_example).1);
}
