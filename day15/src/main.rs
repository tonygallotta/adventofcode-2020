use std::collections::HashMap;
use std::env;
use std::fs;
use std::io;
use std::io::BufRead;
use std::time::Instant;

// PART 1: 662
// PART 2: 37312
// Execution completed in 30573ms
fn main() {
    let timer = Instant::now();
    let filename = env::args()
        .nth(1)
        .unwrap_or_else(|| String::from("input.txt"));
    let (part_1_answer, part_2_answer) = answers(&vec![16, 11, 15, 0, 1, 7]);

    println!("PART 1: {}", part_1_answer);
    println!("PART 2: {}", part_2_answer);
    println!("Execution completed in {}ms", timer.elapsed().as_millis())
}

fn answers(lines: &Vec<u64>) -> (u64, u64) {
    (
        nth_spoken_number(lines, 2020),
        nth_spoken_number(lines, 30_000_000),
    )
}

fn nth_spoken_number(starting_numbers: &Vec<u64>, n: u64) -> u64 {
    let mut history: HashMap<u64, u64> = HashMap::new();
    let mut last_number_spoken = 0;
    let mut last_time_spoken: Option<u64>;
    for turn in 0..n {
        last_time_spoken = history.get(&last_number_spoken).cloned();
        if turn > 0 {
            history.insert(last_number_spoken.clone(), turn);
        }
        if turn < starting_numbers.len() as u64 {
            last_number_spoken = starting_numbers.get(turn as usize).unwrap().clone();
        } else {
            if last_time_spoken.is_some() {
                last_number_spoken = turn - last_time_spoken.unwrap();
            } else {
                last_number_spoken = 0;
            }
        }
        // println!("{}: {}", turn, last_number_spoken);
    }
    last_number_spoken
}

#[test]
fn test_part1() {
    assert_eq!(nth_spoken_number(&vec![0, 3, 6], 10), 0);
    assert_eq!(nth_spoken_number(&vec![0, 3, 6], 2020), 436);
    assert_eq!(nth_spoken_number(&vec![1, 3, 2], 2020), 1);
}

#[test]
fn test_part2() {
    assert_eq!(nth_spoken_number(&vec![0, 3, 6], 30_000_000), 175594);
    assert_eq!(nth_spoken_number(&vec![1, 3, 2], 30_000_000), 2578);
    assert_eq!(nth_spoken_number(&vec![0, 3, 6], 2020), 436);
}
