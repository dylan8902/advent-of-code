use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;
use itertools::Itertools;

#[derive(Clone, Copy, Hash, Eq, PartialEq)]
struct Coord {
    pub x: i32,
    pub y: i32,
}

impl Coord {
    fn in_bounds(&self, bounds: &Vec<i32>) -> bool {
        self.x >= 0 && self.x < bounds[0] && self.y >= 0 && self.y < bounds[1]
    }
}

fn main() {
    // Read the file as rows and columns
    let rows: Vec<Vec<char>> = fs::read_to_string("input.txt")
        .unwrap()
        .lines()
        .map(|s| s.chars().collect())
        .collect();

    let mut antennas: HashMap<char, Vec<Coord>> = HashMap::new();

    // Parse input
    for y in 0..rows.len() {
        for x in 0..rows[y].len() {
            if rows[y][x].eq(&'.') {
                continue;
            }

            let coord = Coord {
                x: x as i32,
                y: y as i32,
            };

            antennas
                .entry(rows[y][x])
                .and_modify(|positions| positions.push(coord))
                .or_insert(vec![coord]);
        }
    }

    // set the bounds of the map
    let bounds: Vec<i32> = vec![rows.len() as i32, rows[0].len() as i32];

    // store unique antinodes
    let mut antinodes: HashSet<Coord> = HashSet::new();

    // part 1
    for (_, positions) in antennas.iter() {
        for permitation in positions.into_iter().permutations(2) {
            let a = permitation[0];
            let b = permitation[1];

            let x_dist = a.x - b.x;
            let y_dist = a.y - b.y;

            let anti_1 = Coord {
                x: a.x + x_dist,
                y: a.y + y_dist
            };
            let anti_2 = Coord {
                x: b.x - x_dist,
                y: b.y - y_dist
            };

            if anti_1.in_bounds(&bounds) {
                antinodes.insert(anti_1);
            }
            if anti_2.in_bounds(&bounds) {
                antinodes.insert(anti_2);
            }
        }
    }

    println!("Part 1: {}", antinodes.len());

    // part 2
    antinodes = HashSet::new();

    for (_, positions) in antennas.iter() {
        for permitation in positions.into_iter().permutations(2) {
            let a = permitation[0];
            let b = permitation[1];

            antinodes.insert(*a);
            antinodes.insert(*b);

            let x_dist = a.x - b.x;
            let y_dist = a.y - b.y;

            let mut anti_1 = Coord {
                x: a.x + x_dist,
                y: a.y + y_dist
            };

            while anti_1.in_bounds(&bounds) {
                antinodes.insert(anti_1);
                anti_1 = Coord {
                    x: anti_1.x + x_dist,
                    y: anti_1.y + y_dist
                };
            }

            let mut anti_2 = Coord {
                x: b.x - x_dist,
                y: b.y - y_dist
            };

            while anti_2.in_bounds(&bounds) {
                antinodes.insert(anti_2);
                anti_2 = Coord {
                    x: anti_2.x - x_dist,
                    y: anti_2.y - y_dist
                };
            }
        }
    }

    println!("Part 2: {}", antinodes.len());
}
