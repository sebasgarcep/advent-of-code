extern crate lib;

use itertools::Itertools;
use lib::reader::read_lines;

pub fn main() {
    first();
    second();
}

fn first() {
    solve(0);
}

fn second() {
    solve(1)
}

fn solve(num_errors: i64) {
    let mut line_collection = read_lines("data/2023/13/input.txt");

    /* Indexed patterns[n][j][i] */
    let mut patterns: Vec<Vec<Vec<bool>>> = vec![];
    /* Indexed current_pattern[j][i] */
    let mut current_pattern: Vec<Vec<bool>> = vec![];
    while let Some(line) = line_collection.next() {
        if line.is_empty() {
            let insert_pattern = std::mem::replace(&mut current_pattern, vec![]);
            patterns.push(insert_pattern);
            continue;
        }
        let current_row = line
            .chars()
            .map(|c| match c {
                '#' => true,
                '.' => false,
                _ => unreachable!(),
            })
            .collect_vec();
        current_pattern.push(current_row);
    }
    patterns.push(current_pattern);

    let mut result: i64 = 0;
    for pattern in patterns.iter() {
        result += get_result(pattern, num_errors);
    }
    println!("{}", result);
}

fn get_result(pattern: &Vec<Vec<bool>>, num_errors: i64) -> i64 {
    for j in 1..pattern.len() {
        if test_mirror_horizontal(pattern, num_errors, j) {
            return (j as i64) * 100;
        }
    }
    for i in 1..pattern[0].len() {
        if test_mirror_vertical(pattern, num_errors, i) {
            return i as i64;
        }
    }
    return 0;
}

fn test_mirror_horizontal(pattern: &Vec<Vec<bool>>, num_errors: i64, j: usize) -> bool {
    let mut curr_errors: i64 = 0;
    let num_rows = std::cmp::min(j, pattern.len() - j);
    for k in 0..num_rows {
        for i in 0..pattern[0].len() {
            if pattern[j - k - 1][i] != pattern[j + k][i] {
                curr_errors += 1;
                if curr_errors > num_errors {
                    return false;
                }
            }
        }
    }
    return num_errors == curr_errors;
}

fn test_mirror_vertical(pattern: &Vec<Vec<bool>>, num_errors: i64, i: usize) -> bool {
    let mut curr_errors: i64 = 0;
    let num_cols = std::cmp::min(i, pattern[0].len() - i);
    for k in 0..num_cols {
        for j in 0..pattern.len() {
            if pattern[j][i - k - 1] != pattern[j][i + k] {
                curr_errors += 1;
                if curr_errors > num_errors {
                    return false;
                }
            }
        }
    }
    return num_errors == curr_errors;
}
