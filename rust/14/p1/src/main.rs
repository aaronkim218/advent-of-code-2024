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

    let mut q1: u128 = 0;
    let mut q2: u128 = 0;
    let mut q3: u128 = 0;
    let mut q4: u128 = 0;

    simulate(&mut robots, 100);

    for robot in robots {
        println!("{:?}", robot);

        if robot.p.0 < WIDTH / 2 && robot.p.1 < HEIGHT / 2 {
            q1 += 1;
        } else if robot.p.0 > WIDTH / 2 && robot.p.1 < HEIGHT / 2 {
            q2 += 1;
        } else if robot.p.0 < WIDTH / 2 && robot.p.1 > HEIGHT / 2 {
            q3 += 1;
        } else if robot.p.0 > WIDTH / 2 && robot.p.1 > HEIGHT / 2 {
            q4 += 1;
        }
    }

    let total: u128 = q1 * q2 * q3 * q4;

    println!("{}", total);
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