use std::fs;

fn main() {
    // Read the file as rows and columns
    let rows: Vec<Vec<char>> = fs::read_to_string("input.txt")
        .unwrap()
        .lines()
        .map(|s| s.chars().collect())
        .collect();

    let mut total = 0;

    for y in 0..rows.len() {
        for x in 0..rows[y].len() {
            if rows[y][x].eq(&'X') {
                println!("Check for xmas");

                // 1. left to right
                if x + 3 < rows[y].len()
                    && rows[y][x + 1].eq(&'M')
                    && rows[y][x + 2].eq(&'A')
                    && rows[y][x + 3].eq(&'S')
                {
                    println!("left to right from {},{}", y, x);
                    total += 1;
                }

                // 2. right to left
                if x >= 3
                    && rows[y][x - 1].eq(&'M')
                    && rows[y][x - 2].eq(&'A')
                    && rows[y][x - 3].eq(&'S')
                {
                    println!("right to left from {},{}", y, x);
                    total += 1;
                }

                // 3. top to bottom
                if y + 3 < rows.len()
                    && rows[y + 1][x].eq(&'M')
                    && rows[y + 2][x].eq(&'A')
                    && rows[y + 3][x].eq(&'S')
                {
                    println!("top to bottom from {},{}", y, x);
                    total += 1;
                }

                // 4. bottom to top
                if y >= 3
                    && rows[y - 1][x].eq(&'M')
                    && rows[y - 2][x].eq(&'A')
                    && rows[y - 3][x].eq(&'S')
                {
                    println!("bottom to top from {},{}", y, x);
                    total += 1;
                }

                // 5. top left to bottom right
                if x + 3 < rows[y].len()
                    && y + 3 < rows.len()
                    && rows[y + 1][x + 1].eq(&'M')
                    && rows[y + 2][x + 2].eq(&'A')
                    && rows[y + 3][x + 3].eq(&'S')
                {
                    println!("top left to bottom right from {},{}", y, x);
                    total += 1;
                }

                // 6. top right to bottom left
                if x >= 3
                    && y + 3 < rows.len()
                    && rows[y + 1][x - 1].eq(&'M')
                    && rows[y + 2][x - 2].eq(&'A')
                    && rows[y + 3][x - 3].eq(&'S')
                {
                    println!("top right to bottom left from {},{}", y, x);
                    total += 1;
                }

                // 7. bottom left to top right
                if x + 3 < rows[y].len()
                    && y >= 3
                    && rows[y - 1][x + 1].eq(&'M')
                    && rows[y - 2][x + 2].eq(&'A')
                    && rows[y - 3][x + 3].eq(&'S')
                {
                    println!("bottom left to top right from {},{}", y, x);
                    total += 1;
                }

                // 8. bottom right to top left
                if x >= 3
                    && y >= 3
                    && rows[y - 1][x - 1].eq(&'M')
                    && rows[y - 2][x - 2].eq(&'A')
                    && rows[y - 3][x - 3].eq(&'S')
                {
                    println!("bottom right to top left from {},{}", y, x);
                    total += 1;
                }
            }
        }
    }

    println!("Part 1: {}", total);

    total = 0;

    for y in 0..rows.len() {
        for x in 0..rows[y].len() {
            if rows[y][x].eq(&'A')
                && y >= 1
                && y + 1 < rows.len()
                && x >= 1
                && x + 1 < rows[y].len()
            {
                println!("Check for m.s");
                println!("          .a.");
                println!("          m.s");
                if rows[y - 1][x - 1].eq(&'M')
                    && rows[y + 1][x - 1].eq(&'M')
                    && rows[y - 1][x + 1].eq(&'S')
                    && rows[y + 1][x + 1].eq(&'S')
                {
                    total += 1;
                }

                println!("Check for s.s");
                println!("          .a.");
                println!("          m.m");
                if rows[y - 1][x - 1].eq(&'S')
                    && rows[y + 1][x - 1].eq(&'M')
                    && rows[y - 1][x + 1].eq(&'S')
                    && rows[y + 1][x + 1].eq(&'M')
                {
                    total += 1;
                }

                println!("Check for s.m");
                println!("          .a.");
                println!("          s.m");
                if rows[y - 1][x - 1].eq(&'S')
                    && rows[y + 1][x - 1].eq(&'S')
                    && rows[y - 1][x + 1].eq(&'M')
                    && rows[y + 1][x + 1].eq(&'M')
                {
                    total += 1;
                }

                println!("Check for m.m");
                println!("          .a.");
                println!("          s.s");
                if rows[y - 1][x - 1].eq(&'M')
                    && rows[y + 1][x - 1].eq(&'S')
                    && rows[y - 1][x + 1].eq(&'M')
                    && rows[y + 1][x + 1].eq(&'S')
                {
                    total += 1;
                }
            }
        }
    }

    println!("Part 2: {}", total);
}
