extern crate lib;

use itertools::Itertools;
use lib::reader::read_lines;

pub fn main() {
    first();
    second();
}

fn first() {
    solve();
}

fn second() {}

fn solve() {
    let line_collection = read_lines("data/2023/12/input.txt");

    let mut result: i64 = 0;
    for mut line in line_collection {
        let split_position = line.find(' ').unwrap();
        let hints = line
            .split_off(split_position)
            .trim()
            .split(',')
            .map(|x| x.parse::<i64>().unwrap())
            .collect_vec();
        let arrangement: Vec<i64> = line
            .chars()
            .map(|c| match c {
                '.' => 0,
                '#' => 1,
                '?' => -1,
                _ => unreachable!(),
            })
            .collect_vec();
        result += get_result(&arrangement, &hints);
    }

    println!("{}", result);
}

fn get_result(arrangement: &Vec<i64>, hints: &Vec<i64>) -> i64 {
    let mut result: i64 = 0;
    let count_unknowns = arrangement.iter().filter(|&&x| x == -1).count() as i64;
    let mut candidate_arrangement = arrangement.clone();
    /* Flatten recursion into a for loop where each candidate arrangement is
       defined by a number in the range [0, 2^N-1]. */
    for i in 0..((1 as i64) << count_unknowns) {
        let mut acc = i;
        for i in 0..arrangement.len() {
            if arrangement[i] == -1 {
                candidate_arrangement[i] = acc & 1;
                acc >>= 1;
            }
        }
        if matches_hints(&candidate_arrangement, &hints) {
            result += 1;
        }
    }
    return result;
}

fn matches_hints(arrangement: &Vec<i64>, hints: &Vec<i64>) -> bool {
    let mut count: i64 = 0;
    let mut curr_hint: usize = 0;
    for i in 0..arrangement.len() {
        if arrangement[i] == 1 {
            if curr_hint >= hints.len() {
                return false;
            }
            count += 1;
        }
        if count > 0 && arrangement[i] == 0 {
            if count != hints[curr_hint] {
                return false;
            }
            count = 0;
            curr_hint += 1;
        }
    }
    if count > 0 {
        if count != hints[curr_hint] {
            return false;
        }
        // count = 0;
        curr_hint += 1;
    }
    return curr_hint == hints.len();
}
