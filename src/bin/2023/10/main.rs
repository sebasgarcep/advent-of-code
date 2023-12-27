extern crate lib;

use lib::reader::read_lines;

const GROUND: u8 = 0;
const WEST: u8 = 1;
const EAST: u8 = 2;
const SOUTH: u8 = 4;
const NORTH: u8 = 8;
const START: u8 = 15;

pub fn main() {
    first();
    second();
}

fn first() {
    solve();
}

fn second() {}

fn solve() {
    let line_collection: Vec<String> = read_lines("data/2023/10/input.txt").collect();

    let width = line_collection[0].len();
    let height = line_collection.len();

    let mut map: Vec<Vec<u8>> = vec![vec![0; height]; width];
    let mut start_position: Option<(usize, usize)> = Option::None;
    for j in 0..line_collection.len() {
        let current_line = &line_collection[j];
        for (i, c) in current_line.chars().enumerate() {
            map[i][j] = match c {
                '|' => SOUTH | NORTH,
                '-' => WEST | EAST,
                'L' => EAST | NORTH,
                'J' => WEST | NORTH,
                '7' => WEST | SOUTH,
                'F' => EAST | SOUTH,
                'S' => {
                    start_position = Option::Some((i, j));
                    START
                }
                '.' => GROUND,
                _ => {
                    unreachable!();
                }
            };
        }
    }
    let start_position = start_position.unwrap();

    let mut candidates: Vec<(usize, usize, u8)> = Vec::with_capacity(4);
    if start_position.0 > 0 && map[start_position.0 - 1][start_position.1] & EAST != 0 {
        candidates.push((start_position.0 - 1, start_position.1, EAST));
    }
    if start_position.0 < width - 1 && map[start_position.0 + 1][start_position.1] & WEST != 0 {
        candidates.push((start_position.0 + 1, start_position.1, WEST));
    }
    if start_position.1 > 0 && map[start_position.0][start_position.1 - 1] & SOUTH != 0 {
        candidates.push((start_position.0, start_position.1 - 1, SOUTH));
    }
    if start_position.1 < height - 1 && map[start_position.0][start_position.1 + 1] & NORTH != 0 {
        candidates.push((start_position.0, start_position.1 + 1, NORTH));
    }

    for (mut i, mut j, mut source) in candidates {
        let mut size: i64 = 0;
        while map[i][j] != START && map[i][j] != GROUND {
            size += 1;
            let next = map[i][j] ^ source;
            match next {
                WEST => {
                    i -= 1;
                    source = EAST;
                }
                EAST => {
                    i += 1;
                    source = WEST;
                }
                SOUTH => {
                    j += 1;
                    source = NORTH;
                }
                NORTH => {
                    j -= 1;
                    source = SOUTH;
                }
                _ => {
                    unreachable!();
                }
            };
        }
        if map[i][j] == START {
            let result = (size + 2) >> 1;
            println!("{}", result);
            return;
        }
    }

    unreachable!();
}
