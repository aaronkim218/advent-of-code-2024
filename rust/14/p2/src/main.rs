use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

const WIDTH: i32 = 101;
const HEIGHT: i32 = 103;

fn main() {
    let lines = read_lines("./input").expect("failed to read input file");
    let mut robots: Vec<Robot> = Vec::new();

    for line in lines.flatten() {
        let (p, v) = parse_line(&line);
        robots.push(Robot {
            p,
            v,
        });
    }

    let (mut min_i, mut min_positions): (usize, usize) = (0, usize::MAX);
    let (mut max_i, mut max_positions): (usize, usize) = (0, 0);
    for i in 0..10000 {
        let positions: HashSet<(i32, i32)> = robots.iter().map(|robot| robot.p).collect();
        let unique_positions = positions.len();

        if unique_positions < min_positions {
            min_positions = unique_positions;
            min_i = i;
        }

        if unique_positions > max_positions {
            max_positions = unique_positions;
            max_i = i;
        }

        simulate(&mut robots, 1);
    }

    println!("seconds: {}, positions: {}", min_i, min_positions);
    println!("seconds: {}, positions: {}", max_i, max_positions);
}

fn parse_line(line: &String) -> ((i32, i32), (i32, i32)) {
    let mut parts: Vec<String> = line.split_whitespace().map(|s| s.to_string()).collect();
    assert!(parts.len() == 2);

    parts
        .iter_mut()
        .for_each(|s| {
            *s = s
                .chars()
                .filter(|c| c.is_digit(10) || *c == ',' || *c == '-')
                .collect();
        });

    let first_pair: Vec<i32> = parts[0]
        .split(',')
        .map(|s| s.parse::<i32>().expect("failed to parse usize"))
        .collect();
    let second_pair: Vec<i32> = parts[1]
        .split(',')
        .map(|s| s.parse::<i32>().expect("failed to parse usize"))
        .collect();

    ((first_pair[0], first_pair[1]), (second_pair[0], second_pair[1]))
}

fn simulate(robots: &mut Vec<Robot>, iterations: u8) {
    for robot in robots {
        let mut px = (robot.p.0 as i32 + (robot.v.0 * iterations as i32)) % WIDTH as i32;
        if px < 0 {
            px += WIDTH;
        }
        let mut py = (robot.p.1 as i32 + (robot.v.1 * iterations as i32)) % HEIGHT as i32;
        if py < 0 {
            py += HEIGHT;
        }

        robot.p = (px, py);
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

#[derive(Debug)]
struct Robot {
    p: (i32, i32),
    v: (i32, i32),
}