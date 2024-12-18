use std::cmp;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let lines: Vec<String> = read_lines("./input").expect("failed to read input file").flatten().collect();
    let mut machines: Vec<Machine> = Vec::new();

    for i in (0..lines.len()).step_by(4) {
        let a = parse_line(&lines[i]);
        let b = parse_line(&lines[i + 1]);
        let prize = parse_line(&lines[i + 2]);

        machines.push((a, b, prize));
    }

    let mut sum: u32 = 0;

    for machine in machines {
        let mut memo = HashMap::new();
        let tokens= count_tokens((0, 0), machine, &mut memo);
        if tokens != u32::MAX {
            sum += tokens;
        }
    }

    println!("{}", sum);
}

fn parse_line(line: &String) -> Point {
    line
        .chars()
        .filter(|&c: &char| c != ',')
        .collect::<String>()
        .split_whitespace()
        .map(|s| s.to_string())
        .rev()
        .take(2)
        .collect::<Vec<String>>()
        .into_iter()
        .rev()
        .map(|s| {
            let digits = s
                .chars()
                .filter(|&c: &char| c.is_digit(10))
                .collect::<String>();

            digits.parse::<usize>().expect("failed to parse number")
        })
        .collect::<Vec<usize>>()
        .try_into()
        .map(|[x, y]: [usize; 2]| (x, y))
        .expect("failed to parse point")
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn count_tokens((curr_x, curr_y): Point, machine: Machine, memo: &mut HashMap<Point, u32>) -> u32 {
    let ((ax, ay), (bx, by), (px, py)) = machine;

    if (curr_x, curr_y) == (px, py) {
        return 0;
    }

    if let Some(&tokens) = memo.get(&(curr_x, curr_y)) {
        return tokens;
    }

    if curr_x > px || curr_y > py {
        return u32::MAX;
    }

    let result = cmp::min(
        count_tokens((curr_x + ax, curr_y + ay), machine, memo).saturating_add(3), 
        count_tokens((curr_x + bx, curr_y + by), machine, memo).saturating_add(1)
    );

    memo.insert((curr_x, curr_y), result);

    result
}

type Point = (usize, usize);
type Machine = (Point, Point, Point);
