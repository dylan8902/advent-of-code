use std::fmt;
use std::fs;
use std::collections::HashSet;


#[derive(Clone, Copy, Hash, Eq, PartialEq)]
struct Coord {
    pub x: i32,
    pub y: i32,
}

impl fmt::Display for Coord {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}, {}", self.x, self.y)
    }
}

impl Coord {
    fn advance(&mut self, direction: &Direction) {
        match direction {
            Direction::Up => self.y -= 1,
            Direction::Down => self.y += 1,
            Direction::Left => self.x += 1,
            Direction::Right => self.x -= 1,
        }
    }
}

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl fmt::Display for Direction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Direction::Up => write!(f, "Up"),
            Direction::Left => write!(f, "Left"),
            Direction::Down => write!(f, "Down"),
            Direction::Right => write!(f, "Right"),
        }
    }
}

impl Direction {
    fn change_direction(self) -> Direction {
        match self {
            Direction::Up => Direction::Left,
            Direction::Left => Direction::Down,
            Direction::Down => Direction::Right,
            Direction::Right => Direction::Up,
        }
    }
}

fn main() {
    // Read the file as rows and columns
    let rows: Vec<Vec<char>> = fs::read_to_string("input.txt")
        .unwrap()
        .lines()
        .map(|s| s.chars().collect())
        .collect();

    let mut starting_position: Coord = Coord { x: 0, y: 0 };
    let mut obstacles: Vec<Coord> = Vec::new();

    // Parse input
    for y in 0..rows.len() {
        for x in 0..rows[y].len() {
            if rows[y][x].eq(&'^') {
                starting_position.x = x as i32;
                starting_position.y = y as i32;
            } else if rows[y][x].eq(&'#') {
                obstacles.push(Coord {
                    x: x as i32,
                    y: y as i32,
                });
            }
        }
    }

    println!(
        "Starting position is {} - there are {} obstacles",
        starting_position,
        obstacles.len()
    );

    let mut path = Vec::new();
    path.push(starting_position);

    let mut direction = Direction::Up;

    loop {
        let mut pos: Coord = *path.last().unwrap();
        pos.advance(&direction);

        // check if position is an obstacle, change direction
        if obstacles.contains(&pos) {
            direction = direction.change_direction();
            println!("Changing direction: {}", direction);
            continue;
        }

        // check if position is out of bounds
        if pos.x >= rows[0].len() as i32 || pos.x < 0 || pos.y >= rows.len() as i32 || pos.y < 0 {
            println!("Leaving the map at {}", pos);
            break;
        }

        path.push(pos);
    }

    let set: HashSet<Coord> = HashSet::from_iter(path.into_iter());

    println!("Part 1: {}", set.len());


}
