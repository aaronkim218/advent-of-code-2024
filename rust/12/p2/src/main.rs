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
                let mut borders = Borders {
                    left: HashSet::new(),
                    right: HashSet::new(),
                    top: HashSet::new(),
                    bottom: HashSet::new(),
                };
                let area = dfs(&map, (i, j), &mut visited, &mut borders);
                let sides = count_sides(&mut borders);
                sum += area * sides;
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

fn dfs(map: &Vec<Vec<char>>, (row, col): (usize, usize), visited: &mut HashSet<(usize, usize)>, borders: &mut Borders) -> u32 {
    let curr = map[row][col];
    visited.insert((row, col));
    let mut area: u32 = 1;

    if col > 0 && map[row][col - 1] == curr && !visited.contains(&(row, col - 1)) {
        let a = dfs(map, (row, col - 1), visited, borders);
        area += a;
    } else if col <= 0 || map[row][col - 1] != curr {
        borders.left.insert((row, col));
    }

    if col < map[0].len() - 1 && map[row][col + 1] == curr && !visited.contains(&(row, col + 1)) {
        let a = dfs(map, (row, col + 1), visited, borders);
        area += a;
    } else if col >= map[0].len() - 1 || map[row][col + 1] != curr {
        borders.right.insert((row, col));
    }

    if row > 0 && map[row - 1][col] == curr && !visited.contains(&(row - 1, col)) {
        let a = dfs(map, (row - 1, col), visited, borders);
        area += a;
    } else if row <= 0 || map[row - 1][col] != curr {
        borders.top.insert((row, col));
    }

    if row < map.len() - 1 && map[row + 1][col] == curr && !visited.contains(&(row + 1, col)) {
        let a = dfs(map, (row + 1, col), visited, borders);
        area += a;
    } else if row >= map.len() - 1 || map[row + 1][col] != curr {
        borders.bottom.insert((row, col));
    }

    return area;
}

fn count_sides(borders: &mut Borders) -> u32 {
    let mut sides: u32 = 0;
    let mut visited: HashSet<(usize, usize)> = HashSet::new();

    for (row, col) in borders.left.iter() {
        if visited.contains(&(*row, *col)) {
            continue;
        }

        sides += 1;

        // remove from both sides
        let mut r = *row;
        while borders.left.contains(&(r, *col)) {
            visited.insert((r, *col));
            if r == 0 {
                // avoid overflow
                break;
            }

            r -= 1;
        }

        r = *row;
        while borders.left.contains(&(r, *col)) {
            visited.insert((r, *col));
            r += 1;
        }
    }

    visited.clear();
    for (row, col) in borders.right.iter() {
        if visited.contains(&(*row, *col)) {
            continue;
        }

        sides += 1;

        // remove from both sides
        let mut r: usize = *row;
        while borders.right.contains(&(r, *col)) {
            visited.insert((r, *col));
            if r == 0 {
                // avoid overflow
                break;
            }

            r -= 1;
        }

        r = *row;
        while borders.right.contains(&(r, *col)) {
            visited.insert((r, *col));
            r += 1;
        }
    }

    visited.clear();
    for (row, col) in borders.top.iter() {
        if visited.contains(&(*row, *col)) {
            continue;
        }

        sides += 1;

        // remove from both sides
        let mut c = *col;
        while borders.top.contains(&(*row, c)) {
            visited.insert((*row, c));
            if c == 0 {
                // avoid overflow
                break;
            }

            c -= 1;
        }

        c = *col;
        while borders.top.contains(&(*row, c)) {
            visited.insert((*row, c));
            c += 1;
        }
    }

    visited.clear();
    for (row, col) in borders.bottom.iter() {
        if visited.contains(&(*row, *col)) {
            continue;
        }

        sides += 1;

        // remove from both sides
        let mut c = *col;
        while borders.bottom.contains(&(*row, c)) {
            visited.insert((*row, c));
            if c == 0 {
                // avoid overflow
                break;
            }

            c -= 1;
        }

        c = *col;
        while borders.bottom.contains(&(*row, c)) {
            visited.insert((*row, c));
            c += 1;
        }
    }

    sides
}

struct Borders {
    left: HashSet<(usize, usize)>,
    right: HashSet<(usize, usize)>,
    top: HashSet<(usize, usize)>,
    bottom: HashSet<(usize, usize)>,
}