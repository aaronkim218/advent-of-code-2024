use core::panic;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let lines = read_lines("./input").expect("failed to read input file");
    let mut map: Vec<Vec<char>> = Vec::new();
    let mut instructions: Vec<char> = Vec::new();
    let mut instruction_section = false;

    for line in lines.flatten() {
        if line == "" {
            instruction_section = true;
            continue;
        }

        if instruction_section {
            let i: Vec<char> = line.chars().collect();
            instructions.extend(i);
        } else {
            let row: Vec<char> = line.chars().collect();
            map.push(row);
        }
    }

    simulate(&mut map, &instructions);

    let mut total: u128 = 0;

    for i in 0..map.len() {
        for j in 0..map[i].len() {
            if map[i][j] == 'O' {
                total += ((100 * i) + j) as u128;
            }
        }
    }

    println!("{}", total);
}

fn simulate(map: &mut Vec<Vec<char>>, instructions: &Vec<char>) {
    let (mut curr_row, mut curr_col) = find_start(&map);

    for instruction in instructions {
        (curr_row, curr_col) = handle_move(map, (curr_row, curr_col), instruction);
    }
}

fn handle_move(map: &mut Vec<Vec<char>>, (row, col): (usize, usize), instruction: &char) -> (usize, usize) {
    match instruction {
        '^' => {
            if map[row - 1][col] == 'O' {
                let (r, c) = handle_move(map, (row - 1, col), instruction);
                if (r, c) == (row - 1, col) {
                    return (row, col);
                }

                map[row - 1][col] = map[row][col];
                map[row][col] = '.';
                return (row - 1, col);
            } else if map[row - 1][col] == '#' {
                return (row, col);
            } else {
                map[row - 1][col] = map[row][col];
                map[row][col] = '.';
                return (row - 1, col);
            }
        },
        'v' => {
            if map[row + 1][col] == 'O' {
                let (r, c) = handle_move(map, (row + 1, col), instruction);
                if (r, c) == (row + 1, col) {
                    return (row, col);
                }

                map[row + 1][col] = map[row][col];
                map[row][col] = '.';
                return (row + 1, col);
            } else if map[row + 1][col] == '#' {
                return (row, col);
            } else {
                map[row + 1][col] = map[row][col];
                map[row][col] = '.';
                return (row + 1, col);
            }
        },
        '<' => {
            if map[row][col - 1] == 'O' {
                let (r, c) = handle_move(map, (row, col - 1), instruction);
                if (r, c) == (row, col - 1) {
                    return (row, col);
                }

                map[row][col - 1] = map[row][col];
                map[row][col] = '.';
                return (row, col - 1);
            } else if map[row][col - 1] == '#' {
                return (row, col);
            } else {
                map[row][col - 1] = map[row][col];
                map[row][col] = '.';
                return (row, col - 1);
            }
        },
        '>' => {
            if map[row][col + 1] == 'O' {
                let (r, c) = handle_move(map, (row, col + 1), instruction);
                if (r, c) == (row, col + 1) {
                    return (row, col);
                }

                map[row][col + 1] = map[row][col];
                map[row][col] = '.';
                return (row, col + 1);
            } else if map[row][col + 1] == '#' {
                return (row, col);
            } else {
                map[row][col + 1] = map[row][col];
                map[row][col] = '.';
                return (row, col + 1);
            }
        },
        _ => panic!("invalid instruction")

    }
}

fn find_start(map: &Vec<Vec<char>>) -> (usize, usize) {
    for i in 0..map.len() {
        for j in 0..map[i].len() {
            if map[i][j] == '@' {
                return (i, j);
            }
        }
    }

    panic!("start not found");
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
