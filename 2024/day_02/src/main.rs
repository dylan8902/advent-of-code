use std::fs;

fn main() {
     // Read the file as reports
     let reports: Vec<String> = fs::read_to_string("input.txt")
     .unwrap()
     .lines()
     .map(String::from)
     .collect();

    let mut try_again = Vec::new();

    let mut total = 0;

     for report in reports {
        let data: Vec<i32> = report.split_whitespace()
        .map(|s| s.to_string().parse::<i32>().unwrap())
        .collect();

        if check_data_is_safe(data.clone()) {
            total += 1;
        } else {
            try_again.push(data);
        }
    }

    // Print out part 1's answer
    println!("Part 1: {}", total);

    for data in &try_again {
        for index in 0..data.len() {
            let mut attempt = data.clone();
            attempt.remove(index);
            if check_data_is_safe(attempt) {
                total += 1;
                break;
            }
        }
    }

    // Print out part 2's answer
    println!("Part 2: {}", total);
}

fn check_data_is_safe(data: Vec<i32>) -> bool {
    let mut direction = 0;
    let mut count = 0;

    for i in 1..data.len() {
        let diff = data[i-1] - data[i];

        if i == 1 {
            if diff < 0 {
                // set decreasing
                direction = -1;
            }
            else if diff > 0 {
                // set increasing
                direction = 1;
            }
            else {
                // unsafe
                break;
            }
        }

        if diff <= 0 && direction != -1 {
            // unsafe
            break;
        }
        else if diff >= 0 && direction != 1 {
            // unsafe
            break;
        }

        if diff < -3 || diff > 3 {
            // unsafe
            break;
        }

        count +=1;
    }

    return count == data.len() - 1;
}
