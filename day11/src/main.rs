use std::env;
use std::fs;
use std::io;
use std::io::BufRead;
use std::time::Instant;

// PART 1: 2448
// PART 2: 2234
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

fn read_file_to_vec(filename: String) -> Vec<Vec<char>> {
    println!("Reading file {}", filename);
    let file = fs::File::open(filename).unwrap();
    let reader = io::BufReader::new(file);
    let mut parsed_lines: Vec<Vec<char>> = Vec::new();
    for (_, line) in reader.lines().enumerate() {
        parsed_lines.push(line.unwrap().chars().collect());
    }
    parsed_lines
}

fn answers(lines: &Vec<Vec<char>>) -> (u64, u64) {
    (run_part1(lines), run_part2(&lines))
}

fn run_part1(seats: &Vec<Vec<char>>) -> u64 {
    let mut last_round_result = apply_round1(&seats);
    let mut occupied_seats = 0;
    let mut iterations = 0;
    let mut changed = true;
    while changed {
        let current_result = apply_round2(&last_round_result);
        let current_occupied_seats = count_occupied(&current_result);
        iterations += 1;
        last_round_result = current_result;
        changed = occupied_seats != current_occupied_seats;
        occupied_seats = current_occupied_seats;
    }
    println!("Completed after {} iterations", iterations);
    occupied_seats
}

fn run_part2(seats: &Vec<Vec<char>>) -> u64 {
    let mut last_round_result = apply_round1(&seats);
    let mut occupied_seats = 0;
    let mut iterations = 0;
    let mut changed = true;
    while changed {
        let current_result = apply_round2_part2(&last_round_result);
        let current_occupied_seats = count_occupied(&current_result);
        iterations += 1;
        last_round_result = current_result;
        changed = occupied_seats != current_occupied_seats;
        occupied_seats = current_occupied_seats;
    }
    println!("Completed after {} iterations", iterations);
    occupied_seats
}

fn count_occupied(seats: &Vec<Vec<char>>) -> u64 {
    let mut occupied_count = 0;
    for (_, row) in seats.iter().enumerate() {
        for (_, status) in row.iter().enumerate() {
            if *status == '#' {
                occupied_count += 1;
            }
        }
    }
    occupied_count
}

fn apply_round1(seats: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut result = Vec::new();
    for (i, row) in seats.iter().enumerate() {
        for (j, status) in row.iter().enumerate() {
            let result_row;
            if result.get(i).is_none() {
                result.insert(i, Vec::new());
            }
            result_row = result.get_mut(i).unwrap();
            if *status == 'L' {
                result_row.insert(j, '#');
            } else {
                result_row.insert(j, *status);
            }
        }
    }
    result
}

fn apply_round2(seats: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut result = Vec::new();
    for (i, row) in seats.iter().enumerate() {
        for (j, status) in row.iter().enumerate() {
            let result_row;
            if result.get(i).is_none() {
                result.insert(i, Vec::new());
            }
            result_row = result.get_mut(i).unwrap();
            let adjacent_occupied_seats = adjacent_occupied_count(i, j, seats);
            if *status == '#' && adjacent_occupied_seats >= 4 {
                result_row.insert(j, 'L');
            } else if *status == 'L' && adjacent_occupied_seats == 0 {
                result_row.insert(j, '#');
            } else {
                result_row.insert(j, *status);
            }
        }
    }
    result
}

fn apply_round2_part2(seats: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut result = Vec::new();
    for (i, row) in seats.iter().enumerate() {
        for (j, status) in row.iter().enumerate() {
            let result_row;
            if result.get(i).is_none() {
                result.insert(i, Vec::new());
            }
            result_row = result.get_mut(i).unwrap();
            let visible_occupied_seats = visible_occupied_count(i, j, seats);
            if *status == '#' && visible_occupied_seats >= 5 {
                result_row.insert(j, 'L');
            } else if *status == 'L' && visible_occupied_seats == 0 {
                result_row.insert(j, '#');
            } else {
                result_row.insert(j, *status);
            }
        }
    }
    result
}

fn adjacent_occupied_count(i: usize, j: usize, seats: &Vec<Vec<char>>) -> usize {
    let occupied_count = is_occupied(i as isize - 1, j as isize, seats)
        + is_occupied(i as isize + 1, j as isize, seats)
        + is_occupied(i as isize, j as isize + 1, seats)
        + is_occupied(i as isize - 1, j as isize + 1, seats)
        + is_occupied(i as isize + 1, j as isize + 1, seats)
        + is_occupied(i as isize, j as isize - 1, seats)
        + is_occupied(i as isize - 1, j as isize - 1, seats)
        + is_occupied(i as isize + 1, j as isize - 1, seats);
    // println!("{} adjacent to {}, {} are occupied", occupied_count, i, j);
    occupied_count
}

fn is_occupied(i: isize, j: isize, seats: &Vec<Vec<char>>) -> usize {
    has_status(i, j, '#', seats)
}

fn has_status(i: isize, j: isize, status: char, seats: &Vec<Vec<char>>) -> usize {
    let empty_vec = Vec::new();
    if i < 0 || j < 0 {
        return 0;
    }
    (seats
        .get(i as usize)
        .unwrap_or(&empty_vec)
        .get(j as usize)
        .cloned()
        .unwrap_or('_')
        == status) as usize
}

fn visible_occupied_count(i: usize, j: usize, seats: &Vec<Vec<char>>) -> usize {
    let occupied_count = has_visible_seat_occupied(i, j, 0, 1, seats)
        + has_visible_seat_occupied(i, j, 1, 0, seats)
        + has_visible_seat_occupied(i, j, 1, 1, seats)
        + has_visible_seat_occupied(i, j, 0, -1, seats)
        + has_visible_seat_occupied(i, j, -1, 0, seats)
        + has_visible_seat_occupied(i, j, -1, -1, seats)
        + has_visible_seat_occupied(i, j, 1, -1, seats)
        + has_visible_seat_occupied(i, j, -1, 1, seats);
    occupied_count
}

fn has_visible_seat_occupied(
    i: usize,
    j: usize,
    xslope: i32,
    yslope: i32,
    seats: &Vec<Vec<char>>,
) -> usize {
    let mut x = add(i, xslope);
    let mut y = add(j, yslope);
    while x.is_some() && y.is_some() {
        let x_value = x.unwrap() as isize;
        let y_value = y.unwrap() as isize;
        if x.unwrap() >= seats.len() || y.unwrap() >= seats.get(x.unwrap()).unwrap().len() {
            return 0;
        }
        if is_occupied(x_value, y_value, seats) == 1 {
            return 1;
        } else if has_status(x_value, y_value, 'L', seats) == 1 {
            return 0;
        }
        x = add(x.unwrap(), xslope);
        y = add(y.unwrap(), yslope);
    }
    0
}

fn add(u: usize, i: i32) -> Option<usize> {
    if i.is_negative() {
        u.checked_sub(i.wrapping_abs() as u32 as usize)
    } else {
        u.checked_add(i as usize)
    }
}

#[test]
fn test_part1() {
    let lines = read_file_to_vec(String::from("sample_input.txt"));
    assert_eq!(run_part1(&lines), 37);
}

#[test]
fn test_part2() {
    let lines = read_file_to_vec(String::from("sample_input.txt"));
    assert_eq!(run_part2(&lines), 26);
}

#[test]
fn test_visible_occupied_count() {
    let example2 = read_file_to_vec(String::from("sample_input_2.txt"));
    assert_eq!(visible_occupied_count(4, 3, &example2), 8);
    let example3 = read_file_to_vec(String::from("sample_input_3.txt"));
    assert_eq!(visible_occupied_count(1, 1, &example3), 0);
    let example4 = read_file_to_vec(String::from("sample_input_4.txt"));
    assert_eq!(visible_occupied_count(3, 3, &example4), 0);
}

#[test]
fn test_add() {
    assert!(add(0, -1).is_none());
}
