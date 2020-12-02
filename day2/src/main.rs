use std::env;
use std::fs;
use std::io;
use std::io::BufRead;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    println!("Reading file {}", filename);
    let file = fs::File::open(filename).unwrap();
    let reader = io::BufReader::new(file);
    let mut valid_passwords = 0;
    for (_, line) in reader.lines().enumerate() {
        if is_valid_password_part2(&line.unwrap()) {
            valid_passwords += 1;
        }
    }
    println!("{} valid passwords", valid_passwords);
}

fn is_valid_password(line: &str) -> bool {
    let parts: Vec<&str> = line.split(" ").collect();
    let min_max_repetitions: Vec<&str> = parts[0].split("-").collect();
    let min_repetitions: u32 = min_max_repetitions[0].parse().unwrap();
    let max_repetitions: u32 = min_max_repetitions[1].parse().unwrap();
    let required_char = parts[1].chars().nth(0).unwrap();
    let password = parts[2];
    println!("{} is required {} to {} times", required_char, min_repetitions, max_repetitions);
    let mut occurrences = 0;
    for c in password.chars() {
        if c == required_char {
            occurrences += 1;
        }
    }
    println!("Found {} occurrences in {}", occurrences, password);
    return occurrences >= min_repetitions && occurrences <= max_repetitions;
}

fn is_valid_password_part2(line: &str) -> bool {
    let parts: Vec<&str> = line.split(" ").collect();
    let possible_positions: Vec<&str> = parts[0].split("-").collect();
    let first_position: usize = possible_positions[0].parse().unwrap();
    let second_position: usize = possible_positions[1].parse().unwrap();
    let required_char = parts[1].chars().nth(0).unwrap();
    let password = parts[2];
    println!("{} is required at {} or {}", required_char, first_position, second_position);
    let mut occurrences = 0;
    for c in password.chars() {
        if c == required_char {
            occurrences += 1;
        }
    }
    println!("Found {} occurrences in {}", occurrences, password);
    return (password.chars().nth(first_position - 1).unwrap_or('_') == required_char) ^ (password.chars().nth(second_position - 1).unwrap_or('_') == required_char);
}

#[test]
fn test() {
    assert!(is_valid_password("1-3 a: abcde"));
    assert_eq!(is_valid_password("1-3 b: cdefg"), false);
}

#[test]
fn test_part2() {
    assert!(is_valid_password_part2("1-3 a: abcde"));
    assert_eq!(is_valid_password_part2("1-3 b: cdefg"), false);
    assert_eq!(is_valid_password_part2("2-9 c: ccccccccc"), false);
}
