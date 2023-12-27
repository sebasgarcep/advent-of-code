extern crate lib;

use lib::reader::read_lines;

const SIZE_HINT: usize = 1024;

pub fn main() {
    first();
    second();
}

fn first() {
    solve(2);
}

fn second() {
    solve(1000000);
}

fn solve(empty_galaxy_size: usize) {
    let line_collection: Vec<String> = read_lines("data/2023/11/input.txt").collect();

    let mut empty_cols: Vec<bool> = Vec::with_capacity(SIZE_HINT);
    let mut empty_rows: Vec<bool> = Vec::with_capacity(SIZE_HINT);
    for (j, line) in line_collection.iter().enumerate() {
        empty_rows.push(true);
        for (i, char) in line.chars().enumerate() {
            if j == 0 {
                empty_cols.push(true);
            }
            match char {
                '.' => {}
                '#' => {
                    empty_cols[i] = false;
                    empty_rows[j] = false;
                }
                _ => {
                    unreachable!();
                }
            }
        }
    }

    let mut i: usize;
    let mut j: usize = 0;

    let mut galaxies: Vec<(usize, usize)> = Vec::with_capacity(SIZE_HINT);
    for (real_j, line) in line_collection.iter().enumerate() {
        i = 0;
        for (real_i, char) in line.chars().enumerate() {
            match char {
                '.' => {}
                '#' => {
                    galaxies.push((i, j));
                }
                _ => {
                    unreachable!();
                }
            }
            if empty_cols[real_i] {
                i += empty_galaxy_size;
            } else {
                i += 1;
            }
        }
        if empty_rows[real_j] {
            j += empty_galaxy_size;
        } else {
            j += 1;
        }
    }

    let mut result: i64 = 0;
    for b in 1..galaxies.len() {
        for a in 0..b {
            let (gax, gay) = galaxies[a];
            let (gbx, gby) = galaxies[b];
            let distance =
                ((gax as i64) - (gbx as i64)).abs() + ((gay as i64) - (gby as i64)).abs();
            result += distance;
        }
    }

    println!("{}", result);
}
