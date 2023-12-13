extern crate lib;

use lib::reader::read_lines;

const SKIP_SEED_LINE: usize = 7;
const RANGES_CAPACITY: usize = 64;

pub fn main() {
    let mut line_iterator = read_lines("./data/2023/05/input.txt").peekable();

    let mut seeds: Vec<i64> = line_iterator.next().unwrap()[SKIP_SEED_LINE..]
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    while line_iterator.peek().is_some() {
        let _ = line_iterator.next();
        let _ = line_iterator.next();
        let mut ranges: Vec<(i64, i64, i64)> = Vec::with_capacity(RANGES_CAPACITY);
        while line_iterator.peek().is_some() && !line_iterator.peek().unwrap().is_empty() {
            let line = line_iterator.next().unwrap();
            let mut parts = line.split_whitespace();
            let first: i64 = parts.next().unwrap().parse().unwrap();
            let second: i64 = parts.next().unwrap().parse().unwrap();
            let third: i64 = parts.next().unwrap().parse().unwrap();
            let current_range: (i64, i64, i64) = (first, second, third);
            ranges.push(current_range);
        }
        for i in 0..seeds.len() {
            for current_range in ranges.iter() {
                let remainder = seeds[i] - current_range.1;
                if remainder >= 0 && remainder < current_range.2 {
                    seeds[i] = current_range.0 + remainder;
                    break;
                }
            }
        }
    }

    let result = *seeds.iter().min().unwrap();
    println!("{}", result);
}
