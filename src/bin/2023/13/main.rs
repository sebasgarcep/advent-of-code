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
        result += get_result(pattern);
    }
    println!("{}", result);
}

fn get_result(pattern: &Vec<Vec<bool>>) -> i64 {
    for j in 1..pattern.len() {
        if test_mirror_horizontal(pattern, j) {
            return (j as i64) * 100;
        }
    }
    for i in 1..pattern[0].len() {
        if test_mirror_vertical(pattern, i) {
            return i as i64;
        }
    }
    return 0;
}

fn test_mirror_horizontal(pattern: &Vec<Vec<bool>>, j: usize) -> bool {
    let num_rows = std::cmp::min(j, pattern.len() - j);
    for k in 0..num_rows {
        if !equal_rows(pattern, j-k-1, j+k) {
            return false;
        }
    }
    return true;
}

fn test_mirror_vertical(pattern: &Vec<Vec<bool>>, i: usize) -> bool {
    let num_cols = std::cmp::min(i, pattern[0].len() - i);
    for k in 0..num_cols {
        if !equal_cols(pattern, i-k-1, i+k) {
            return false;
        }
    }
    return true;
}

fn equal_rows(pattern: &Vec<Vec<bool>>, row_a: usize, row_b: usize) -> bool {
    for i in 0..pattern[0].len() {
        if pattern[row_a][i] != pattern[row_b][i] {
            return false;
        }
    }
    return true;
}

fn equal_cols(pattern: &Vec<Vec<bool>>, col_a: usize, col_b: usize) -> bool {
    for j in 0..pattern.len() {
        if pattern[j][col_a] != pattern[j][col_b] {
            return false;
        }
    }
    return true;
}