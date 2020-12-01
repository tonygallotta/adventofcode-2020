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
    let mut expenses = Vec::new();
    for (_, line) in reader.lines().enumerate() {
        let expense: u32 = line.unwrap().parse().unwrap(); // Ignore errors.
        expenses.push(expense);
    }
    let summing_pair = sum_to_2020_pair(&expenses);
    println!("The pair is {}, {}", summing_pair.0, summing_pair.1);
    println!("Product is {}", summing_pair.0 * summing_pair.1);


    let summing_triplet = sum_to_2020_triplet(&expenses);
    println!("The triplet is {}, {}, {}", summing_triplet.0, summing_triplet.1, summing_triplet.2);
    println!("Product is {}", summing_triplet.0 * summing_triplet.1 * summing_triplet.2);
}

fn sum_to_2020_pair(expenses: &Vec<u32>) -> (u32, u32) {
    for (i, e1) in expenses.iter().enumerate() {
        for e2 in expenses.iter().skip(i) {
            if (e1 + e2) == 2020 {
                println!("{} + {} = 2020", e1, e2);
                return (*e1, *e2);
            }
        }
    }
    return (0, 0);
}


fn sum_to_2020_triplet(expenses: &Vec<u32>) -> (u32, u32, u32) {
    for (i, e1) in expenses.iter().enumerate() {
        for (j, e2) in expenses.iter().skip(i).enumerate() {
            for e3 in expenses.iter().skip(j) {
                if (e1 + e2 + e3) == 2020 {
                    println!("{} + {} + {} = 2020", e1, e2, e3);
                    return (*e1, *e2, *e3);
                }
            }
        }
    }
    return (0, 0, 0);
}

#[test]
fn test() {
    let sample_data = vec![1721, 979, 366, 299, 675, 1456];
    assert_eq!((1721, 299), sum_to_2020_pair(&sample_data));
}
