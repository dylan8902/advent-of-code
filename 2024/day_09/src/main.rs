use std::fs;

fn main() {
    // Read the file as rows and columns
    let chars: Vec<char> = fs::read_to_string("input.txt").unwrap().chars().collect();

    // create address space
    let mut address_space = Vec::new();
    let mut id = 0;
    for i in (0..chars.len()).step_by(2) {
        let file = chars[i].to_digit(10).unwrap();
        for _ in 0..file {
            address_space.push(id.to_string());
        }

        if i + 1 >= chars.len() {
            continue;
        }

        let space = chars[i + 1].to_digit(10).unwrap();
        for _ in 0..space {
            address_space.push(".".to_string());
        }
        id += 1;
    }

    let mut total: u64 = 0;
    let mut part_2 = address_space.clone();

    // calculate part 1
    loop {
        match &address_space.iter().position(|x| x == ".") {
            Some(first_free_space) => address_space.swap_remove(*first_free_space),
            None => break,
        };
    }
    for position in 0..address_space.len() {
        total += position as u64 * address_space[position].parse::<u64>().unwrap();
    }

    println!("Part 1: {}", total);

    // work backwards
    let mut end_position = part_2.len() - 1;

    'outer: for position in (0..part_2.len()).rev() {
        let a = part_2[end_position].clone();
        let b = part_2[position].clone();

        if a == b {
            continue;
        }

        if a == "." {
            end_position = position;
            continue;
        }

        let length = end_position - position;

        let mut space_counter: i64 = 0;
        for i in 0..end_position {
            if part_2[i] == "." {
                space_counter += 1;
                if space_counter == length as i64 {
                    for j in 0..length {
                        part_2[i - j] = a.to_string();
                        part_2[end_position - j] = ".".to_string();
                    }
                    end_position = position;
                    continue 'outer;
                }
            } else {
                space_counter = 0;
            }
        }

        end_position = position;
    }

    total = 0;
    for position in 0..part_2.len() {
        match part_2[position].parse::<u64>() {
            Ok(result) => total += position as u64 * result,
            Err(_) => (),
        };
    }

    println!("Part 2: {}", total);
}
