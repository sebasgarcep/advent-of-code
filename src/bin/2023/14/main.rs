extern crate lib;

use lib::reader::read_lines;
use std::collections::hash_map::DefaultHasher;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::time::Instant;

const EMPTY: u8 = 0;
const ROUND: u8 = 1;
const CUBE: u8 = 2;

pub fn main() {
    first();
    second();
}

fn first() {
    let solver = FirstSolver::new();
    solve(&solver);
}

struct FirstSolver {}

impl FirstSolver {
    pub fn new() -> Self {
        return Self {};
    }
}

impl Solver for FirstSolver {
    fn transform(&self, map: &mut Vec<Vec<u8>>) {
        let height = map.len();
        let width = map[0].len();

        for j in 0..height {
            for i in 0..width {
                let mut curr = j;
                while curr > 0 && map[curr - 1][i] == EMPTY && map[curr][i] == ROUND {
                    map[curr - 1][i] = ROUND;
                    map[curr][i] = EMPTY;
                    curr -= 1;
                }
            }
        }
    }

    fn calculate_load(&self, map: &Vec<Vec<u8>>) -> i64 {
        let mut result: i64 = 0;
        for j in 0..map.len() {
            for i in 0..map[j].len() {
                if map[j][i] == ROUND {
                    result += (map.len() as i64) - (j as i64);
                }
            }
        }
        return result;
    }
}

fn second() {
    let solver = SecondSolver::new();
    solve(&solver);
}

trait Solver {
    fn transform(&self, map: &mut Vec<Vec<u8>>);
    fn calculate_load(&self, map: &Vec<Vec<u8>>) -> i64;
}

struct SecondSolver {}

impl SecondSolver {
    pub fn new() -> Self {
        return Self {};
    }

    fn tilt_north(&self, map: &mut Vec<Vec<u8>>, width: usize, height: usize) {
        for j in 0..height {
            for i in 0..width {
                let mut curr = j;
                while curr > 0 && map[curr - 1][i] == EMPTY && map[curr][i] == ROUND {
                    map[curr - 1][i] = ROUND;
                    map[curr][i] = EMPTY;
                    curr -= 1;
                }
            }
        }
    }

    fn tilt_west(&self, map: &mut Vec<Vec<u8>>, width: usize, height: usize) {
        for i in 0..width {
            for j in 0..height {
                let mut curr = i;
                while curr > 0 && map[j][curr - 1] == EMPTY && map[j][curr] == ROUND {
                    map[j][curr - 1] = ROUND;
                    map[j][curr] = EMPTY;
                    curr -= 1;
                }
            }
        }
    }

    fn tilt_south(&self, map: &mut Vec<Vec<u8>>, width: usize, height: usize) {
        for j in (0..height).rev() {
            for i in 0..width {
                let mut curr = j;
                while curr < map.len() - 1 && map[curr + 1][i] == EMPTY && map[curr][i] == ROUND {
                    map[curr + 1][i] = ROUND;
                    map[curr][i] = EMPTY;
                    curr += 1;
                }
            }
        }
    }

    fn tilt_east(&self, map: &mut Vec<Vec<u8>>, width: usize, height: usize) {
        for i in (0..width).rev() {
            for j in 0..height {
                let mut curr = i;
                while curr < map[j].len() - 1 && map[j][curr + 1] == EMPTY && map[j][curr] == ROUND
                {
                    map[j][curr + 1] = ROUND;
                    map[j][curr] = EMPTY;
                    curr += 1;
                }
            }
        }
    }

    fn calculate_hash<T: Hash>(&self, data: &T) -> u64 {
        let mut hasher = DefaultHasher::new();
        data.hash(&mut hasher);
        hasher.finish()
    }
}

impl Solver for SecondSolver {
    /*
    Find if there are loops, jump ahead using loop length.
     */
    fn transform(&self, map: &mut Vec<Vec<u8>>) {
        let start_time = Instant::now();

        let mut memory: HashMap<u64, usize> = HashMap::new();
        let height = map.len();
        let width = map[0].len();
        let limit = 1000000000;
        let mut step: usize = 0;
        let mut has_jumped = false;
        while step < limit {
            step += 1;
            self.tilt_north(map, width, height);
            self.tilt_west(map, width, height);
            self.tilt_south(map, width, height);
            self.tilt_east(map, width, height);
            let hash = self.calculate_hash(map);
            if !has_jumped && memory.contains_key(&hash) {
                let jump_size = step - memory[&hash];
                /* step + k * jump_size <= limit */
                let k = (limit - step) / jump_size;
                step += k * jump_size;
                has_jumped = true;
                continue;
            } else if !has_jumped {
                memory.insert(hash, step);
            }
        }

        let end_time = Instant::now();
        let elapsed_time = end_time - start_time;

        println!("Transform took {:.2}s", elapsed_time.as_secs_f64());
    }

    fn calculate_load(&self, map: &Vec<Vec<u8>>) -> i64 {
        let mut result: i64 = 0;
        for j in 0..map.len() {
            for i in 0..map[j].len() {
                if map[j][i] == ROUND {
                    result += (map.len() as i64) - (j as i64);
                }
            }
        }
        return result;
    }
}

fn solve<S: Solver>(solver: &S) {
    let line_collection: Vec<String> = read_lines("data/2023/14/input.txt").collect();

    let mut map: Vec<Vec<u8>> = line_collection
        .into_iter()
        .map(|l| {
            l.chars()
                .map(|c| match c {
                    '.' => EMPTY,
                    'O' => ROUND,
                    '#' => CUBE,
                    _ => {
                        unreachable!();
                    }
                })
                .collect()
        })
        .collect();

    solver.transform(&mut map);

    let result = solver.calculate_load(&map);
    println!("{}", result);
}
