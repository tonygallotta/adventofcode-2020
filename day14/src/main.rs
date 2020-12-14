use regex::Regex;
use std::collections::HashMap;
use std::env;
use std::fs;
use std::io;
use std::io::BufRead;
use std::time::Instant;

// PART 1: 3059488894985
// PART 2: 2900994392308
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
    let instruction_re = Regex::new(r"^(?P<instr>.*) = (?P<value>\w+)$").unwrap();
    let mem_re = Regex::new(r"^mem\[(?P<addr>\d+)\].*$").unwrap();
    let mut mask_to_and: u64 = 1;
    let mut mask_to_or: u64 = 0;
    let mut mem: HashMap<u64, u64> = HashMap::new();
    for line in lines {
        let instruction_capture = instruction_re.captures(line).unwrap();
        let instruction = instruction_capture.name("instr").unwrap().as_str();
        let value = instruction_capture.name("value").unwrap().as_str();
        if instruction == "mask" {
            mask_to_and =
                u64::from_str_radix(String::from(value).replace("X", "1").as_str(), 2).unwrap();
            mask_to_or =
                u64::from_str_radix(String::from(value).replace("X", "0").as_str(), 2).unwrap();
        } else {
            let addr = mem_re
                .captures(instruction)
                .unwrap()
                .name("addr")
                .unwrap()
                .as_str()
                .parse::<u64>()
                .unwrap();
            mem.insert(
                addr,
                value.parse::<u64>().unwrap() & mask_to_and | mask_to_or,
            );
        }
    }
    mem.values().sum()
}

fn run_part2(lines: &Vec<String>) -> u64 {
    let instruction_re = Regex::new(r"^(?P<instr>.*) = (?P<value>\w+)$").unwrap();
    let mem_re = Regex::new(r"^mem\[(?P<addr>\d+)\].*$").unwrap();
    let mut mask: &str = "";
    let mut mem: HashMap<u64, u64> = HashMap::new();
    for line in lines {
        let instruction_capture = instruction_re.captures(line).unwrap();
        let instruction = instruction_capture.name("instr").unwrap().as_str();
        let value = instruction_capture.name("value").unwrap().as_str();
        if instruction == "mask" {
            mask = value;
        } else {
            let addr = format!(
                "{:036b}",
                mem_re
                    .captures(instruction)
                    .unwrap()
                    .name("addr")
                    .unwrap()
                    .as_str()
                    .parse::<u64>()
                    .unwrap()
            );
            let value_to_write = value.parse::<u64>().unwrap();
            let floating_bits = count_xs(mask);
            let mask_chars: Vec<char> = mask.chars().collect();
            for i in 0..2_u64.pow(floating_bits as u32) {
                let mut to_change: Vec<char> = format!("{:09b}", i)
                    .chars()
                    .skip(9_i8.checked_sub(floating_bits as i8).unwrap() as usize)
                    .collect();
                // println!("Permutation: {}", format!("{:09b}", i));
                let mut addr_to_write = String::new();
                for (j, c) in addr.chars().enumerate() {
                    let bitmask_char = mask_chars.get(j).unwrap();
                    match bitmask_char {
                        '1' => addr_to_write.push(*bitmask_char),
                        '0' => addr_to_write.push(c),
                        _ => addr_to_write.push(to_change.pop().unwrap()),
                    }
                }
                // println!("Writing to {}", addr_to_write);
                let decimal_addr = u64::from_str_radix(addr_to_write.as_str(), 2).unwrap();
                // println!("{} = {}", decimal_addr, value_to_write);
                mem.insert(decimal_addr, value_to_write);
            }
        }
    }
    mem.values().sum()
}

fn count_xs(l: &str) -> usize {
    l.replace("0", "").replace("1", "").len()
}

#[test]
fn test_part1() {
    let lines = read_file_to_vec(String::from("sample_input.txt"));
    assert_eq!(run_part1(&lines), 165);
}

#[test]
fn test_part2() {
    let lines = read_file_to_vec(String::from("sample_input_2.txt"));
    assert_eq!(run_part2(&lines), 208);
}

#[test]
fn test_masks() {
    let lines = read_file_to_vec(String::from("masks.txt"));
    let max_xs = lines
        .iter()
        .map(|l| l.replace("0", "").replace("1", "").len())
        .max()
        .unwrap();
    println!("Max is {}", max_xs);
}
