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

    paint_path(&mut map, start);

    let total = get_distinct_positions(&map);
    println!("{}", total);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn paint_path(map: &mut Vec<Vec<char>>, start: (usize, usize)) {
    let mut curr_dir = Direction::Up;
    let (mut x, mut y) = (start.0 as i16, start.1 as i16);

    while in_bounds(&map, x, y) {        
        let x_idx = x as usize;
        let y_idx = y as usize;
        map[y_idx][x_idx] = 'X';

        match curr_dir {
            Direction::Up => {
                if y > 0 && map[y_idx - 1][x_idx] == '#' {
                    curr_dir = Direction::Right;
                } else {
                    y -= 1;
                }
            }
            Direction::Down => {
                if y_idx < map.len() - 1 && map[y_idx + 1][x_idx] == '#' {
                    curr_dir = Direction::Left;
                } else {
                    y += 1;
                }
            }
            Direction::Left => {
                if x_idx > 0 && map[y_idx][x_idx - 1] == '#' {
                    curr_dir = Direction::Up;
                } else {
                    x -= 1;
                }
            }
            Direction::Right => {
                if x_idx < map[0].len() - 1 && map[y_idx][x_idx + 1] == '#' {
                    curr_dir = Direction::Down;
                } else {
                    x += 1;
                }
            }
        }
    }
}

fn in_bounds(map: &Vec<Vec<char>>, x: i16, y: i16) -> bool {
    x >= 0 && x < map[0].len() as i16 && y >= 0 && y < map.len() as i16
}

fn get_distinct_positions(map: &Vec<Vec<char>>) -> usize {
    let mut total = 0;

    for row in map.iter() {
        for cell in row.iter() {
            if *cell == 'X' {
                total += 1;
            }
        }
    }

    total
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

enum Direction {
    Up,
    Down,
    Left,
    Right,
}
