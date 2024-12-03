use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

use regex::Regex;

const ENABLE_INSTRUCTION: &str = "do()";
const DISABLE_INSTRUCTION: &str = "don't()";

fn main() {
    let lines = read_lines("./input").expect("failed to read input file");
    let mut text = String::new();

    for line in lines.flatten() {
        text.push_str(&line);
    }

    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").expect("failed to compile regex");
    let mut sum: i128 = 0;
    let enabled_text = get_enabled_text(&text);

    for (_, [num1, num2]) in re.captures_iter(&enabled_text).map(|c| c.extract()) {
        let n1: i128 = num1.parse().expect("failed to parse number");
        let n2: i128 = num2.parse().expect("failed to parse number");

        sum += n1 * n2;
    }

    println!("sum: {}", sum);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn get_enabled_text(text: &str) -> String {
    let mut enabled_text = String::new();
    let mut is_enabled = true;

    for (i, c) in text.chars().enumerate() {
        match c {
            'd' => {
                if text[i..].starts_with(DISABLE_INSTRUCTION) {
                    is_enabled = false;
                } else if text[i..].starts_with(ENABLE_INSTRUCTION) {
                    is_enabled = true;
                }
            },
            _ => {
                if is_enabled {
                    enabled_text.push(c);
                }
            }
        }
    }

    enabled_text
}
