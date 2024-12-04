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
            if i + 1 < chars.len() && i >= 1 && j + 1 < chars[i].len() && j >= 1 {
                if is_mas(chars[i+1][j-1], chars[i][j], chars[i-1][j+1]) && is_mas(chars[i-1][j-1], chars[i][j], chars[i+1][j+1]) {
                    total += 1;
                }
            }
        }
    }

    return total;
}

fn is_mas(c1: char, c2: char, c3: char) -> bool {
    (c1 == 'M' && c2 == 'A' && c3 == 'S') || (c1 == 'S' && c2 == 'A' && c3 == 'M')
}
