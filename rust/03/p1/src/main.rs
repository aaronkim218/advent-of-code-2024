use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

use regex::Regex;

fn main() {
    let lines = read_lines("./input").expect("failed to read input file");
    let mut text = String::new();

    for line in lines.flatten() {
        text.push_str(&line);
    }

    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").expect("failed to compile regex");
    let mut sum: u128 = 0;

    for (_, [num1, num2]) in re.captures_iter(&text).map(|c| c.extract()) {
        let n1: u128 = num1.parse().expect("failed to parse number");
        let n2: u128 = num2.parse().expect("failed to parse number");

        sum += n1 * n2;
    }

    println!("sum: {}", sum);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
