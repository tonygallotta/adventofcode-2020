use std::collections::HashSet;
use std::env;
use std::fs;
use std::io;
use std::io::BufRead;
use std::time::Instant;

struct AnswerGroup(Vec<HashSet<char>>);

impl AnswerGroup {
    fn from(lines: Vec<&String>) -> AnswerGroup {
        let mut answers: Vec<HashSet<char>> = Vec::new();
        for line in lines {
            let line_answers: HashSet<char> = line.chars().collect();
            answers.push(line_answers);
        }
        AnswerGroup(answers)
    }

    fn distinct_answer_count(&self) -> u32 {
        let mut distinct_answers: HashSet<char> = HashSet::new();
        for answers in &self.0 {
            distinct_answers.extend(answers);
        }
        distinct_answers.len() as u32
    }

    fn common_answer_count(&self) -> u32 {
        let mut common_answers: HashSet<char> = HashSet::new();
        for (i, answers) in self.0.iter().enumerate() {
            if i == 0 {
                common_answers.extend(answers);
            } else {
                common_answers = common_answers.intersection(answers).cloned().collect();
            }
        }
        common_answers.len() as u32
    }
}

// PART 1: 6310
// PART 2: 3193
fn main() {
    let timer = Instant::now();
    let filename = env::args().nth(1).unwrap_or(String::from("input.txt"));
    let lines = read_file_to_vec(filename);
    let answer_groups: Vec<AnswerGroup> = to_answer_groups(&lines);
    let (part_1_answer, part_2_answer) = answers(&answer_groups);

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

fn to_answer_groups(lines: &Vec<String>) -> Vec<AnswerGroup> {
    let mut results = Vec::new();
    let mut answer_group_lines: Vec<&String> = Vec::new();
    for line in lines {
        if line.is_empty() {
            results.push(AnswerGroup::from(answer_group_lines.clone()));
            answer_group_lines.clear();
        } else {
            answer_group_lines.push(line);
        }
    }
    results.push(AnswerGroup::from(answer_group_lines.clone()));
    results
}

fn answers(answer_groups: &Vec<AnswerGroup>) -> (u32, u32) {
    let part_1_answer: u32 = answer_groups
        .iter()
        .map(|a| a.distinct_answer_count())
        .sum();
    let part_2_answer: u32 = answer_groups.iter().map(|a| a.common_answer_count()).sum();
    (part_1_answer, part_2_answer)
}

#[test]
fn test() {
    let answer_groups = to_answer_groups(&read_file_to_vec(String::from("sample_input_1.txt")));
    let answers = answers(&answer_groups);
    assert_eq!(11, answers.0);
    assert_eq!(6, answers.1);
}
