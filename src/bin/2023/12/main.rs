extern crate lib;

use std::time::Instant;

use itertools::Itertools;
use lib::reader::read_lines;

pub fn main() {
    first();
    second();
}

fn first() {
    FirstSolver::solve();
}

enum FirstSolver {}

impl Solver for FirstSolver {
    fn parse_line(mut line: String) -> (Vec<Symbol>, Vec<usize>) {
        let split_position = line.find(' ').unwrap();
        let hints = line
            .split_off(split_position)
            .trim()
            .split(',')
            .map(|x| x.parse::<usize>().unwrap())
            .collect_vec();
        let arrangement = line
            .chars()
            .map(|c| match c {
                '.' => Symbol::Operational,
                '#' => Symbol::Broken,
                '?' => Symbol::Unknown,
                _ => unreachable!(),
            })
            .collect_vec();
        return (arrangement, hints);
    }
}

fn second() {
    SecondSolver::solve();
}

enum SecondSolver {}

impl Solver for SecondSolver {
    fn parse_line(line: String) -> (Vec<Symbol>, Vec<usize>) {
        let (arrangement, hints) = FirstSolver::parse_line(line);
        let mut next_arrangement = Vec::with_capacity(5 * arrangement.len() + 4);
        let mut next_hints = Vec::with_capacity(5 * arrangement.len());
        for i in 0..5 {
            next_hints.extend(hints.iter());
            next_arrangement.extend(arrangement.clone());
            if i < 4 {
                next_arrangement.push(Symbol::Unknown);
            }
        }
        return (next_arrangement, next_hints);
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum Symbol {
    Operational,
    Broken,
    Unknown,
}

trait Solver {
    fn parse_line(line: String) -> (Vec<Symbol>, Vec<usize>);

    fn solve() {
        let line_collection = read_lines("data/2023/12/input.txt");

        let mut result: usize = 0;
        for line in line_collection {
            println!("{}", line);
            let (arrangement, hints) = Self::parse_line(line);
            let start_time: Instant = Instant::now();
            result += Self::get_result(&arrangement, &hints);
            let end_time = Instant::now();
            let elapsed_time = end_time - start_time;
            println!("Duration: {:.2}", elapsed_time.as_secs_f64());
        }

        println!("{}", result);
    }

    fn get_result(arrangement: &Vec<Symbol>, hints: &Vec<usize>) -> usize {
        let num_broken_total: usize = hints.iter().sum();
        let num_broken_known: usize = arrangement.iter().filter(|&&x| x == Symbol::Broken).count();
        let num_available = arrangement.iter().filter(|&&x| x == Symbol::Unknown).count();
        let num_available_broken = num_broken_total - num_broken_known;
        let num_available_operational = num_available - num_available_broken;
        let result = Self::dfs(
            arrangement,
            hints,
            num_available_broken,
            num_available_operational,
            0,
            0,
            0,
        );
        return result;
    }

    fn dfs(
        arrangement: &Vec<Symbol>,
        hints: &Vec<usize>,
        num_available_broken: usize,
        num_available_operational: usize,
        pos: usize,
        count: usize,
        curr_hint: usize,
    ) -> usize {
        // Sanity check
        if pos > arrangement.len() {
            unreachable!();
        }

        if pos == arrangement.len() {
            return Self::dfs_edge_case(
                hints,
                num_available_broken,
                num_available_operational,
                count,
                curr_hint,
            );
        }

        let val = arrangement[pos];
        if val != Symbol::Unknown {
            return Self::dfs_known_case(
                arrangement,
                hints,
                num_available_broken,
                num_available_operational,
                pos,
                count,
                curr_hint,
                val,
            );
        }

        return if num_available_operational > 0 {
            Self::dfs_known_case(
                arrangement,
                hints,
                num_available_broken,
                num_available_operational - 1,
                pos,
                count,
                curr_hint,
                Symbol::Operational,
            )
        } else {
            0
        } + if num_available_broken > 0 {
            Self::dfs_known_case(
                arrangement,
                hints,
                num_available_broken - 1,
                num_available_operational,
                pos,
                count,
                curr_hint,
                Symbol::Broken,
            )
        } else {
            0
        };
    }

    fn dfs_edge_case(
        hints: &Vec<usize>,
        num_available_broken: usize,
        num_available_operational: usize,
        count: usize,
        mut curr_hint: usize,
    ) -> usize {
        if count > 0 {
            if count != hints[curr_hint] {
                return 0;
            }
            curr_hint += 1;
        }
        return if num_available_broken == 0
            && num_available_operational == 0
            && curr_hint == hints.len()
        {
            1
        } else {
            0
        };
    }

    fn dfs_known_case(
        arrangement: &Vec<Symbol>,
        hints: &Vec<usize>,
        num_available_broken: usize,
        num_available_operational: usize,
        pos: usize,
        count: usize,
        curr_hint: usize,
        val: Symbol,
    ) -> usize {
        if val == Symbol::Broken {
            if curr_hint >= hints.len() {
                return 0;
            }
            return Self::dfs(
                arrangement,
                hints,
                num_available_broken,
                num_available_operational,
                pos + 1,
                count + 1,
                curr_hint,
            );
        } else if val == Symbol::Operational {
            if count == 0 {
                return Self::dfs(
                    arrangement,
                    hints,
                    num_available_broken,
                    num_available_operational,
                    pos + 1,
                    count,
                    curr_hint,
                );
            }
            if count != hints[curr_hint] {
                return 0;
            }
            return Self::dfs(
                arrangement,
                hints,
                num_available_broken,
                num_available_operational,
                pos + 1,
                0,
                curr_hint + 1,
            );
        } else {
            // Sanity check
            unreachable!();
        }
    }
}
