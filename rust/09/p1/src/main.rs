use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let lines = read_lines("./input").expect("failed to read input file");
    let disk_map: Vec<u32> = lines
        .flatten()
        .next()
        .expect("no lines in input file")
        .chars()
        .map(|c| c.to_digit(10).expect("invalid digit"))
        .collect();

    let mut blocks = get_blocks(&disk_map);
    move_blocks(&mut blocks);

    let checksum = calc_checksum(&blocks);

    println!("{}", checksum);
}

fn calc_checksum(blocks: &Vec<i16>) -> u128 {
    let mut checksum: u128 = 0;

    for i in 0..blocks.len() {
        if blocks[i] != -1 {
            checksum += (i as u128) * blocks[i] as u128;
        }
    }

    checksum
}

fn move_blocks(blocks: &mut Vec<i16>) {
    let mut l: usize = 0;
    let mut r = blocks.len() - 1;

    while l < r {
        while l < r && blocks[l] != -1 {
            l += 1;
        }

        while l < r && blocks[r] == -1 {
            r -= 1;
        }

        if l < r {
            blocks[l] = blocks[r];
            blocks[r] = -1;
        }
    }
}

fn get_blocks(disk_map: &Vec<u32>) -> Vec<i16> {
    let mut blocks: Vec<i16> = Vec::new();
    let mut disk_map_idx: usize = 0;
    let mut curr_file_id: u16 = 0;

    while disk_map_idx < disk_map.len() {
        for _ in 0..disk_map[disk_map_idx] {
            blocks.push(curr_file_id as i16);
        }

        disk_map_idx += 1;
        if disk_map_idx < disk_map.len() {
            // mark empty blocks with -1
            for _ in 0..disk_map[disk_map_idx] {
                blocks.push(-1);
            }
        }

        disk_map_idx += 1;
        curr_file_id += 1;
    }

    blocks
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
