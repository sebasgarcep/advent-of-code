extern crate lib;

use lib::reader::read_lines;

pub fn main() {
    first();
    second();
}

fn first() {
    let mut solver = FirstSolver::new();
    solve(&mut solver);
}

struct FirstSolver {
    extrapolated: i64,
}

impl FirstSolver {
    pub fn new() -> Self {
        return FirstSolver { extrapolated: 0 };
    }
}

impl Solver for FirstSolver {
    fn init_extrapolation(&mut self, _values: &Vec<i64>) {
        self.extrapolated = 0;
    }

    fn consume_values(&mut self, values: &Vec<i64>, size: usize) {
        self.extrapolated += values[size - 1];
    }

    fn extrapolate(&self) -> i64 {
        return self.extrapolated;
    }
}

fn second() {
    let mut solver = SecondSolver::new();
    solve(&mut solver);
}

struct SecondSolver {
    heads: Vec<i64>,
}

impl SecondSolver {
    pub fn new() -> Self {
        return SecondSolver {
            heads: vec![],
        };
    }
}

impl Solver for SecondSolver {
    fn init_extrapolation(&mut self, values: &Vec<i64>) {
        self.heads = Vec::with_capacity(values.len());
    }

    fn consume_values(&mut self, values: &Vec<i64>, _size: usize) {
        self.heads.push(values[0]);
    }

    fn extrapolate(&self) -> i64 {
        let mut extrapolated: i64 = 0;
        for &val in self.heads.iter().rev() {
            extrapolated = val - extrapolated;
        }
        return extrapolated;
    }
}

trait Solver {
    fn init_extrapolation(&mut self, values: &Vec<i64>);
    fn consume_values(&mut self, values: &Vec<i64>, size: usize);
    fn extrapolate(&self) -> i64;
}

fn solve<S: Solver>(solver: &mut S) {
    let line_iterator = read_lines("data/2023/09/input.txt");

    let mut result: i64 = 0;
    for line in line_iterator {
        let mut values: Vec<i64> = line.split(" ").map(|x| x.parse().unwrap()).collect();
        let mut differences: Vec<i64> = vec![0; values.len()];
        solver.init_extrapolation(&values);
        for size in (2..=values.len()).rev() {
            solver.consume_values(&values, size);
            let mut is_constant = true;
            for idx in 0..(size - 1) {
                let current = values[idx + 1] - values[idx];
                if current != 0 {
                    is_constant = false;
                }
                differences[idx] = current;
            }
            if is_constant {
                break;
            }
            (values, differences) = (differences, values);
        }
        result += solver.extrapolate();
    }

    println!("{}", result);
}
