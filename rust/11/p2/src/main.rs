use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let lines = read_lines("./input").expect("failed to read input file");
    let mut stone_counts: HashMap<u128, usize> = lines
        .flatten()
        .next()
        .expect("failed to read stones")
        .split_whitespace()
        .map(|s| s.parse().expect("failed to parse stone"))
        .fold(HashMap::new(), |mut acc, stone| {
            *acc.entry(stone).or_insert(0) += 1;
            acc
        });

    for _ in  0..75 {
        stone_counts = blink(&mut stone_counts);
    }

    let mut total: u128 = 0;
    for (_, count) in stone_counts {
        total += count as u128;
    }
    
    println!("{}", total);
}

fn blink(stone_counts: &mut HashMap<u128, usize>) -> HashMap<u128, usize> {
    let mut new_stone_counts: HashMap<u128, usize> = HashMap::new();
    let mut updates: Vec<(u128, usize)> = Vec::new();

    for (stone, count) in stone_counts.iter() {
        if *stone == 0 {
            updates.push((1, *count));
        } else if let Some(digits) = has_even_number_of_digits(*stone) {
            let (left, right) = split_stone(*stone, digits);
            updates.push((left, *count));
            updates.push((right, *count));
        } else {
            updates.push((*stone * 2024, *count));
        }
    }

    for (stone, count) in updates {
        *new_stone_counts.entry(stone).or_insert(0) += count;
    }

    new_stone_counts
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
