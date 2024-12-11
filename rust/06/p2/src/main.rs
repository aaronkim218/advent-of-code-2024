use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let lines = read_lines("./input").expect("failed to read input file");
    let mut map: Vec<Vec<char>> = Vec::new();

    for line in lines.flatten() {
        let row: Vec<char> = line.chars().collect();
        map.push(row);
    }

    let start = find_start(&map);

    let mut total: u32 = 0;
    for i in 0..map.len() {
        for j in 0..map[0].len() {
            if map[i][j] == '.' {
                map[i][j] = '#';
                if has_loop(&map, start) {
                    total += 1;
                }
                map[i][j] = '.';
            }
        }
    }

    println!("{}", total);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn has_loop(map: &Vec<Vec<char>>, start: (usize, usize)) -> bool {
    let mut curr_dir = Direction::Up;
    let (mut x, mut y) = (start.0 as i16, start.1 as i16);
    let mut prev_pivots: HashSet<((usize, usize), Direction)> = HashSet::new();

    while in_bounds(&map, x, y) {
        let x_idx = x as usize;
        let y_idx = y as usize;

        match handle_obstruction(&map, x_idx, y_idx, curr_dir) {
            Some(new_dir) => {
                if prev_pivots.contains(&((x_idx, y_idx), curr_dir)) {
                    return true;
                }
    
                prev_pivots.insert(((x_idx, y_idx), curr_dir));
                curr_dir = new_dir;
            }
            None => {
                (x, y) = handle_move((x, y), curr_dir);
            }
        }
    }

    return false;
}

fn handle_obstruction(map: &Vec<Vec<char>>, x: usize, y: usize, dir: Direction) -> Option<Direction> {
    match dir {
        Direction::Up => {
            if y > 0 && map[y - 1][x] == '#' {
                return Some(Direction::Right);
            }
        }
        Direction::Down => {
            if y < map.len() - 1 && map[y + 1][x] == '#' {
                return Some(Direction::Left);
            }
        }
        Direction::Left => {
            if x > 0 && map[y][x - 1] == '#' {
                return Some(Direction::Up);
            }
        }
        Direction::Right => {
            if x < map[0].len() - 1 && map[y][x + 1] == '#' {
                return Some(Direction::Down);
            }
        }
    }

    return None;
}

fn handle_move((x, y): (i16, i16), dir: Direction) -> (i16, i16) {
    match dir {
        Direction::Up => (x, y - 1),
        Direction::Down => (x, y + 1),
        Direction::Left => (x - 1, y),
        Direction::Right => (x + 1, y),
    }
}

fn in_bounds(map: &Vec<Vec<char>>, x: i16, y: i16) -> bool {
    x >= 0 && x < map[0].len() as i16 && y >= 0 && y < map.len() as i16
}

fn find_start(map: &Vec<Vec<char>>) -> (usize, usize) {
    for (y, row) in map.iter().enumerate() {
        for (x, cell) in row.iter().enumerate() {
            if *cell == '^' {
                return (x, y);
            }
        }
    }

    panic!("No start found");
}

#[derive(Eq, PartialEq, Hash, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}
