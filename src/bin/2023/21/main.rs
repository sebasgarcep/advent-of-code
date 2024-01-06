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

impl Solver for FirstSolver {
    fn get_num_steps() -> usize {
        return 64;
    }
}

fn second() {}

trait Solver {
    fn get_num_steps() -> usize;

    fn solve() {
        let line_collection = read_lines("data/2023/21/input.txt");

        /* Indexed map[j][i] */
        let mut map: Vec<Vec<bool>> = vec![];
        let mut curr_counts: Vec<Vec<usize>> = vec![];
        for line in line_collection {
            let mut map_row = Vec::with_capacity(line.len());
            let mut curr_counts_row = Vec::with_capacity(line.len());
            for (i, char) in line.chars().enumerate() {
                map_row.push(true);
                curr_counts_row.push(0);
                match char {
                    '.' => {}
                    'S' => curr_counts_row[i] = 1,
                    '#' => map_row[i] = false,
                    _ => unreachable!(),
                };
            }
            map.push(map_row);
            curr_counts.push(curr_counts_row);
        }
        let height = map.len();
        let width = map[0].len();

        let mut prev_counts = curr_counts.clone();
        for _ in 0..Self::get_num_steps() {
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

        let result = curr_counts.iter().map(|row| row.iter().sum::<usize>()).sum::<usize>();
        println!("{}", result);
    }

    fn clear_counts(counts: &mut Vec<Vec<usize>>) {
        for j in 0..counts.len() {
            for i in 0..counts[j].len() {
                counts[j][i] = 0;
            }
        }
    }
}
