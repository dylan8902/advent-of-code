use std::fs;

fn main() {
    // Read the file as rows and columns
    let chars: Vec<char> = fs::read_to_string("input.txt")
        .unwrap()
        .chars()
        .collect();

    // create address space
    let mut address_space = Vec::new();
    let mut id = 0;
    for i in (0..chars.len()).step_by(2) {
        let file = chars[i].to_digit(10).unwrap();
        for _ in 0..file {
            address_space.push(id.to_string());
        }

        if i+1 >= chars.len() {
            continue;
        }

        let space = chars[i+1].to_digit(10).unwrap();
        for _ in 0..space {
            address_space.push(".".to_string());
        }
        id += 1;
    }

    // calculate part 1
    loop {
        match &address_space.iter().position(|x| x == ".") {
            Some(first_free_space) => address_space.swap_remove(*first_free_space),
            None => break
        };
    }
    let mut total: u64 = 0;
    for position in 0..address_space.len() {
        total += position as u64 * address_space[position].parse::<u64>().unwrap();
    }

    println!("Part 1: {}", total);
}
