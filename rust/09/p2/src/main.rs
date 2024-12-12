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
    let mut file_stack = get_file_stack(&blocks);

    while !file_stack.is_empty() {
        let file_id = file_stack.pop().unwrap();
        let (file_l, file_r) = find_file_bounds(&blocks, file_id);
        let size = file_r - file_l + 1;
        if let Some(free_l) = find_free_space(&blocks, file_l, size) {
            for i in 0..size {
                blocks.swap(file_l + i, free_l + i);
            }
        }
    }
}

fn get_file_stack(blocks: &Vec<i16>) -> Vec<i16> {
    let mut file_stack: Vec<i16> = Vec::new();
    let mut i: usize = 0;
    let mut curr_file_id: i16 = -1;

    while i < blocks.len() {
        if blocks[i] != -1 {
            if blocks[i] != curr_file_id {
                file_stack.push(blocks[i]);
                curr_file_id = blocks[i];
            }
        }

        i += 1;
    }

    file_stack
}

fn find_free_space(blocks: &Vec<i16>, file_start: usize, size: usize) -> Option<usize> {
    let mut l: usize = 0;

    while l < file_start {
        while l < file_start && blocks[l] != -1 {
            l += 1;
        }

        let mut r = l + 1;
        while r < file_start && blocks[r] == -1 {
            r += 1;
        }

        if r - l >= size {
            return Some(l);
        }

        l = r;
    }

    None
}

// finds bounds of the next file to consider
fn find_file_bounds(blocks: &Vec<i16>, file_id: i16) -> (usize, usize) {
    let mut l: usize = 0;

    while blocks[l] != file_id {
        l += 1;
    }

    let mut r: usize = blocks.len() - 1;

    while blocks[r] != file_id {
        r -= 1;
    }

    (l, r)
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
