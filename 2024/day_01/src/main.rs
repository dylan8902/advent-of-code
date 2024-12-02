use std::fs;

fn main() {
    // Read the file as rows
    let rows: Vec<String> = fs::read_to_string("input.txt")
    .unwrap()
    .lines()
    .map(String::from)
    .collect();

    // Parse the row into 2 variables
    let mut left = Vec::new();
    let mut right = Vec::new();

    for row in rows {
        let parts: Vec<i32> = row.split_whitespace()
        .map(|s| s.to_string().parse::<i32>().unwrap())
        .collect();
        left.push(parts[0].clone());
        right.push(parts[1].clone());
    }

    // Sort both
    left.sort();
    right.sort();

    // Total up the distance between them
    let mut total = 0;
    for i in 0..left.len() {
        let mut diff = left[i] - right[i];
        if diff < 0 {
            diff = diff * -1
        }
        total += diff;
    }

    // Print out part 1's answer
    println!("Part 1: {}", total);

    total = 0;
    for i in left {
        let count = right.iter().filter(|&n| *n == i).count() as i32;
        total += i * count;
    }

    // Print out part 2's answer
    println!("Part 2: {}", total);
}
