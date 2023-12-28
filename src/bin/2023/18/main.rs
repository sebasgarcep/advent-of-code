extern crate lib;

use lib::reader::read_lines;

pub fn main() {
    first();
    second();
}

fn first() {
    solve::<FirstSolver>();
}

struct FirstSolver;

impl Solver for FirstSolver {
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
}

fn second() {
    solve::<SecondSolver>();
}

struct SecondSolver;

impl Solver for SecondSolver {
    fn parse_line(line: &str) -> (Direction, i64) {
        let hex_start = line.find('#').unwrap() + 1;
        let direction = match line.as_bytes()[hex_start + 5] as char {
            '0' => Direction::East,
            '1' => Direction::South,
            '2' => Direction::West,
            '3' => Direction::North,
            _ => unreachable!(),
        };
        let steps: i64 = i64::from_str_radix(&line[hex_start..(hex_start + 5)], 16).unwrap();
        return (direction, steps);
    }
}

enum Direction {
    North,
    West,
    South,
    East,
}

trait Solver {
    fn parse_line(line: &str) -> (Direction, i64);
}

fn solve<S: Solver>() {
    let line_collection: Vec<String> = read_lines("data/2023/18/input.txt").collect();

    let mut j: i64 = 0;
    let mut area: i64 = 0;
    let mut boundary: i64 = 0;
    for line in line_collection.iter() {
        let (direction, steps) = S::parse_line(&line);
        boundary += steps;
        match direction {
            Direction::North => {
                j -= steps;
            }
            Direction::West => {
                /* Simplified from 1/2 * (y_1 + y_0) * (x_1 - x_0) */
                area -= j * steps;
            }
            Direction::South => {
                j += steps;
            }
            Direction::East => {
                /* Simplified from 1/2 * (y_1 + y_0) * (x_1 - x_0) */
                area += j * steps;
            }
        }
    }

    // Calculate result using Pick's theorem
    let result = boundary + area.abs() - (boundary >> 1) + 1;
    println!("{}", result);
}
