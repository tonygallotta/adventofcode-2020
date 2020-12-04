use std::env;
use std::fs;
use std::io;
use std::io::BufRead;
use std::collections::{HashSet, HashMap};
use std::iter::FromIterator;
use regex::Regex;

struct Passport {
    field_map: HashMap<String, String>
}

impl Passport {
    fn from(lines: &Vec<String>) -> Passport {
        let mut field_map = HashMap::new();
        for line in lines.iter() {
            let key_value_pairs = Vec::from_iter(line.split(" "));
            for (_, key_value_pair) in key_value_pairs.iter().enumerate() {
                let split_kvp: Vec<&str> = Vec::from_iter(key_value_pair.split(":"));
                let key = split_kvp[0];
                let value = split_kvp[1];
                field_map.insert(String::from(key), String::from(value));
            }
        }
        Passport { field_map }
    }

    fn has_all_required_fields(&self) -> bool {
        // Not sure how to make this more of a constant
        let required_fields: HashSet<&str> = vec!("byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid").into_iter().collect();
        let field_map = &self.field_map;
        let present_keys: HashSet<&str> = field_map.keys().map(String::as_str).collect();
        required_fields.is_subset(&present_keys)
    }

    fn present_fields_valid(&self) -> bool {
        let hcl_regex = Regex::new(r"^#[0-9a-f]{6}$").unwrap();
        let pid_regex = Regex::new(r"^[0-9]{9}$").unwrap();
        let valid_ecls : HashSet<&str> = vec!("amb", "blu", "brn", "gry", "grn", "hzl", "oth").into_iter().collect();
        let field_map = &self.field_map;
        let mut is_valid = true;

        for (key, value) in field_map {
            is_valid = is_valid && match key.as_str() {
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
                    let mut result = false;
                    if units == 'i' {
                        result = numeric_value >= 59 && numeric_value <= 76;
                    } else if units == 'c' {
                        result = numeric_value >= 150 && numeric_value <= 193;
                    }
                    result
                },
                "hcl" => hcl_regex.is_match(value),
                "ecl" => valid_ecls.contains(value.as_str()),
                "pid" => pid_regex.is_match(value),
                _ => true
            }
        }
        is_valid
    }
}

fn main() {
    let filename = env::args().nth(1).unwrap_or(String::from("input.txt"));
    let passports = read_file_to_vec(filename);
    let mut part_1_answer = 0;
    let mut part_2_answer = 0;

    for passport in passports {
        if passport.has_all_required_fields() {
            part_1_answer += 1;
            if passport.present_fields_valid() {
                part_2_answer += 1
            }
        }
    }

    println!("PART 1: {}", part_1_answer);
    println!("PART 2: {}", part_2_answer);
}

fn read_file_to_vec(filename: String) -> Vec<Passport> {
    println!("Reading file {}", filename);
    let file = fs::File::open(filename).unwrap();
    let reader = io::BufReader::new(file);
    let mut parsed_lines: Vec<Passport> = Vec::new();
    let mut passport_block: Vec<String> = Vec::new();
    for (_, line) in reader.lines().enumerate() {
        let unwrapped = line.unwrap();
        if unwrapped.is_empty() {
            parsed_lines.push(Passport::from(&passport_block));
            passport_block.clear();
        } else {
            passport_block.push(unwrapped);
        }
    }
    parsed_lines.push(Passport::from(&passport_block));
    return parsed_lines;
}

#[test]
fn test_part1() {
    let passports = read_file_to_vec(String::from("sample_input_1.txt"));
    assert_eq!(2, passports.iter().filter(|&p| p.has_all_required_fields()).count());
}

#[test]
fn test_part2() {
    let invalid_passports = read_file_to_vec(String::from("sample_input_2.txt"));
    assert_eq!(0, invalid_passports.iter().filter(|&p| p.has_all_required_fields() && p.present_fields_valid()).count());

    let valid_passports = read_file_to_vec(String::from("sample_input_3.txt"));
    assert_eq!(4, valid_passports.iter().filter(|&p| p.has_all_required_fields() && p.present_fields_valid()).count());
}

