extern crate lib;

use lib::reader::read_split;

pub fn main() {
    first();
    second();
}

fn first() {
    let mut solver = FirstSolver::new();
    solve(&mut solver);
}

struct FirstSolver {
    total: i64,
}

impl FirstSolver {
    pub fn new() -> Self {
        return Self { total: 0 };
    }
}

impl Solver for FirstSolver {
    fn consume(&mut self, line: String) {
        self.total += hash_line(&line);
    }

    fn get_result(&self) -> i64 {
        return self.total;
    }
}

fn hash_line(line: &str) -> i64 {
    let mut hash: i64 = 0;
    for char in line.chars() {
        hash += char as i64;
        hash *= 17;
        hash = hash % 256;
    }
    return hash;
}

fn second() {
    let mut solver = SecondSolver::new();
    solve(&mut solver);
}

enum Operation {
    Insert(i64),
    Remove,
}

struct SecondSolver {
    boxes: Vec<Vec<(String, i64)>>,
}

impl SecondSolver {
    pub fn new() -> Self {
        return Self {
            boxes: vec![vec![]; 256],
        };
    }

    fn get_operation(&self, line: &str) -> (String, Operation) {
        let limit = line.chars().position(|c| c == '=' || c == '-').unwrap();
        let label = line[0..limit].to_owned();
        return match line.as_bytes()[limit] as char {
            '=' => (
                label,
                Operation::Insert(line.as_bytes()[limit + 1] as i64 - '0' as i64),
            ),
            '-' => (label, Operation::Remove),
            _ => {
                unreachable!();
            }
        };
    }
}

impl Solver for SecondSolver {
    fn consume(&mut self, line: String) {
        let (label, operation) = self.get_operation(&line);
        let hash = hash_line(&label) as usize;

        let maybe_position = self.boxes[hash]
            .iter()
            .position(|(item_label, _)| *item_label == label);

        match operation {
            Operation::Insert(focal_length) => {
                if let Some(position) = maybe_position {
                    self.boxes[hash][position].1 = focal_length;
                } else {
                    self.boxes[hash].push((label, focal_length));
                }
            }
            Operation::Remove => {
                if let Some(position) = maybe_position {
                    self.boxes[hash].remove(position);
                }
            }
        }
    }

    fn get_result(&self) -> i64 {
        return self
            .boxes
            .iter()
            .enumerate()
            .map(|(box_num, box_contents)| {
                box_contents
                    .iter()
                    .enumerate()
                    .map(|(slot_num, slot_contents)| {
                        (box_num as i64 + 1) * (slot_num as i64 + 1) * slot_contents.1
                    })
                    .sum::<i64>()
            })
            .sum();
    }
}

trait Solver {
    fn consume(&mut self, line: String);
    fn get_result(&self) -> i64;
}

fn solve<S: Solver>(solver: &mut S) {
    let line_collection: Vec<String> = read_split("data/2023/15/input.txt", ',').collect();

    for line in line_collection {
        solver.consume(line);
    }
    let result = solver.get_result();
    println!("{}", result);
}
