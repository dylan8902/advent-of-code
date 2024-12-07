use std::fs;

fn main() {
    // Read the file as rows and columns
    let lines: Vec<String> = fs::read_to_string("input.txt")
        .unwrap()
        .lines()
        .map(String::from)
        .collect();

    let mut total = 0;

    'line: for line in lines {
        // parse each equation
        let parts: Vec<&str> = line.split(": ").collect();
        let answer: i64 = parts[0].parse::<i64>().unwrap();
        let numbers: Vec<i64> = parts[1]
            .split(" ")
            .collect::<Vec<&str>>()
            .into_iter()
            .map(|s| s.parse::<i64>().unwrap())
            .collect();
        println!(
            "{} has answer {} made up of {} numbers",
            line,
            answer,
            numbers.len()
        );

        let base: i32 = 2;
        let max = base.pow((numbers.len() - 1) as u32) - 1;

        for bit in 0..=max {
            let binary_representation = String::from(format!("{bit:0width$b}", width=numbers.len() - 1));
            let multiply_positions: Vec<char> = binary_representation.chars().collect();

            let mut sum: i64 = numbers[0];
            for i in 0..multiply_positions.len() {
                if multiply_positions[i] == '1' {
                    sum *= numbers[i+1];
                } else {
                    sum += numbers[i+1];
                }
            }
            if sum == answer {
                total += answer;
                continue 'line;
            }
        }
    }

    println!("Part 1: {}", total);
}
