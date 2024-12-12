use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let lines = read_lines("./input").expect("failed to read input file");
    let mut map: Vec<Vec<u32>> = Vec::new();

    for line in lines.flatten() {
        let row: Vec<u32> = line
            .chars()
            .map(|c| c.to_digit(10).expect("invalid digit"))
            .collect();
        map.push(row);
    }


    let mut sum: u32 = 0;
    for (i, row) in map.iter().enumerate() {
        for (j, cell) in row.iter().enumerate() {
            if *cell == 0 {
                let mut ends: HashSet<(usize, usize)> = HashSet::new();
                dfs(&map, (j, i), &mut ends);
                sum += ends.len() as u32;
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

fn dfs(map: &Vec<Vec<u32>>, (x, y): (usize, usize), ends: &mut HashSet<(usize, usize)>) {
    let curr = map[y][x];
    if curr == 9 {
        ends.insert((x, y));
        return;
    }

    if x > 0 && map[y][x - 1] == curr + 1 {
        dfs(map, (x - 1, y), ends);
    }

    if x < map[0].len() - 1 && map[y][x + 1] == curr + 1 {
        dfs(map, (x + 1, y), ends);
    }

    if y > 0 && map[y - 1][x] == curr + 1 {
        dfs(map, (x, y - 1), ends);
    }

    if y < map.len() - 1 && map[y + 1][x] == curr + 1 {
        dfs(map, (x, y + 1), ends);
    }
}
