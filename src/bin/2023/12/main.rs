extern crate lib;

use std::time::Instant;

use itertools::Itertools;
use lib::reader::read_lines;

pub fn main() {
    first();
    second();
}

fn first() {
    solve::<FirstSolver>();
}

enum FirstSolver {}

impl Solver for FirstSolver {
    fn parse_line(mut line: String) -> (Vec<i8>, Vec<usize>) {
        let split_position = line.find(' ').unwrap();
        let hints = line
            .split_off(split_position)
            .trim()
            .split(',')
            .map(|x| x.parse::<usize>().unwrap())
            .collect_vec();
        let arrangement: Vec<i8> = line
            .chars()
            .map(|c| match c {
                '.' => 0,
                '#' => 1,
                '?' => -1,
                _ => unreachable!(),
            })
            .collect_vec();
        return (arrangement, hints);
    }
}

fn second() {
    solve::<SecondSolver>();
}

enum SecondSolver {}

impl Solver for SecondSolver {
    fn parse_line(line: String) -> (Vec<i8>, Vec<usize>) {
        let (arrangement, hints) = FirstSolver::parse_line(line);
        let mut next_arrangement = Vec::with_capacity(5 * arrangement.len() + 4);
        let mut next_hints = Vec::with_capacity(5 * arrangement.len());
        for i in 0..5 {
            next_hints.extend(hints.iter());
            next_arrangement.extend(arrangement.iter());
            if i < 4 {
                next_arrangement.push(-1);
            }
        }
        return (next_arrangement, next_hints);
    }
}

trait Solver {
    fn parse_line(line: String) -> (Vec<i8>, Vec<usize>);
}

fn solve<S: Solver>() {
    let line_collection = read_lines("data/2023/12/input.txt");

    let mut result: usize = 0;
    for line in line_collection {
        let (arrangement, hints) = S::parse_line(line);
        result += get_result(&arrangement, &hints);
    }

    println!("{}", result);
}

fn get_result(arrangement: &Vec<i8>, hints: &Vec<usize>) -> usize {
    let start_time = Instant::now();
    let num_broken_total: usize = hints.iter().sum();
    let num_broken_known: usize = arrangement.iter().filter(|&&x| x == 1).count();
    let num_available = num_broken_total - num_broken_known;
    println!(
        "Meta: {} {} {}",
        num_broken_total, num_broken_known, num_available,
    );
    let result = dfs(arrangement, hints, num_available, 0, 0, 0);
    let end_time = Instant::now();
    let elapsed_time = end_time - start_time;
    println!("Duration: {:.2}", elapsed_time.as_secs_f64());
    return result;
}

fn dfs(
    arrangement: &Vec<i8>,
    hints: &Vec<usize>,
    num_available: usize,
    pos: usize,
    count: usize,
    curr_hint: usize,
) -> usize {
    // Sanity check
    if pos > arrangement.len() {
        unreachable!();
    }

    if pos == arrangement.len() {
        return dfs_edge_case(hints, num_available, count, curr_hint);
    }

    let val = arrangement[pos];
    if val != -1 {
        return dfs_known_case(
            arrangement,
            hints,
            num_available,
            pos,
            count,
            curr_hint,
            val,
        );
    }

    return dfs_known_case(arrangement, hints, num_available, pos, count, curr_hint, 0)
        + if num_available > 0 {
            dfs_known_case(
                arrangement,
                hints,
                num_available - 1,
                pos,
                count,
                curr_hint,
                1,
            )
        } else {
            0
        };
}

fn dfs_edge_case(
    hints: &Vec<usize>,
    num_available: usize,
    count: usize,
    mut curr_hint: usize,
) -> usize {
    if count > 0 {
        if count != hints[curr_hint] {
            return 0;
        }
        curr_hint += 1;
    }
    return if num_available == 0 && curr_hint == hints.len() {
        1
    } else {
        0
    };
}

fn dfs_known_case(
    arrangement: &Vec<i8>,
    hints: &Vec<usize>,
    num_available: usize,
    pos: usize,
    count: usize,
    curr_hint: usize,
    val: i8,
) -> usize {
    if val == 1 {
        if curr_hint >= hints.len() {
            return 0;
        }
        return dfs(
            arrangement,
            hints,
            num_available,
            pos + 1,
            count + 1,
            curr_hint,
        );
    } else if val == 0 {
        if count == 0 {
            return dfs(arrangement, hints, num_available, pos + 1, count, curr_hint);
        }
        if count != hints[curr_hint] {
            return 0;
        }
        return dfs(arrangement, hints, num_available, pos + 1, 0, curr_hint + 1);
    } else {
        // Sanity check
        unreachable!();
    }
}
