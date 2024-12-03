use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let lines = read_lines("./input").expect("failed to read input file");
    let mut v1: Vec<u32> = Vec::new();
    let mut v2: Vec<u32> = Vec::new();

    for line in lines.flatten() {
        let parts: Vec<&str> = line.split_whitespace().collect();

        assert!(parts.len() == 2, "line does not contain exactly two numbers");

        let num1: u32 = parts[0].parse().expect("failed to parse first number");
        let num2: u32 = parts[1].parse().expect("failed to parse second number");

        v1.push(num1);
        v2.push(num2);
    }

    v1.sort();
    v2.sort();

    let sum = calc_delta_sum(&v1, &v2);
    println!("sum: {}", sum);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn calc_delta_sum(v1: &Vec<u32>, v2: &Vec<u32>) -> u32 {
    let mut sum: u32 = 0;

    for i in 0..v1.len() {
        sum += v1[i].abs_diff(v2[i]);
    }

    sum
}