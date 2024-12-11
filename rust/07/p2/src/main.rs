use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let lines = read_lines("./input").expect("failed to read input file");
    let mut equations: Vec<(u128, Vec<u16>)> = Vec::new();

    for line in lines.flatten() {
        let mut parts = line.split(" ");

        let result_str: &str = parts.next().expect("missing result");
        // check that result contains at least one digit and ":"
        assert!(result_str.len() >= 2);
        let result: u128 = result_str[..result_str.len() - 1].parse().expect("failed to parse result");

        let operands: Vec<u16> = parts
            .map(|part| part.parse().expect("failed to parse operand"))
            .collect();

        equations.push((result, operands));
    }

    let mut total: u128 = 0;
    for (result, operands) in equations {
        if is_equal(result, &operands) {
            total += result;
        }
    }

    println!("{}", total);
}

fn is_equal(result: u128, operands: &Vec<u16>) -> bool {
    fn dfs(
        i: usize,
        curr: u128,
        target: u128,
        operands: &Vec<u16>,
    ) -> bool {
        if i == operands.len() {
            return curr == target;
        }

        if curr > target {
            return false;
        }

        let concatenation: u128 = (curr.to_string() + operands[i].to_string().as_str())
            .parse()
            .expect("failed to parse concatenation");
        return dfs(i + 1, curr + operands[i] as u128, target, operands)
            || dfs(i + 1, curr * operands[i] as u128, target, operands)
            || dfs(i + 1, concatenation, target, operands);
    }

    assert!(operands.len() > 0);
    dfs(1, operands[0] as u128, result, &operands)
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
