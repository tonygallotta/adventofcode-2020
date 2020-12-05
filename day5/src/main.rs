use std::env;
use std::fs;
use std::io;
use std::io::BufRead;
use std::time::Instant;

#[derive(Copy, Clone)]
struct Seat {
    row: u8, column: u8, id: u32
}

fn main() {
    let timer = Instant::now();
    let filename = env::args().nth(1).unwrap_or(String::from("input.txt"));
    let lines = read_file_to_vec(filename);
    let mut part_1_answer = 0;
    let mut part_2_answer = 0;
    let mut seats :Vec<Seat> = Vec::new();
    for line in lines {
        seats.push(get_seat(line.as_str()));
    }

    for seat in &seats {
        if seat.id > part_1_answer {
            part_1_answer = seat.id
        }
    }
    let mut sorted_seat_ids : Vec<u32> = seats.iter().map(|s| s.id).collect();
    sorted_seat_ids.sort();
    let mut previous_seat_id = sorted_seat_ids[0];
    for seat_id in sorted_seat_ids {
        if seat_id - previous_seat_id == 2 {
            println!("Found the missing seat ID between {} and {}", previous_seat_id, seat_id);
            part_2_answer = seat_id - 1;
        }
        previous_seat_id = seat_id;
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

fn get_seat(encoded: &str) -> Seat {
    let row_raw : String = encoded.replace("F", "0").replace("B", "1").chars().take(7).collect();
    let row = u8::from_str_radix(&row_raw, 2).unwrap();

    let column_raw : String = encoded.replace("L", "0").replace("R", "1").chars().skip(7).take(3).collect();
    let column = u8::from_str_radix(&column_raw, 2).unwrap();
    Seat{row: row, column: column, id: row as u32 * 8 + column as u32 }
}

#[test]
fn test_get_seat() {
    let seat = get_seat("BFFFBBFRRR");
    assert_eq!(70, seat.row);
    assert_eq!(7, seat.column);
    assert_eq!(567, seat.id);

    let seat2 = get_seat("FFFBBBFRRR");
    assert_eq!(14, seat2.row);
    assert_eq!(7, seat2.column);
    assert_eq!(119, seat2.id);

    let seat3 = get_seat("BBFFBBFRLL");
    assert_eq!(102, seat3.row);
    assert_eq!(4, seat3.column);
    assert_eq!(820, seat3.id);
}