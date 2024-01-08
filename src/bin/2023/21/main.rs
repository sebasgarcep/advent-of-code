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

impl SecondSolver {
    fn clear_reached(reached: &mut Vec<Vec<bool>>, width: usize, height: usize) {
        for j in 0..height {
            for i in 0..width {
                reached[j][i] = false;
            }
        }
    }

    fn calculate_steps(
        map: &Vec<Vec<bool>>,
        width: usize,
        height: usize,
        start_position: (usize, usize),
        max_num_steps: usize,
    ) -> (usize, usize) {
        let mut prev_reached = vec![vec![false; width]; height];
        let mut curr_reached = prev_reached.clone();
        curr_reached[start_position.1][start_position.0] = true;
        let mut prev_size = 0;
        let mut curr_size = 0;
        let mut steps = 0;
        for _ in 0..max_num_steps {
            std::mem::swap(&mut prev_reached, &mut curr_reached);
            Self::clear_reached(&mut curr_reached, width, height);
            curr_size = 0;
            for j in 0..height {
                for i in 0..width {
                    if i > 0 && map[j][i - 1] {
                        curr_reached[j][i - 1] |= prev_reached[j][i];
                    }
                    if i < width - 1 && map[j][i + 1] {
                        curr_reached[j][i + 1] |= prev_reached[j][i];
                    }
                    if j > 0 && map[j - 1][i] {
                        curr_reached[j - 1][i] |= prev_reached[j][i];
                    }
                    if j < width - 1 && map[j + 1][i] {
                        curr_reached[j + 1][i] |= prev_reached[j][i];
                    }
                }
            }
            for j in 0..height {
                for i in 0..width {
                    if curr_reached[j][i] {
                        curr_size += 1;
                    }
                }
            }
            steps += 1;
            if max_num_steps == usize::MAX && steps % 1 != 0 {
                continue;
            }
            if max_num_steps == usize::MAX && prev_size == curr_size {
                break;
            }
            prev_size = curr_size;
        }
        return (steps - 2, curr_size);
    }
}

impl Solver for SecondSolver {
    /// We make the following assumptions:
    /// 1. The grid is square.
    /// 2. The start position is right in the middle of the map.
    /// 2. The horizontal/vertical path from the start position to the edge of
    /// the map doesn't have obstacles.
    /// 3. The edges of the map are empty.
    /// 4. From the start
    /// Therefore:
    /// 1. Color the map like a chessboard. If the elf starts from a white tile
    /// then after an odd number of steps it will be at a black tile and after
    /// an even number of steps it will be a white tile.
    /// 2. Because the map is based around the manhattan distance, and because
    /// the edges and straight paths from the start position are empty, then we
    /// will enter a copy of the input grid at either the corners or the middles
    /// of the edges. Furthermore, any path coming in later into the copy of the
    /// grid will not reach a cell before the path coming in from the corners or
    /// the middle of te edges, so we only need to consider how the path propagates
    /// from the start position, corners and middle of edges, depending on where
    /// the copy of the grid is in the map relative to the starting grid.
    /// 3. The number of reachable cells is monotonically increasing (separating
    /// the white tile subsequence from the black tile subsequence), as you can
    /// always go backwards/forwards to ensure a tile is always reached again after
    /// N steps.
    /// 4. The number of steps we need to take is 26501365 which satisfies
    /// 26501365 mod grid_size == (grid_size - 1) / 2. Also, the number of steps
    /// is odd, so we only need to find the max number of reachable black tiles
    /// in the grid. The number of steps to fill a
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
