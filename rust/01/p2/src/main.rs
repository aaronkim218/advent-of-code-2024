use std::collections::HashMap;
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

    let c = get_counter(&v2);

    let score = calc_similarity_score(&v1, &c);

    println!("similarity score: {}", score);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn get_counter(v: &Vec<u32>) -> HashMap<u32, u8> {
    let mut counter: HashMap<u32, u8> = HashMap::new();

    for num in v {
        *counter.entry(*num).or_insert(0) += 1;
    }

    counter
}

fn calc_similarity_score(v: &Vec<u32>, c: &HashMap<u32, u8>) -> u64 {
    let mut score: u64 = 0;

    for num in v {
        score += *num as u64 * *c.get(num).unwrap_or(&0) as u64;
    }

    score
}