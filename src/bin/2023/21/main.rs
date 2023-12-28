extern crate lib;

use std::collections::HashSet;

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
    let line_collection = read_lines("data/2023/21/input.txt");

    let mut seed: Option<(i64, i64)> = Option::None;
    /* Indexed map[j][i] */
    let mut map: Vec<Vec<bool>> = Vec::with_capacity(1024);
    for (j, line) in line_collection.enumerate() {
        let mut row = Vec::with_capacity(line.len());
        for (i, char) in line.chars().enumerate() {
            match char {
                '.' => row.push(true),
                'S' => {
                    row.push(true);
                    seed = Option::Some((i as i64, j as i64));
                }
                '#' => row.push(false),
                _ => unreachable!(),
            };
        }
        map.push(row);
    }
    let seed = seed.unwrap();
    let width = map[0].len() as i64;
    let height = map.len() as i64;

    let positions = HashSet::from([seed]);
    let result = bfs(64, &map, width, height, positions, 0);
    println!("{}", result);
}

fn bfs(
    limit: i64,
    map: &Vec<Vec<bool>>,
    width: i64,
    height: i64,
    positions: HashSet<(i64, i64)>,
    step: i64,
) -> i64 {
    if step >= limit {
        return positions.len() as i64;
    }
    let mut next_positions: HashSet<(i64, i64)> = HashSet::with_capacity(256);
    for (i, j) in positions {
        propose_candidate(map, width, height, i - 1, j, &mut next_positions);
        propose_candidate(map, width, height, i + 1, j, &mut next_positions);
        propose_candidate(map, width, height, i, j - 1, &mut next_positions);
        propose_candidate(map, width, height, i, j + 1, &mut next_positions);
    }
    return bfs(limit, map, width, height, next_positions, step + 1);
}

fn propose_candidate(
    map: &Vec<Vec<bool>>,
    width: i64,
    height: i64,
    i: i64,
    j: i64,
    positions: &mut HashSet<(i64, i64)>,
) {
    if j < 0 || j >= height || i < 0 || i >= width || !map[j as usize][i as usize] {
        return;
    }
    positions.insert((i, j));
}
