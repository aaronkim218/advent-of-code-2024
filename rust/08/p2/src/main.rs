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

    let antennas = get_unique_antennas(&map);
    let mut antinode_map = map.clone();

    paint_antinode_map(&map, &mut antinode_map, &antennas);

    let total = get_antinode_positions(&antinode_map);
    println!("{}", total);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn paint_antinode_map(map: &Vec<Vec<char>>, antinode_map: &mut Vec<Vec<char>>, antennas: &HashSet<char>) {
    for antenna in antennas {
        let positions = get_antenna_positions(&map, *antenna);

        for i in 0..positions.len() {
            for j in i+1..positions.len() {
                let (x1, y1) = positions[i];
                let (x2, y2) = positions[j];
                antinode_map[y1][x1] = '#';
                antinode_map[y2][x2] = '#';

                let dx = x2 as i16 - x1 as i16;
                let dy = y2 as i16 - y1 as i16;

                let mut ax = x1 as i16 - dx;
                let mut ay = y1 as i16 - dy;

                while in_bounds(&map, ax, ay) {
                    antinode_map[ay as usize][ax as usize] = '#';
                    ax -= dx;
                    ay -= dy;
                }

                ax = x2 as i16 + dx;
                ay = y2 as i16 + dy;

                while in_bounds(&map, ax, ay) {
                    antinode_map[ay as usize][ax as usize] = '#';
                    ax += dx;
                    ay += dy;
                }
            }
        }

    }
}

fn in_bounds(map: &Vec<Vec<char>>, x: i16, y: i16) -> bool {
    x >= 0 && x < map[0].len() as i16 && y >= 0 && y < map.len() as i16
}

fn get_antenna_positions(map: &Vec<Vec<char>>, antenna: char) -> Vec<(usize, usize)> {
    let mut positions: Vec<(usize, usize)> = Vec::new();

    for (y, row) in map.iter().enumerate() {
        for (x, cell) in row.iter().enumerate() {
            if *cell == antenna {
                positions.push((x, y));
            }
        }
    }

    positions
}

fn get_antinode_positions(map: &Vec<Vec<char>>) -> usize {
    let mut total = 0;

    for row in map.iter() {
        for cell in row.iter() {
            if *cell == '#' {
                total += 1;
            }
        }
    }

    total
}

fn get_unique_antennas(map: &Vec<Vec<char>>) -> HashSet<char> {
    let mut antennas: HashSet<char> = HashSet::new();

    for row in map.iter() {
        for cell in row.iter() {
            if !antennas.contains(cell) && *cell != '.' {
                antennas.insert(*cell);
            }
        }
    }

    antennas
}
