use std::{
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader, Write},
};

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn vector(&self) -> (i64, i64) {
        match self {
            Direction::Up => (0, -1),
            Direction::Down => (0, 1),
            Direction::Left => (-1, 0),
            Direction::Right => (1, 0),
        }
    }

    fn next_direction(&self) -> Direction {
        match self {
            Direction::Up => Direction::Right,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
            Direction::Right => Direction::Down,
        }
    }
}

impl TryFrom<char> for Direction {
    type Error = String;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '^' => Ok(Direction::Up),
            'v' => Ok(Direction::Down),
            '<' => Ok(Direction::Left),
            '>' => Ok(Direction::Right),
            _ => Err("Unknown char".to_owned()),
        }
    }
}

#[derive(Debug, Clone)]
struct Map {
    obstacles: Vec<(i64, i64)>,
    size: (i64, i64),
    guard_position: (i64, i64),
    guard_direction: Direction,
}

fn load_input(path: &'static str) -> Map {
    let file = match File::open(path) {
        Ok(x) => x,
        Err(err) => {
            panic!(
                "Make sure that you copied your input to the day directory!\nError: {}",
                err
            );
        }
    };

    let file_reader = BufReader::new(file);

    let mut map = Map {
        obstacles: Vec::new(),
        size: (0, 0),
        guard_position: (0, 0),
        guard_direction: Direction::Up,
    };

    for (y, line) in file_reader.lines().filter_map(Result::ok).enumerate() {
        map.size.1 = y as i64 + 1;
        for (x, char) in line.chars().enumerate() {
            map.size.0 = x as i64 + 1;
            match char {
                '#' => map.obstacles.push((x as i64, y as i64)),
                '^' | '<' | '>' | 'v' => {
                    map.guard_position = (x as i64, y as i64);
                    map.guard_direction = Direction::try_from(char).unwrap();
                }
                _ => {}
            }
        }
    }

    map
}

fn part1(map: &Map) -> i64 {
    let mut current_guard_position = map.guard_position;
    let mut current_guard_direction = map.guard_direction;

    let mut visited_positions: HashSet<(i64, i64)> = HashSet::new();

    while current_guard_position.0 >= 0
        && current_guard_position.0 < map.size.0
        && current_guard_position.1 >= 0
        && current_guard_position.1 < map.size.1
    {
        visited_positions.insert(current_guard_position);

        let dir_vector = current_guard_direction.vector();
        let new_x = current_guard_position.0 + dir_vector.0;
        let new_y = current_guard_position.1 + dir_vector.1;

        if map.obstacles.contains(&(new_x, new_y)) {
            current_guard_direction = current_guard_direction.next_direction();
        } else {
            current_guard_position = (new_x, new_y);
        }
    }

    visited_positions.len() as i64
}

fn part2(map: &Map) -> i64 {
    let mut current_guard_position = map.guard_position;
    let mut current_guard_direction = map.guard_direction;

    let mut visited_positions: HashSet<(i64, i64)> = HashSet::new();

    while current_guard_position.0 >= 0
        && current_guard_position.0 < map.size.0
        && current_guard_position.1 >= 0
        && current_guard_position.1 < map.size.1
    {
        visited_positions.insert(current_guard_position);

        let dir_vector = current_guard_direction.vector();
        let new_x = current_guard_position.0 + dir_vector.0;
        let new_y = current_guard_position.1 + dir_vector.1;

        if map.obstacles.contains(&(new_x, new_y)) {
            current_guard_direction = current_guard_direction.next_direction();
        } else {
            current_guard_position = (new_x, new_y);
        }
    }

    let mut visited_positions_dir: HashSet<(i64, i64, Direction)> = HashSet::new();

    let mut out = 0;

    for new_obstacle in visited_positions {
        current_guard_position = map.guard_position;
        current_guard_direction = map.guard_direction;

        visited_positions_dir.clear();

        while current_guard_position.0 >= 0
            && current_guard_position.0 < map.size.0
            && current_guard_position.1 >= 0
            && current_guard_position.1 < map.size.1
        {
            let visited_dir = (
                current_guard_position.0,
                current_guard_position.1,
                current_guard_direction,
            );

            if visited_positions_dir.contains(&visited_dir) {
                out += 1;
                break;
            }

            visited_positions_dir.insert(visited_dir);

            let dir_vector = current_guard_direction.vector();
            let new_x = current_guard_position.0 + dir_vector.0;
            let new_y = current_guard_position.1 + dir_vector.1;

            if map.obstacles.contains(&(new_x, new_y))
                || (new_x == new_obstacle.0 && new_y == new_obstacle.1)
            {
                current_guard_direction = current_guard_direction.next_direction();
            } else {
                current_guard_position = (new_x, new_y);
            }
        }
    }

    out
}

fn main() {
    let map = load_input("./day6/input.txt");

    println!("Part 1: {}", part1(&map));
    println!("Part 2: {}", part2(&map));
}
