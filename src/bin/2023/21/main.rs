extern crate lib;

use lib::reader::read_lines;

pub fn main() {
    first();
    second();
}

fn first() {
    FirstSolver::solve();
}

enum FirstSolver {}

impl FirstSolver {
    fn clear_counts(counts: &mut Vec<Vec<usize>>) {
        for j in 0..counts.len() {
            for i in 0..counts[j].len() {
                counts[j][i] = 0;
            }
        }
    }
}

impl Solver for FirstSolver {
    fn get_result(map: Vec<Vec<bool>>, start_position: (usize, usize)) -> usize {
        let height = map.len();
        let width = map[0].len();

        let mut curr_counts: Vec<Vec<usize>> = vec![vec![0; width]; height];
        curr_counts[start_position.1][start_position.0] = 1;

        let mut prev_counts = curr_counts.clone();
        for _ in 0..64 {
            std::mem::swap(&mut prev_counts, &mut curr_counts);
            Self::clear_counts(&mut curr_counts);

            for j in 0..height {
                for i in 0..width {
                    if i > 0 && map[j][i - 1] {
                        curr_counts[j][i - 1] |= prev_counts[j][i];
                    }
                    if i < width - 1 && map[j][i + 1] {
                        curr_counts[j][i + 1] |= prev_counts[j][i];
                    }
                    if j > 0 && map[j - 1][i] {
                        curr_counts[j - 1][i] |= prev_counts[j][i];
                    }
                    if j < width - 1 && map[j + 1][i] {
                        curr_counts[j + 1][i] |= prev_counts[j][i];
                    }
                }
            }
        }

        return curr_counts
            .iter()
            .map(|row| row.iter().sum::<usize>())
            .sum::<usize>();
    }
}

fn second() {
    SecondSolver::solve();
}

enum SecondSolver {}

impl Solver for SecondSolver {
    fn get_result(map: Vec<Vec<bool>>, start_position: (usize, usize)) -> usize {
        return 0;
    }
}

trait Solver {
    fn get_result(map: Vec<Vec<bool>>, start_position: (usize, usize)) -> usize;

    fn solve() {
        /* Indexed map[j][i] */
        let (map, start_position) = Self::get_map();
        let result = Self::get_result(map, start_position);
        println!("{}", result);
    }

    fn get_map() -> (Vec<Vec<bool>>, (usize, usize)) {
        let line_collection = read_lines("data/2023/21/input.txt");
        let mut start_position = Option::None;
        let mut map: Vec<Vec<bool>> = vec![];
        for (j, line) in line_collection.enumerate() {
            let mut map_row = Vec::with_capacity(line.len());
            for (i, char) in line.chars().enumerate() {
                map_row.push(true);
                match char {
                    '.' => {}
                    'S' => start_position = Option::Some((i, j)),
                    '#' => map_row[i] = false,
                    _ => unreachable!(),
                };
            }
            map.push(map_row);
        }
        return (map, start_position.unwrap());
    }
}
