use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let lines = read_lines("./input").expect("failed to read input file");
    let mut map: Vec<Vec<char>> = Vec::new();

    for line in lines.flatten() {
        let row: Vec<char> = line
            .chars()
            .collect();
        map.push(row);
    }

    let mut sum: u32 = 0;
    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    for i in 0..map.len() {
        for j in 0..map[0].len() {
            if !visited.contains(&(i, j)) {
                let (perimeter, area) = dfs(&map, (i, j), &mut visited);
                sum += area * perimeter;
            }
        }
    }

    println!("{}", sum);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn dfs(map: &Vec<Vec<char>>, (row, col): (usize, usize), visited: &mut HashSet<(usize, usize)>) -> (u32, u32) {
    let curr = map[row][col];
    visited.insert((row, col));
    let mut perimeter: u32 = 0;
    let mut area: u32 = 1;

    if col > 0 && map[row][col - 1] == curr && !visited.contains(&(row, col - 1)) {
        let (p, a) = dfs(map, (row, col - 1), visited);
        perimeter += p;
        area += a;
    } else if col <= 0 || map[row][col - 1] != curr {
        perimeter += 1;
    }

    if col < map[0].len() - 1 && map[row][col + 1] == curr && !visited.contains(&(row, col + 1)) {
        let (p, a) = dfs(map, (row, col + 1), visited);
        perimeter += p;
        area += a;
    } else if col >= map[0].len() - 1 || map[row][col + 1] != curr {
        perimeter += 1;
    }

    if row > 0 && map[row - 1][col] == curr && !visited.contains(&(row - 1, col)) {
        let (p, a) = dfs(map, (row - 1, col), visited);
        perimeter += p;
        area += a;
    } else if row <= 0 || map[row - 1][col] != curr {
        perimeter += 1;
    }

    if row < map.len() - 1 && map[row + 1][col] == curr && !visited.contains(&(row + 1, col)) {
        let (p, a) = dfs(map, (row + 1, col), visited);
        perimeter += p;
        area += a;
    } else if row >= map.len() - 1 || map[row + 1][col] != curr {
        perimeter += 1;
    }

    return (perimeter, area);
}