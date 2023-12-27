extern crate lib;

use lib::reader::read_lines;

const EMPTY: u8 = 0;
const ROUND: u8 = 1;
const CUBE: u8 = 2;

pub fn main() {
    first();
    second();
}

fn first() {
    solve();
}

fn second() {
}

fn solve() {
    let line_collection: Vec<String> = read_lines("data/2023/14/input.txt").collect();

    let mut map: Vec<Vec<u8>> = line_collection.into_iter().map(
        |l| l.chars().map(|c| match c {
            '.' => EMPTY,
            'O' => ROUND,
            '#' => CUBE,
            _ => {
                unreachable!();
            },
        }).collect()
    ).collect();

    let mut result: i64 = 0;
    for j in 0..map.len() {
        for i in 0..map[j].len() {
            let mut curr = j;
            while curr > 0 && map[curr-1][i] == EMPTY && map[curr][i] == ROUND {
                map[curr - 1][i] = ROUND;
                map[curr][i] = EMPTY;
                curr -= 1;
            }
            if map[curr][i] == ROUND {
                result += (map.len() as i64) - (curr as i64);
            }
        }
    }

    println!("{}", result);
}
