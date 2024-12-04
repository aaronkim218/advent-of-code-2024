use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let lines = read_lines("./input").expect("failed to read input file");
    let mut chars: Vec<Vec<char>> = Vec::new();

    for line in lines.flatten() {
        chars.push(line.chars().collect());
    }

    let total: u32 = search(chars);
    
    println!("total: {}", total);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn search(chars: Vec<Vec<char>>) -> u32 {
    let mut total: u32 = 0;

    for i in 0..chars.len() {
        for j in 0..chars[i].len() {
            // horizontal right
            if j + 3 < chars[i].len() {
                if is_xmas(chars[i][j], chars[i][j + 1], chars[i][j + 2], chars[i][j + 3]) {
                    total += 1;
                }
            }

            // horizontal left
            if j >= 3 {
                if is_xmas(chars[i][j], chars[i][j - 1], chars[i][j - 2], chars[i][j - 3]) {
                    total += 1;
                }
            }

            // vertical up
            if i + 3 < chars.len() {
                if is_xmas(chars[i][j], chars[i + 1][j], chars[i + 2][j], chars[i + 3][j]) {
                    total += 1;
                }
            }

            // vertical down
            if i >= 3 {
                if is_xmas(chars[i][j], chars[i - 1][j], chars[i - 2][j], chars[i - 3][j]) {
                    total += 1;
                }
            }

            // diagonal bottom right
            if i + 3 < chars.len() && j + 3 < chars[i].len() {
                if is_xmas(chars[i][j], chars[i + 1][j + 1], chars[i + 2][j + 2], chars[i + 3][j + 3]) {
                    total += 1;
                }
            }

            // diagonal top left
            if i >= 3 && j >= 3 {
                if is_xmas(chars[i][j], chars[i - 1][j - 1], chars[i - 2][j - 2], chars[i - 3][j - 3]) {
                    total += 1;
                }
            }

            // diagonal bottom left
            if i + 3 < chars.len() && j >= 3 {
                if is_xmas(chars[i][j], chars[i + 1][j - 1], chars[i + 2][j - 2], chars[i + 3][j - 3]) {
                    total += 1;
                }
            }

            // diagonal top right
            if i >= 3 && j + 3 < chars[i].len() {
                if is_xmas(chars[i][j], chars[i - 1][j + 1], chars[i - 2][j + 2], chars[i - 3][j + 3]) {
                    total += 1;
                }
            }
        }
    }

    return total;
}

fn is_xmas(c1: char, c2: char, c3: char, c4: char) -> bool {
    c1 == 'X' && c2 == 'M' && c3 == 'A' && c4 == 'S'
}
