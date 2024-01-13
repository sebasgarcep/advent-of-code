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
    fn clear_reached(reached: &mut Vec<Vec<bool>>) {
        let grid_size: usize = reached.len();
        for j in 0..grid_size {
            for i in 0..grid_size {
                reached[j][i] = false;
            }
        }
    }

    fn count_reached(
        map: &Vec<Vec<bool>>,
        start_position: (usize, usize),
        num_steps: usize,
    ) -> usize {
        let grid_size = map.len();
        let mut prev_reached = vec![vec![false; grid_size]; grid_size];
        let mut curr_reached = prev_reached.clone();
        curr_reached[start_position.1][start_position.0] = true;
        for _ in 0..num_steps {
            std::mem::swap(&mut prev_reached, &mut curr_reached);
            Self::clear_reached(&mut curr_reached);
            for j in 0..grid_size {
                for i in 0..grid_size {
                    if i > 0 && map[j][i - 1] {
                        curr_reached[j][i - 1] |= prev_reached[j][i];
                    }
                    if i < grid_size - 1 && map[j][i + 1] {
                        curr_reached[j][i + 1] |= prev_reached[j][i];
                    }
                    if j > 0 && map[j - 1][i] {
                        curr_reached[j - 1][i] |= prev_reached[j][i];
                    }
                    if j < grid_size - 1 && map[j + 1][i] {
                        curr_reached[j + 1][i] |= prev_reached[j][i];
                    }
                }
            }
        }

        let mut num_reached = 0;
        for j in 0..grid_size {
            for i in 0..grid_size {
                if curr_reached[j][i] {
                    num_reached += 1;
                }
            }
        }
        return num_reached;
    }
}

impl Solver for SecondSolver {
    /// We make the following assumptions:
    /// 1. The grid is square.
    /// 2. The start position is right in the middle of the map.
    /// 3. The horizontal/vertical path from the start position to the edge of
    /// the map doesn't have obstacles.
    /// 4. The edges of the map are empty.
    /// 5. The square is filled in the same amount of steps it takes to reach
    /// the furthest corner.
    /// We also make the following unchecked assumptions:
    /// 1. Any group of unreachable empty spaces in the grid is of size 1.
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
    /// 26501365 mod grid_size == (grid_size - 1) / 2.
    fn get_result(map: Vec<Vec<bool>>, start_position: (usize, usize)) -> usize {
        let width = map[0].len();
        let height = map.len();

        // First assumption
        assert!(width == height);
        let grid_size = width;

        // Second assumption
        assert!(start_position == (grid_size >> 1, grid_size >> 1));

        // Third assumption
        for p in 0..grid_size {
            assert!(map[start_position.1][p]);
            assert!(map[p][start_position.0]);
        }

        // Fourth assumption
        for p in 0..grid_size {
            assert!(map[0][p]);
            assert!(map[p][0]);
            assert!(map[grid_size - 1][p]);
            assert!(map[p][grid_size - 1]);
        }

        // Fifth assumption
        let mut num_tiles: Vec<usize> = vec![0, 0];
        for i in 0..grid_size {
            for j in 0..grid_size {
                if map[j][i]
                    && (j == 0
                        || map[j - 1][i]
                        || j == grid_size - 1
                        || map[j + 1][i]
                        || i == 0
                        || map[j][i - 1]
                        || i == grid_size - 1
                        || map[j][i + 1])
                {
                    num_tiles[(i + j) & 1] += 1;
                }
            }
        }
        let full_size = grid_size - 1;
        let half_size = grid_size >> 1;

        let checks = vec![
            (start_position, full_size),
            ((0, 0), 2 * full_size),
            ((0, grid_size - 1), 2 * full_size),
            ((grid_size - 1, 0), 2 * full_size),
            ((grid_size - 1, grid_size - 1), 2 * full_size),
            ((0, grid_size >> 1), 2 * full_size + half_size),
            ((grid_size >> 1, 0), 2 * full_size + half_size),
            ((grid_size - 1, grid_size >> 1), 2 * full_size + half_size),
            ((grid_size >> 1, grid_size - 1), 2 * full_size + half_size),
        ];
        for (position, steps) in checks {
            assert!(Self::count_reached(&map, position, steps) == num_tiles[0]);
            assert!(Self::count_reached(&map, position, steps - 1) == num_tiles[1]);
        }

        // Solve the problem now
        let num_steps: usize = 26501365;
        let num_jumps: usize = num_steps / grid_size;

        // The reachable area is a rhombus with full squares in the middle, corners,
        // and the edges are composed of smaller and bigger triangles.
        let small_triangles = [
            Self::count_reached(&map, (0, 0), half_size - 1),
            Self::count_reached(&map, (grid_size - 1, 0), half_size - 1),
            Self::count_reached(&map, (0, grid_size - 1), half_size - 1),
            Self::count_reached(&map, (grid_size - 1, grid_size - 1), half_size - 1),
        ];

        let big_triangles = [
            Self::count_reached(&map, (0, 0), full_size + half_size),
            Self::count_reached(&map, (grid_size - 1, 0), full_size + half_size),
            Self::count_reached(&map, (0, grid_size - 1), full_size + half_size),
            Self::count_reached(&map, (grid_size - 1, grid_size - 1), full_size + half_size),
        ];

        let corners = [
            Self::count_reached(&map, (grid_size >> 1, 0), full_size),
            Self::count_reached(&map, (0, grid_size >> 1), full_size),
            Self::count_reached(&map, (grid_size - 1, grid_size >> 1), full_size),
            Self::count_reached(&map, (grid_size >> 1, grid_size - 1), full_size),
        ];

        // The number of steps is odd, so the parity of the center square will be odd.
        let mut counts = vec![0, 1];
        for idx in 0..num_jumps {
            // The size of each ring is 4 * (how far it is from the center square)
            counts[(idx + 1) & 1] += 4 * idx;
        }

        return num_tiles
            .iter()
            .zip(counts.iter())
            .map(|(nt, ct)| nt * ct)
            .sum::<usize>()
            // There is a small triangle in between each pair of big triangle or
            // between a pair with a big triangle and a corner. Therefore there
            // must be one more small triangle than a big triangle.
            + (num_jumps - 1) * big_triangles.iter().sum::<usize>()
            + num_jumps * small_triangles.iter().sum::<usize>()
            // Just 4 corners
            + corners.iter().sum::<usize>();
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
