use std::fs;

fn main() {
    // Read the file as input
    let input: Vec<String> = fs::read_to_string("input.txt")
    .unwrap()
    .lines()
    .map(String::from)
    .collect();

    let mut ordering_rules: Vec<Vec<i32>> = Vec::new();
    let mut updates: Vec<Vec<i32>> = Vec::new();
    let mut invalid_updates: Vec<Vec<i32>> = Vec::new();
    let mut total = 0;

    // parse the lines into rules and updates
    for line in input {
        if line.contains("|") {
            ordering_rules.push(line.split("|").map(|s| s.parse::<i32>().unwrap()).collect());
        } else if line.contains(",") {
            updates.push(line.split(",").map(|s| s.parse::<i32>().unwrap()).collect());
        }
    }

    // check each update against the rules
    'updates: for update in updates {
        println!("checking {}", stringify(&update));

        // for each rule, check the rule is valid for this update, ie contains both numbers in correct order
        for rule in &ordering_rules {
            if update.contains(&rule[0]) && update.contains(&rule[1]) {
                let earlier = update.iter().position(|&x| x == rule[0]).unwrap();
                let later = update.iter().position(|&x| x == rule[1]).unwrap();
                if earlier > later {
                    println!("not valid, {} comes after {}", earlier, later);
                    invalid_updates.push(update);
                    continue 'updates;
                }
            }
        }

        println!("valid update starting with {} is length {}", update[0], update.len());
        let middle_position = update.len()/2;
        println!("valid update - middle position {} is {}", middle_position, update[middle_position]);
        total += update[middle_position];
    }

    println!("Part 1: {}", total);

    total = 0;

    for mut invalid_update in invalid_updates {
        println!("update starting for {} ", stringify(&invalid_update));

        let mut valid = false;
        while valid == false {
            let mut all_rules_successful = true;
            for rule in &ordering_rules {
                if invalid_update.contains(&rule[0]) && invalid_update.contains(&rule[1]) {
                    let earlier = invalid_update.iter().position(|&x| x == rule[0]).unwrap();
                    let later = invalid_update.iter().position(|&x| x == rule[1]).unwrap();
                    if earlier > later {
                        println!("not valid, {} comes after {} - swapping", earlier, later);
                        invalid_update[earlier] = rule[1];
                        invalid_update[later] = rule[0];
                        all_rules_successful = false;
                    }
                }
            }
            valid = all_rules_successful;
        }
        println!("Now valid: {}", stringify(&invalid_update));

        println!("update starting with {} is length {}", invalid_update[0], invalid_update.len());
        let middle_position = invalid_update.len()/2;
        println!("update - middle position {} is {}", middle_position, invalid_update[middle_position]);
        total += invalid_update[middle_position];
    }

    println!("Part 2: {}", total);
}

fn stringify(numbers: &Vec<i32>) -> String {
    let string_representation: Vec<String> = numbers.iter().map(|x| x.to_string()).collect();
    return string_representation.join(",");
}
