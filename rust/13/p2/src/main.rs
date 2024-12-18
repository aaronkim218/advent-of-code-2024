use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let lines: Vec<String> = read_lines("./input").expect("failed to read input file").flatten().collect();
    let mut machines: Vec<Machine> = Vec::new();

    for i in (0..lines.len()).step_by(4) {
        let a = parse_line(&lines[i]);
        let b = parse_line(&lines[i + 1]);
        let (mut px, mut py) = parse_line(&lines[i + 2]);

        px += 10000000000000;
        py += 10000000000000;

        machines.push((a, b, (px, py)));
    }

    let mut sum: i128 = 0;

    for machine in machines {
        let tokens = count_tokens(machine);
        sum += tokens;
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

            digits.parse::<i128>().expect("failed to parse number")
        })
        .collect::<Vec<i128>>()
        .try_into()
        .map(|[x, y]: [i128; 2]| (x, y))
        .expect("failed to parse point")
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn count_tokens(((ax, ay), (bx, by), (px, py)): Machine) -> i128 {
    if ((py * ax) - (ay * px)) % ((by * ax) - (ay * bx)) != 0 {
        // not possible so return 0
        return 0;
    } 

    let nb = ((py * ax) - (ay * px)) / ((by * ax) - (ay * bx));

    if (px - (nb * bx)) % ax != 0 {
        // not possible so return 0
        return 0;
    }

    let na = (px - (nb * bx)) / ax;

    (3 * na) + (1 * nb)
}

type Point = (i128, i128);
type Machine = (Point, Point, Point);
