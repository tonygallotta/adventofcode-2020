use regex::Regex;
use std::collections::HashSet;
use std::fs;
use std::io;
use std::io::BufRead;
use std::ops::Range;
use std::time::Instant;

#[allow(dead_code)]
struct InputData {
    rules: Vec<String>,
    my_ticket: String,
    nearby_tickets: Vec<String>,
}

struct Rule {
    field_name: String,
    low_range: Range<u64>,
    high_range: Range<u64>,
}

impl Rule {
    fn is_valid(&self, value: &u64) -> bool {
        self.low_range.contains(value) || self.high_range.contains(value)
    }
}

// PART 1: 29759
// PART 2: 1307550234719
// Execution completed in 189ms
fn main() {
    let timer = Instant::now();
    let input = read_file_to_input(String::from("input.txt"));
    let part_1_answer = nearby_error_rate(&input);

    println!("PART 1: {}", part_1_answer);
    println!("PART 2: {}", part_2(&input));
    println!("Execution completed in {}ms", timer.elapsed().as_millis())
}

fn read_file_to_input(filename: String) -> InputData {
    println!("Reading file {}", filename);
    let file = fs::File::open(filename).unwrap();
    let reader = io::BufReader::new(file);
    let mut parsed_lines = Vec::new();
    let mut my_ticket = Vec::new();
    let mut rules = Vec::new();
    let mut line_breaks_hit = 0;
    for (_, line) in reader.lines().enumerate() {
        let unwrapped_line = line.unwrap();
        if unwrapped_line.is_empty() {
            match line_breaks_hit {
                0 => rules = parsed_lines.clone(),
                1 => my_ticket = parsed_lines.clone(),
                _ => {}
            }
            parsed_lines.clear();
            line_breaks_hit += 1;
        } else if !unwrapped_line.starts_with("your") && !unwrapped_line.starts_with("nearby") {
            parsed_lines.push(unwrapped_line);
        }
    }
    InputData {
        rules,
        my_ticket: my_ticket.get(0).unwrap().clone(),
        nearby_tickets: parsed_lines.clone(),
    }
}

fn nearby_error_rate(input_data: &InputData) -> u64 {
    let rules = parse_rules(&input_data.rules);
    let mut error_rate = 0;
    for ticket in &input_data.nearby_tickets {
        let field_values: Vec<u64> = ticket
            .split(",")
            .map(|v| v.parse::<u64>().unwrap())
            .collect();
        for value in field_values {
            if !rules.iter().any(|rule| rule.is_valid(&value)) {
                error_rate += value;
            }
        }
    }
    error_rate
}

fn part_2(input_data: &InputData) -> u64 {
    let ordered_fields = get_ordered_fields(&input_data);
    let my_ticket_fields: Vec<u64> = input_data
        .my_ticket
        .split(",")
        .map(|v| v.parse().unwrap())
        .collect();
    let mut ans = 1;
    for (i, field) in ordered_fields.iter().enumerate() {
        if field.starts_with("departure") {
            ans *= my_ticket_fields.get(i).unwrap();
        }
    }
    ans
}

fn get_ordered_fields(input_data: &InputData) -> Vec<String> {
    let rules = parse_rules(&input_data.rules);
    let valid_tickets = get_valid_tickets(&input_data.nearby_tickets, &rules);
    let field_count = rules.len();
    let mut field_possible_values: Vec<HashSet<String>> = Vec::new();
    let mut ordered_fields = vec![String::new(); field_count];

    for _ in 0..field_count {
        let possible_values: HashSet<String> = rules.iter().map(|r| r.field_name.clone()).collect();
        field_possible_values.push(possible_values);
    }

    for ticket in valid_tickets {
        let field_values: Vec<u64> = ticket
            .split(",")
            .map(|v| v.parse::<u64>().unwrap())
            .collect();
        for (idx, value) in field_values.iter().enumerate() {
            if rules.iter().any(|rule| rule.is_valid(&value)) {
                for rule in &rules {
                    if !rule.is_valid(&value) {
                        let mut new_possible_values =
                            field_possible_values.get(idx).unwrap().clone();
                        new_possible_values.remove(&rule.field_name);
                        field_possible_values.remove(idx);
                        field_possible_values.insert(idx, new_possible_values)
                    }
                }
            }
        }
    }
    let mut all_matched = false;
    while !all_matched {
        for (idx, field_names) in field_possible_values.iter_mut().enumerate() {
            let matched_values: HashSet<String> = ordered_fields.iter().cloned().collect();
            let unmatched: HashSet<String> = field_names
                .difference(&matched_values)
                .map(|d| d.clone())
                .collect();
            if unmatched.len() == 1 {
                ordered_fields.remove(idx);
                let value = unmatched.iter().next().unwrap().clone();
                ordered_fields.insert(idx, value.clone());
            }
        }
        all_matched = ordered_fields.iter().filter(|v| !v.is_empty()).count() == field_count;
    }
    ordered_fields
}

fn get_valid_tickets(all_tickets: &Vec<String>, rules: &Vec<Rule>) -> Vec<String> {
    let mut valid_tickets = Vec::new();
    for ticket in all_tickets {
        let field_values: Vec<u64> = ticket
            .split(",")
            .map(|v| v.parse::<u64>().unwrap())
            .collect();
        for value in field_values {
            if rules.iter().any(|rule| rule.is_valid(&value)) {
                valid_tickets.push(ticket.clone());
            }
        }
    }
    valid_tickets
}

fn parse_rules(raw_rules: &Vec<String>) -> Vec<Rule> {
    let rule_re = Regex::new(r"^(?P<field>.*): (?P<low_range_lb>\d+)-(?P<low_range_ub>\d+) or (?P<high_range_lb>\d+)-(?P<high_range_ub>\d+)$").unwrap();
    let mut rules = Vec::new();
    for raw_rule in raw_rules {
        let captures = rule_re.captures(raw_rule.as_str()).unwrap();
        rules.push(Rule {
            field_name: String::from(captures.name("field").unwrap().as_str()),
            low_range: to_range(
                captures.name("low_range_lb").unwrap().as_str(),
                captures.name("low_range_ub").unwrap().as_str(),
            ),
            high_range: to_range(
                captures.name("high_range_lb").unwrap().as_str(),
                captures.name("high_range_ub").unwrap().as_str(),
            ),
        });
    }
    rules
}

fn to_range(lb: &str, ub: &str) -> Range<u64> {
    Range {
        start: lb.parse().unwrap(),
        end: ub.parse::<u64>().unwrap() + 1,
    }
}

#[test]
fn test_part1() {
    let input = read_file_to_input(String::from("sample_input.txt"));
    assert_eq!(input.rules.len(), 3);
    assert_eq!(input.nearby_tickets.len(), 4);
    assert_eq!(nearby_error_rate(&input), 71);
}

#[test]
fn test_part2() {
    let input = read_file_to_input(String::from("sample_input_2.txt"));
    let ordered_fields = get_ordered_fields(&input);
    assert_eq!(ordered_fields.len(), 3);
    assert_eq!(ordered_fields.get(0).unwrap(), "row");
    assert_eq!(ordered_fields.get(1).unwrap(), "class");
    assert_eq!(ordered_fields.get(2).unwrap(), "seat");
}
