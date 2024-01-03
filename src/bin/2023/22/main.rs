extern crate lib;

use std::collections::VecDeque;

use bit_set::BitSet;
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
    fn get_result(
        bricks: &Vec<Brick>,
        dependants: &Vec<BitSet>,
        dependencies: &Vec<BitSet>,
    ) -> usize {
        return (0..bricks.len())
            .filter(|a| {
                for b in dependants[*a].iter() {
                    if dependencies[b]
                        .difference(&BitSet::from_iter(vec![*a].into_iter()))
                        .collect::<BitSet>()
                        .is_empty()
                    {
                        return false;
                    }
                }
                return true;
            })
            .count();
    }
}

fn second() {
    SecondSolver::solve();
}

enum SecondSolver {}

impl Solver for SecondSolver {
    fn get_result(
        bricks: &Vec<Brick>,
        dependants: &Vec<BitSet>,
        dependencies: &Vec<BitSet>,
    ) -> usize {
        return (0..bricks.len())
            .map(|a| {
                let mut removed_elems = BitSet::with_capacity(bricks.len());
                let mut queue = VecDeque::new();
                queue.push_back(a);
                while let Option::Some(elem) = queue.pop_front() {
                    removed_elems.insert(elem);
                    for b in dependants[elem].iter() {
                        if removed_elems.contains(b)
                            || !dependencies[b]
                                .difference(&removed_elems)
                                .collect::<BitSet>()
                                .is_empty()
                        {
                            continue;
                        }
                        queue.push_back(b);
                    }
                }
                return removed_elems.len() - 1;
            })
            .sum();
    }
}

#[derive(Debug)]
struct Brick {
    start: (usize, usize, usize),
    end: (usize, usize, usize),
}

impl Brick {
    pub fn from_line(mut line: String) -> Self {
        let split_position = line.find('~').unwrap();
        let right = line.split_off(split_position + 1);
        let _ = line.split_off(split_position);
        let left = line;
        let mut left_split = left.split(',');
        let mut right_split = right.split(',');
        return Self {
            start: (
                left_split.next().unwrap().parse::<usize>().unwrap(),
                left_split.next().unwrap().parse::<usize>().unwrap(),
                left_split.next().unwrap().parse::<usize>().unwrap() - 1, // Remap height to be zero indexed
            ),
            end: (
                right_split.next().unwrap().parse::<usize>().unwrap(),
                right_split.next().unwrap().parse::<usize>().unwrap(),
                right_split.next().unwrap().parse::<usize>().unwrap() - 1, // Remap height to be zero indexed
            ),
        };
    }
}

trait Solver {
    fn get_result(
        bricks: &Vec<Brick>,
        dependants: &Vec<BitSet>,
        dependencies: &Vec<BitSet>,
    ) -> usize;

    fn solve() {
        let mut bricks = read_lines("data/2023/22/input.txt")
            .map(Brick::from_line)
            .collect_vec();

        let space_width = bricks
            .iter()
            .map(|b| std::cmp::max(b.start.0, b.end.0))
            .max()
            .unwrap()
            + 1;
        let space_depth = bricks
            .iter()
            .map(|b| std::cmp::max(b.start.1, b.end.1))
            .max()
            .unwrap()
            + 1;
        let space_height = bricks
            .iter()
            .map(|b| std::cmp::max(b.start.2, b.end.2))
            .max()
            .unwrap()
            + 1;

        // We don't care about the initial order of the bricks
        bricks.sort_by_key(|b| std::cmp::min(b.start.2, b.end.2));
        let mut space: Vec<Vec<Vec<usize>>> =
            vec![vec![vec![usize::MAX; space_height]; space_depth]; space_width];
        let mut dependants: Vec<BitSet> = vec![BitSet::new(); bricks.len()];
        let mut dependencies: Vec<BitSet> = vec![BitSet::new(); bricks.len()];

        for (idx, brick) in bricks.iter().enumerate() {
            let min_x = std::cmp::min(brick.start.0, brick.end.0);
            let max_x = std::cmp::max(brick.start.0, brick.end.0);
            let min_y = std::cmp::min(brick.start.1, brick.end.1);
            let max_y = std::cmp::max(brick.start.1, brick.end.1);
            let min_z = std::cmp::min(brick.start.2, brick.end.2);
            let max_z = std::cmp::max(brick.start.2, brick.end.2);
            // Insert brick into space
            for i in min_x..=max_x {
                for j in min_y..=max_y {
                    for k in min_z..=max_z {
                        if space[i][j][k] != usize::MAX {
                            unreachable!();
                        }
                        space[i][j][k] = idx;
                    }
                }
            }
            // Drop brick
            let mut curr_min_z = min_z;
            while Self::can_drop(&space, min_x, max_x, min_y, max_y, curr_min_z) {
                let curr_max_z = max_z - (min_z - curr_min_z);

                for i in min_x..=max_x {
                    for j in min_y..=max_y {
                        space[i][j][curr_min_z - 1] = idx;
                        space[i][j][curr_max_z] = usize::MAX;
                    }
                }

                curr_min_z -= 1;
            }
            // Mark as dependant
            if curr_min_z > 0 {
                for i in min_x..=max_x {
                    for j in min_y..=max_y {
                        let dependency = space[i][j][curr_min_z - 1];
                        if dependency == usize::MAX {
                            continue;
                        }
                        dependants[dependency].insert(idx);
                        dependencies[idx].insert(dependency);
                    }
                }
            }
        }

        let result = Self::get_result(&bricks, &dependants, &dependencies);
        println!("{}", result);
    }

    fn can_drop(
        space: &Vec<Vec<Vec<usize>>>,
        min_x: usize,
        max_x: usize,
        min_y: usize,
        max_y: usize,
        min_z: usize,
    ) -> bool {
        if min_z == 0 {
            return false;
        }
        for i in min_x..=max_x {
            for j in min_y..=max_y {
                if space[i][j][min_z - 1] != usize::MAX {
                    return false;
                }
            }
        }
        return true;
    }
}
