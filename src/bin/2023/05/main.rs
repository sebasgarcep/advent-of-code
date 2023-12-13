extern crate lib;

use lib::reader::read_lines;

const SKIP_SEED_LINE: usize = 7;
const RANGES_CONTAINER_CAPACITY: usize = 8;
const RANGES_CAPACITY: usize = 64;

pub fn main() {
    first();
    second();
}

fn read_input() -> (Vec<i64>, Vec<Vec<(i64, i64, i64)>>) {
    let mut line_iterator = read_lines("./data/2023/05/input.txt").peekable();

    let seeds: Vec<i64> = line_iterator.next().unwrap()[SKIP_SEED_LINE..]
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    let mut ranges_container: Vec<Vec<(i64, i64, i64)>> =
        Vec::with_capacity(RANGES_CONTAINER_CAPACITY);

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
        ranges_container.push(ranges);
    }

    return (seeds, ranges_container);
}

fn first() {
    let (seeds, ranges_container) = read_input();

    let result = seeds
        .iter()
        .map(|&v| (v, v))
        .map(|r| propagate_range(&ranges_container, r, 0, 0))
        .min()
        .unwrap();

    println!("{}", result);
}

fn second() {
    let (seeds, ranges_container) = read_input();

    let result = seeds
        .chunks(2)
        .map(|chunk| {
            return match chunk {
                &[a, b] => (a, b - 1),
                _ => unreachable!(),
            };
        })
        .map(|r| propagate_range(&ranges_container, r, 0, 0))
        .min()
        .unwrap();

    println!("{}", result);
}

fn propagate_range(
    ranges_container: &Vec<Vec<(i64, i64, i64)>>,
    range: (i64, i64),
    pos: usize,
    start: usize,
) -> i64 {
    if pos >= ranges_container.len() {
        return range.0;
    }
    let ranges = &ranges_container[pos];
    for ridx in start..ranges.len() {
        let transformation_range = ranges[ridx];
        let lower_bound = transformation_range.1;
        let upper_bound = transformation_range.1 + transformation_range.2 - 1;
        if lower_bound <= range.0 && range.1 <= upper_bound {
            let prop_range = get_propagated_range(range, transformation_range);
            return propagate_range(&ranges_container, prop_range, pos + 1, 0);
        } else if range.0 <= lower_bound && lower_bound <= range.1 && range.1 <= upper_bound {
            let stay_range = (range.0, lower_bound - 1);
            let prop_range = get_propagated_range((lower_bound, range.1), transformation_range);
            return std::cmp::min(
                propagate_range(&ranges_container, stay_range, pos, ridx + 1),
                propagate_range(&ranges_container, prop_range, pos + 1, 0),
            );
        } else if lower_bound <= range.0 && range.0 <= upper_bound && upper_bound <= range.1 {
            let stay_range = (upper_bound + 1, range.1);
            let prop_range = get_propagated_range((range.0, upper_bound), transformation_range);
            return std::cmp::min(
                propagate_range(&ranges_container, stay_range, pos, ridx + 1),
                propagate_range(&ranges_container, prop_range, pos + 1, 0),
            );
        } else if range.0 <= lower_bound && upper_bound <= range.1 {
            let left_range = (range.0, lower_bound - 1);
            let prop_range = get_propagated_range((lower_bound, upper_bound), transformation_range);
            let right_range = (upper_bound + 1, range.1);
            return std::cmp::min(
                propagate_range(&ranges_container, left_range, pos, ridx + 1),
                std::cmp::min(
                    propagate_range(&ranges_container, prop_range, pos + 1, 0),
                    propagate_range(&ranges_container, right_range, pos, ridx + 1),
                ),
            );
        }
    }

    return propagate_range(&ranges_container, range, pos + 1, 0);
}

fn get_propagated_range(range: (i64, i64), transformation_range: (i64, i64, i64)) -> (i64, i64) {
    return (
        transformation_range.0 + range.0 - transformation_range.1,
        transformation_range.0 + range.1 - transformation_range.1,
    );
}