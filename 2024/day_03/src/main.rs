use regex::Regex;
use std::fs;

fn main() {
    // Read the file as input
    let input: String = fs::read_to_string("input.txt").unwrap();

    // find x and y by 1 to 3 digits
    let part1 = Regex::new(r"mul\((?<x>\d{1,3}),(?<y>\d{1,3})\)").unwrap();

    // sum the multiplication up
    let mut total = 0;
    for find in part1.captures_iter(&input) {
        total += &find["x"].parse::<i32>().unwrap() * &find["y"].parse::<i32>().unwrap();
    }

    println!("Part 1: {}", total);

    // reset
    total = 0;

    // find the pattern and do's and don'ts
    let part2 = Regex::new(r"(mul\((?<x>\d{1,3}),(?<y>\d{1,3})\))|(do\(\))|(don't\(\))").unwrap();

    // default to on
    let mut enabled = true;

    for find in part2.captures_iter(&input) {
        // turn it on
        if find[0].eq("do()") {
            enabled = true;
            continue;
        }
        // turn it off
        if find[0].eq("don't()") {
            enabled = false;
            continue;
        }
        // sum the multiplication up if we are enabled
        if enabled {
            total += &find["x"].parse::<i32>().unwrap() * &find["y"].parse::<i32>().unwrap();
        }
    }

    println!("Part 2: {}", total);
}
