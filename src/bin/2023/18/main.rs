extern crate lib;

use lib::reader::read_lines;

pub fn main() {
    first();
    second();
}

fn first() {
    solve();
}

fn second() {}

enum Direction {
    North,
    West,
    South,
    East,
}

fn solve() {
    let line_collection: Vec<String> = read_lines("data/2023/18/input.txt").collect();

    // Find size of field
    /*
    let mut min_i: i64 = 0;
    let mut min_j: i64 = 0;
    let mut max_i: i64 = 0;
    let mut max_j: i64 = 0;
    let mut i: i64 = 0;
    let mut j: i64 = 0;
    for line in line_collection.iter() {
        let (direction, steps) = parse_line(line);
        match direction {
            Direction::North => {
                j -= steps;
                min_j = std::cmp::min(min_j, j);
            }
            Direction::West => {
                i -= steps;
                min_i = std::cmp::min(min_i, i);
            }
            Direction::South => {
                j += steps;
                max_j = std::cmp::max(max_j, j);
            }
            Direction::East => {
                i += steps;
                max_i = std::cmp::max(max_i, i);
            }
        }
    }
    */

    // Calculate area using shoelace trapezoid formula
    // let width: i64 = max_i - min_i + 1;
    // let height: i64 = max_j - min_j + 1;
    // let start_i: i64 = -min_i;
    // let start_j: i64 = -min_j;
    let start_j: i64 = 0;
    // let mut i: i64 = start_i;
    let mut j: i64 = start_j;
    let mut area: i64 = 0;
    let mut boundary: i64 = 0;
    for line in line_collection.iter() {
        let (direction, steps) = parse_line(line);
        boundary += steps;
        match direction {
            Direction::North => {
                j -= steps;
            }
            Direction::West => {
                /* Simplified from 1/2 * (y_1 + y_0) * (x_1 - x_0) */
                area -= j * steps;
                // i -= steps;
            }
            Direction::South => {
                j += steps;
            }
            Direction::East => {
                /* Simplified from 1/2 * (y_1 + y_0) * (x_1 - x_0) */
                area += j * steps;
                // i += steps;
            }
        }
    }

    // Calculate result using Pick's theorem
    let result = boundary + area.abs() - (boundary >> 1) + 1;
    println!("{}", result);
}

fn parse_line(line: &str) -> (Direction, i64) {
    let direction = match line.as_bytes()[0] as char {
        'U' => Direction::North,
        'L' => Direction::West,
        'D' => Direction::South,
        'R' => Direction::East,
        _ => unreachable!(),
    };
    let first_split = line.find(' ').unwrap();
    let second_split = first_split + 1 + line[(first_split + 1)..].find(' ').unwrap();
    let steps: i64 = line[(first_split + 1)..second_split].parse().unwrap();
    return (direction, steps);
}
