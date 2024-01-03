extern crate lib;

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
            next_arrangement.extend(arrangement.iter());
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
            let (arrangement, hints) = Self::parse_line(line);
            result += Self::get_result(&arrangement, &hints);
        }
        println!("{}", result);
    }

    fn get_result(arrangement: &Vec<Symbol>, hints: &Vec<usize>) -> usize {
        let max_hint_count = hints.len();
        let max_block_size = *hints.iter().max().unwrap();
        let mut prev_state = vec![vec![0; max_block_size + 1]; hints.len() + 1];
        let mut curr_state = vec![vec![0; max_block_size + 1]; hints.len() + 1];
        for (idx, symbol) in arrangement.iter().enumerate() {
            (prev_state, curr_state) = (curr_state, prev_state);
            Self::clear_vec(&mut curr_state);
            if idx == 0 {
                prev_state[0][0] = 1;
            }
            Self::propagate_state(
                &hints,
                max_hint_count,
                max_block_size,
                &prev_state,
                &mut curr_state,
                *symbol,
            );
        }
        return curr_state[hints.len()][0] + curr_state[hints.len() - 1][hints[hints.len() - 1]];
    }

    fn clear_vec(data: &mut Vec<Vec<usize>>) {
        for j in 0..data.len() {
            for i in 0..data[j].len() {
                data[j][i] = 0;
            }
        }
    }

    fn propagate_state(
        hints: &Vec<usize>,
        max_hint_count: usize,
        max_block_size: usize,
        prev_state: &Vec<Vec<usize>>,
        mut curr_state: &mut Vec<Vec<usize>>,
        symbol: Symbol,
    ) {
        match symbol {
            Symbol::Operational => {
                for i in 0..=max_hint_count {
                    for j in 0..=max_block_size {
                        if j > 0 {
                            if i >= max_hint_count || hints[i] != j {
                                continue;
                            }
                            curr_state[i + 1][0] += prev_state[i][j];
                        } else {
                            curr_state[i][0] += prev_state[i][j];
                        }
                    }
                }
            }
            Symbol::Broken => {
                for i in 0..=max_hint_count {
                    for j in 0..max_block_size {
                        curr_state[i][j + 1] += prev_state[i][j];
                    }
                }
            }
            Symbol::Unknown => {
                Self::propagate_state(
                    hints,
                    max_hint_count,
                    max_block_size,
                    &prev_state,
                    &mut curr_state,
                    Symbol::Operational,
                );
                Self::propagate_state(
                    hints,
                    max_hint_count,
                    max_block_size,
                    &prev_state,
                    &mut curr_state,
                    Symbol::Broken,
                );
            }
        }
    }
}
