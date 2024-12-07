use radix_fmt::radix_3;
use std::fs;

fn main() {
    // Read the file as rows and columns
    let lines: Vec<String> = fs::read_to_string("input.txt")
        .unwrap()
        .lines()
        .map(String::from)
        .collect();

    let mut part_1 = 0;
    let mut part_2 = 0;

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

        // part 1
        let mut base: i32 = 2;
        let mut max = base.pow((numbers.len() - 1) as u32) - 1;

        for bit in 0..=max {
            let binary_representation =
                String::from(format!("{bit:0width$b}", width = numbers.len() - 1));
            let positions: Vec<char> = binary_representation.chars().collect();

            let mut sum: i64 = numbers[0];
            for i in 0..positions.len() {
                if positions[i] == '1' {
                    sum *= numbers[i + 1];
                } else {
                    sum += numbers[i + 1];
                }
            }
            if sum == answer {
                part_1 += answer;
                continue 'line;
            }
        }

        // part 2
        base = 3;
        max = base.pow((numbers.len() - 1) as u32) - 1;

        for bit in 0..=max {
            let trinary_representation = String::from(format!(
                "{:0width$}",
                radix_3(bit).to_string().parse::<i64>().unwrap(),
                width = numbers.len() - 1
            ));
            let positions: Vec<char> = trinary_representation.chars().collect();

            let mut sum: i64 = numbers[0];
            for i in 0..positions.len() {
                if positions[i] == '2' {
                    sum = format!("{}{}", sum, numbers[i + 1]).parse::<i64>().unwrap();
                } else if positions[i] == '1' {
                    sum *= numbers[i + 1];
                } else {
                    sum += numbers[i + 1];
                }
            }
            if sum == answer {
                part_2 += answer;
                continue 'line;
            }
        }
    }

    println!("Part 1: {}", part_1);
    println!("Part 2: {}", part_1 + part_2);
}
