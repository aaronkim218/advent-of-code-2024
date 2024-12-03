use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let lines = read_lines("./input").expect("failed to read input file");
    let mut v: Vec<Vec<i32>> = Vec::new();

    for line in lines.flatten() {
        let mut nums: Vec<i32> = Vec::new();

        for part in line.split_whitespace() {
            let num: i32 = part.parse().expect("failed to parse number");
            nums.push(num);
        }

        v.push(nums);
    }

    let mut sum: u16 = 0;
    for nums in v {
        if is_safe_with_removal(&nums) {
            sum += 1;
        }
    }

    println!("number of safe reports: {}", sum);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn is_increasing_safe(v: &Vec<i32>) -> bool {
    for i in 1..v.len() {
        if v[i] < v[i - 1] + 1 || v[i] > v[i - 1] + 3 {
            return false;
        }
    }

    true
}

fn is_decreasing_safe(v: &Vec<i32>) -> bool {
    for i in 1..v.len() {
        if v[i] > v[i - 1] - 1 || v[i] < v[i - 1] - 3 {
            return false;
        }
    }

    true
}

fn is_safe_with_removal(v: &Vec<i32>) -> bool {
    for i in 0..v.len() {
        let mut reduced = v.clone();
        reduced.remove(i);

        if is_increasing_safe(&reduced) || is_decreasing_safe(&reduced) {
            return true;
        }
    }

    false
}
