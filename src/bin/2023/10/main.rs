extern crate lib;

use lib::reader::read_lines;

const GROUND: u8 = 0;
const WEST: u8 = 1;
const EAST: u8 = 2;
const SOUTH: u8 = 4;
const NORTH: u8 = 8;
const START: u8 = 15;

pub fn main() {
    first();
    second();
}

fn first() {
    let mut solver = FirstSolver::new();
    solve(&mut solver);
}

struct FirstSolver {
    size: i64,
}

impl FirstSolver {
    pub fn new() -> Self {
        return FirstSolver { size: 0 };
    }
}

impl Solver for FirstSolver {
    fn init_candidate(&mut self, _i: usize, _j: usize) {
        self.size = 0;
    }

    fn consume_step(&mut self, _i: usize, _j: usize) {
        self.size += 1;
    }

    fn get_result(&mut self) -> i64 {
        return (self.size + 1) >> 1;
    }
}

fn second() {
    let mut solver = SecondSolver::new();
    solve(&mut solver);
}

struct SecondSolver {
    double_area: i64,
    steps: i64,
    previous: (i64, i64),
}

impl SecondSolver {
    pub fn new() -> Self {
        return SecondSolver {
            double_area: 0,
            steps: 0,
            previous: (-1, -1),
        };
    }
}

impl Solver for SecondSolver {
    fn init_candidate(&mut self, i: usize, j: usize) {
        self.double_area = 0;
        self.steps = 0;
        self.previous = (i as i64, j as i64);
    }

    /*
    The problem reduces to counting the number of lattice points within the polygon
    but not on the perimeter. This can be calculated using Pick's theorem:
    i = A - b/2 + 1, where A is the area of the polygon, b is the number of integer
    points in the boundaty and i is the number of interior lattice points.
     */
    fn consume_step(&mut self, i: usize, j: usize) {
        /* Use the shoelace trapezoid formula to calculate area. Instead of computing halfs, compute
           double the amount and divide by half at the end. */
        let value = ((j as i64) + self.previous.1) * ((i as i64) - self.previous.0);
        self.double_area += value;
        self.steps += 1;
        self.previous = (i as i64, j as i64);
    }

    fn get_result(&mut self) -> i64 {
        /* Use absolute value to cover for clockwise/counter-clockwise traversal of the loop. */
        return (self.double_area >> 1).abs() - (self.steps >> 1) + 1;
    }
}

trait Solver {
    fn init_candidate(&mut self, i: usize, j: usize);
    fn consume_step(&mut self, i: usize, j: usize);
    fn get_result(&mut self) -> i64;
}

fn solve<S: Solver>(solver: &mut S) {
    let line_collection: Vec<String> = read_lines("data/2023/10/input.txt").collect();

    let width = line_collection[0].len();
    let height = line_collection.len();

    let mut map: Vec<Vec<u8>> = vec![vec![0; height]; width];
    let mut start_position: Option<(usize, usize)> = Option::None;
    for j in 0..line_collection.len() {
        let current_line = &line_collection[j];
        for (i, c) in current_line.chars().enumerate() {
            map[i][j] = match c {
                '|' => SOUTH | NORTH,
                '-' => WEST | EAST,
                'L' => EAST | NORTH,
                'J' => WEST | NORTH,
                '7' => WEST | SOUTH,
                'F' => EAST | SOUTH,
                'S' => {
                    start_position = Option::Some((i, j));
                    START
                }
                '.' => GROUND,
                _ => {
                    unreachable!();
                }
            };
        }
    }
    let start_position = start_position.unwrap();

    let mut candidates: Vec<(usize, usize, u8)> = Vec::with_capacity(4);
    if start_position.0 > 0 && map[start_position.0 - 1][start_position.1] & EAST != 0 {
        candidates.push((start_position.0 - 1, start_position.1, EAST));
    }
    if start_position.0 < width - 1 && map[start_position.0 + 1][start_position.1] & WEST != 0 {
        candidates.push((start_position.0 + 1, start_position.1, WEST));
    }
    if start_position.1 > 0 && map[start_position.0][start_position.1 - 1] & SOUTH != 0 {
        candidates.push((start_position.0, start_position.1 - 1, SOUTH));
    }
    if start_position.1 < height - 1 && map[start_position.0][start_position.1 + 1] & NORTH != 0 {
        candidates.push((start_position.0, start_position.1 + 1, NORTH));
    }

    for (mut i, mut j, mut source) in candidates {
        solver.init_candidate(start_position.0, start_position.1);
        while map[i][j] != START && map[i][j] != GROUND {
            solver.consume_step(i, j);
            let next = map[i][j] ^ source;
            match next {
                WEST => {
                    i -= 1;
                    source = EAST;
                }
                EAST => {
                    i += 1;
                    source = WEST;
                }
                SOUTH => {
                    j += 1;
                    source = NORTH;
                }
                NORTH => {
                    j -= 1;
                    source = SOUTH;
                }
                _ => {
                    break;
                }
            };
        }
        if map[i][j] == START {
            solver.consume_step(i, j);
            let result = solver.get_result();
            println!("{}", result);
            return;
        }
    }

    unreachable!();
}
