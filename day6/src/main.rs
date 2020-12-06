use std::env;
use std::fs;
use std::io;
use std::io::BufRead;
use std::time::Instant;
use std::collections::{HashSet, HashMap};

// PART 1: 6310
// PART 2: 3193
fn main() {
    let timer = Instant::now();
    let filename = env::args().nth(1).unwrap_or(String::from("input.txt"));
    let lines = read_file_to_vec(filename);
    let mut part_1_answer = 0;
    let mut part_2_answer = 0;

    let mut yes_answered_questions: HashSet<char> = HashSet::new();
    let mut answer_counts : HashMap<char, u32> = HashMap::new();
    let mut answers_for_block :u32 = 0;
    for line in lines {
        if line.is_empty() {
            // println!("Adding {} yesses", yes_answered_questions.len());
            part_1_answer += yes_answered_questions.len();
            for (_, answer_count) in &answer_counts {
                if answer_count.clone() == answers_for_block.clone() {
                    part_2_answer += 1;
                }
            }
            answer_counts.clear();
            answers_for_block = 0;
            yes_answered_questions.clear();
        } else {
            let yes_answers : Vec<char> = line.chars().collect();
            for question in yes_answers {
                // println!("Adding {}", question);
                yes_answered_questions.insert(question);
                let count: &u32 = answer_counts.get(&question).unwrap_or(&0 );
                answer_counts.insert(question, count + 1);
            }
            answers_for_block += 1;
        }
    }
    part_1_answer += yes_answered_questions.len();
    for (_, answer_count) in &answer_counts {
        if answer_count == &answers_for_block {
            part_2_answer += 1;
        }
    }
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

#[test]
fn test() {

}
