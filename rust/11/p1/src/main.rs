use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let lines = read_lines("./input").expect("failed to read input file");
    let mut stones: Vec<u128> = lines
        .flatten()
        .next()
        .expect("failed to read stones")
        .split_whitespace()
        .map(|s| s.parse().expect("failed to parse stone"))
        .collect();

    for _ in  0..25 {
        blink(&mut stones);
    }
    
    println!("{}", stones.len());
}

fn blink(stones: &mut Vec<u128>) {
    let mut i: usize = 0;

    while i < stones.len() {
        if stones[i] == 0 {
            stones[i] = 1;
        } else if let Some(digits) = has_even_number_of_digits(stones[i]) {
            let (left, right) = split_stone(stones[i], digits);
            stones[i] = left;
            stones.insert(i + 1, right);
            i += 1;
        } else {
            stones[i] *= 2024;
        }

        i += 1;
    }
}

fn has_even_number_of_digits(n: u128) -> Option<u32> {
    let mut n = n;
    let mut count = 0;

    // account for the first digit (even if it's 0)
    count += 1;
    n /= 10;

    while n != 0 {
        count += 1;
        n /= 10;
    }

    if count % 2 == 0 {
        return Some(count);
    } else {
        return None;
    }
}

fn split_stone(n: u128, digits: u32) -> (u128, u128) {
    return (n / 10u128.pow(digits / 2), n % 10u128.pow(digits / 2));
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
