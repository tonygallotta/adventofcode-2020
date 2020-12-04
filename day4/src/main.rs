use std::env;
use std::fs;
use std::io;
use std::io::BufRead;
use std::collections::HashSet;
use std::iter::FromIterator;
use regex::Regex;

fn main() {
    let filename = env::args().nth(1).unwrap_or(String::from("input.txt"));
    let parsed_lines = read_file_to_vec(filename);
    let mut required_fields: HashSet<String> = HashSet::new();
    required_fields.insert(String::from("byr")); // (Birth Year)
    required_fields.insert(String::from("iyr")); // (Issue Year)
    required_fields.insert(String::from("eyr")); // (Expiration Year)
    required_fields.insert(String::from("hgt")); // (Height)
    required_fields.insert(String::from("hcl")); // (Hair Color)
    required_fields.insert(String::from("ecl")); // (Eye Color)
    required_fields.insert(String::from("pid")); // (Passport ID)
    //required_fields.insert("cid"); // (Country ID)

    let hcl_regex = Regex::new(r"^#[0-9a-f]{6}$").unwrap();
    let pid_regex = Regex::new(r"^[0-9]{9}$").unwrap();
    let mut valid_ecls = HashSet::new();
    valid_ecls.insert("amb");
    valid_ecls.insert("blu");
    valid_ecls.insert("brn");
    valid_ecls.insert("gry");
    valid_ecls.insert("grn");
    valid_ecls.insert("hzl");
    valid_ecls.insert("oth");

    let mut part_1_answer = 0;
    for (_, passport_block) in parsed_lines.iter().enumerate() {
        // println!("Passport {}: \n{}", i, passport_block.join("\n"));
        let mut passport_fields: HashSet<String> = HashSet::new();
        let mut valid: bool = true;
        for (_, line) in passport_block.iter().enumerate() {
            // Vec::from_iter(line.split(" ").map(String::from));
            let key_value_pairs = Vec::from_iter(line.split(" ").map(String::from));
            for (_, key_value_pair) in key_value_pairs.iter().enumerate() {
                let split_kvp: Vec<String> = Vec::from_iter(key_value_pair.split(":").map(String::from));
                let key = &split_kvp[0];
                let value = &split_kvp[1];
                passport_fields.insert(key.clone());
                let is_value_valid = match key.as_str() {
                    "byr" => {
                        let numeric_value = value.parse::<i32>().unwrap();
                        numeric_value >= 1920 && numeric_value <= 2002
                    }
                    "iyr" => {
                        let numeric_value = value.parse::<i32>().unwrap();
                        numeric_value >= 2010 && numeric_value <= 2020
                    }
                    "eyr" => {
                        let numeric_value = value.parse::<i32>().unwrap();
                        numeric_value >= 2020 && numeric_value <= 2030
                    }
                    "hgt" => {
                        let units = value.chars().nth(value.len() - 2).unwrap_or('_');
                        let numeric_value = value.replace("in", "").replace("cm", "").parse::<u32>().unwrap();
                        println!("Checking height block {}, units={}, value={}", key_value_pair, units, numeric_value);
                        let mut result = false;
                        if units == 'i' {
                            result = numeric_value >= 59 && numeric_value <= 76;
                        } else if units == 'c' {
                            result = numeric_value >= 150 && numeric_value <= 193;
                        }
                        result
                    }
                    "hcl" => hcl_regex.is_match(value),
                    "ecl" => valid_ecls.contains(value.as_str()),
                    "pid" => pid_regex.is_match(value),
                    _ => true
                };
                valid = valid && is_value_valid
            }
        }
        if valid && required_fields.is_subset(&passport_fields) {
            part_1_answer += 1;
        }
    }

    println!("PART 1: {}", part_1_answer);
}

fn read_file_to_vec(filename: String) -> Vec<Vec<String>> {
    println!("Reading file {}", filename);
    let file = fs::File::open(filename).unwrap();
    let reader = io::BufReader::new(file);
    let mut parsed_lines: Vec<Vec<String>> = Vec::new();
    let mut passport_block: Vec<String> = Vec::new();
    for (_, line) in reader.lines().enumerate() {
        let unwrapped = line.unwrap();
        if unwrapped.is_empty() {
            parsed_lines.push(passport_block.clone());
            passport_block.clear();
        } else {
            passport_block.push(unwrapped);
        }
    }
    parsed_lines.push(passport_block);
    return parsed_lines;
}

#[test]
fn test() {
    let sample_lines = read_file_to_vec(String::from("sample_input.txt"));
    assert_eq!(7, part1(&sample_lines, 66, 3, 1));
}

