use std::env;
use std::fs;
use std::io;
use std::io::BufRead;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    println!("In file {}", filename);
    let file = fs::File::open(filename).unwrap();
    let reader = io::BufReader::new(file);
    let mut total_fuel_required = 0;
    for (_, line) in reader.lines().enumerate() {
        let mass:u32 = line.unwrap().parse().unwrap(); // Ignore errors.
        total_fuel_required += total_fuel_requirement(mass);
    }
    println!("Total fuel requirement is {}", total_fuel_required);
}

fn total_fuel_requirement(mass: u32) -> u32 {
    if mass < 9 {
        return 0
    }
    let current_fuel_requirement = fuel_requirement(mass);
    return current_fuel_requirement + total_fuel_requirement(current_fuel_requirement)
}

fn fuel_requirement(mass: u32) -> u32 {
    return mass / 3 - 2
}

#[test]
fn test_fuel_requirement() {
    assert_eq!(fuel_requirement(12), 2);
    assert_eq!(fuel_requirement(14), 2);
    assert_eq!(fuel_requirement(1969), 654);
    assert_eq!(fuel_requirement(100756), 33583);
}

#[test]
fn test_total_fuel_requirement() {
    assert_eq!(total_fuel_requirement(12), 2);
    assert_eq!(total_fuel_requirement(1969), 966);
    assert_eq!(total_fuel_requirement(100756), 50346);
}
