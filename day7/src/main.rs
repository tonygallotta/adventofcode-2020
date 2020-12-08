use regex::Regex;
use std::collections::HashMap;
use std::env;
use std::fs;
use std::io;
use std::io::BufRead;
use std::iter::FromIterator;
use std::time::Instant;

#[allow(dead_code)]
#[derive(Eq, PartialEq)]
struct Bag {
    color: String,
    allowed_bags: HashMap<String, u32>,
}

// PART 1: 235
// PART 2: 158493
fn main() {
    let timer = Instant::now();
    let filename = env::args().nth(1).unwrap_or(String::from("input.txt"));
    let lines = read_file_to_vec(filename);
    let bags: HashMap<String, Bag> = to_bags(&lines);
    let (part_1_answer, part_2_answer) = answers(&bags);

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

fn to_bags(lines: &Vec<String>) -> HashMap<String, Bag> {
    let mut bags_by_name: HashMap<String, Bag> = HashMap::new();
    let line_regex = Regex::new(r"(\d+ \w+ \w+)+").unwrap();
    let count_and_type = Regex::new(r"^(?P<count>\d+) (?P<type>\w+ \w+)$").unwrap();
    for line in lines {
        let split: Vec<&str> = Vec::from_iter(line.split(" contain "));
        let (color, contained) = (split[0], split[1]);
        let mut allowed_bags: HashMap<String, u32> = HashMap::new();
        for contained_bag_match in line_regex.find_iter(contained) {
            for m2 in count_and_type.captures_iter(contained_bag_match.as_str()) {
                allowed_bags.insert(
                    String::from(&m2["type"]),
                    *&m2["count"].parse::<u32>().unwrap().clone(),
                );
            }
        }

        let bag = Bag {
            color: String::from(color.replace("bags", "").replace("bag", "").trim()),
            allowed_bags,
        };
        let key = bag.color.clone();
        bags_by_name.insert(key, bag);
    }
    bags_by_name
}

#[allow(unused)]
fn answers(bags_by_name: &HashMap<String, Bag>) -> (u32, u32) {
    let desired_bag = String::from("shiny gold");
    let part_1_answer = count_possible_holders(&desired_bag, bags_by_name);
    let shiny_gold_bag = bags_by_name.get(desired_bag.as_str()).unwrap();

    (
        part_1_answer,
        contained_bag_count(shiny_gold_bag, bags_by_name),
    )
}

fn count_possible_holders(bag_type: &String, bags_by_name: &HashMap<String, Bag>) -> u32 {
    let mut holder_count: u32 = 0;
    for (_, bag) in bags_by_name {
        if can_hold_bag_type(bag_type, bag, bags_by_name) {
            holder_count += 1;
        }
    }
    holder_count
}

fn can_hold_bag_type(
    bag_type: &String,
    bag_to_check: &Bag,
    bags_by_name: &HashMap<String, Bag>,
) -> bool {
    if bag_to_check.allowed_bags.contains_key(bag_type) {
        return true;
    } else if bag_to_check.allowed_bags.is_empty() {
        return false;
    }
    for (contained_bag, _) in &bag_to_check.allowed_bags {
        let next_bag_to_check = bags_by_name.get(contained_bag).unwrap();
        if can_hold_bag_type(bag_type, next_bag_to_check, bags_by_name) {
            return true;
        }
    }
    return false;
}

fn contained_bag_count(bag_to_check: &Bag, bags_by_name: &HashMap<String, Bag>) -> u32 {
    if bag_to_check.allowed_bags.len() == 0 {
        // println!("Base case: {} contains 0 bags", bag_to_check.color);
        return 0;
    }
    let mut answer: u32 = 0;
    for (contained_bag, count) in &bag_to_check.allowed_bags {
        let next_bag_to_check = bags_by_name.get(contained_bag).unwrap();
        answer += count * (1 + contained_bag_count(next_bag_to_check, bags_by_name));
    }
    // println!("{} contains {} bags", bag_to_check.color, answer);
    return answer;
}

#[test]
fn test() {
    let lines = read_file_to_vec(String::from("sample_input.txt"));
    let bags = to_bags(&lines);

    for (_, bag) in bags {
        println!("{} contains {}", bag.color, bag.allowed_bags.len());
    }
}

#[test]
fn test_part2() {
    let lines = read_file_to_vec(String::from("sample_input.txt"));
    let bags = to_bags(&lines);
    let bag_name = String::from("shiny gold");
    let shiny_gold_bag = bags.get(bag_name.as_str()).unwrap();

    assert_eq!(126, contained_bag_count(shiny_gold_bag, &bags));
}

#[test]
fn test_regex() {
    let sample =
        "contain 4 vibrant bronze bags, 5 bright bronze bags, 1 faded red bag, 4 clear tan bags.";
    let re = Regex::new(r"\d+").unwrap();
    let counts: Vec<u32> = re
        .find_iter(sample)
        .map(|m| m.as_str().parse::<u32>().unwrap())
        .collect();
    assert!(counts.contains(&4));
    assert!(counts.contains(&5));
    assert!(counts.contains(&1));
}

#[test]
fn test_regex2() {
    let sample =
        "contain 4 vibrant bronze bags, 5 bright bronze bags, 1 faded red bag, 4 clear tan bags.";
    let line_regex = Regex::new(r"(\d+ \w+ \w+)+").unwrap();
    let count_and_type = Regex::new(r"^(?P<count>\d+) (?P<type>\w+ \w+)$").unwrap();
    for m in line_regex.find_iter(sample) {
        println!("Match: {}", m.as_str());
        for m2 in count_and_type.captures_iter(m.as_str()) {
            println!("Match2: {} / {}", &m2["count"], &m2["type"]);
        }
    }
}
