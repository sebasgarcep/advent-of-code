extern crate lib;
extern crate priority_queue;

use itertools::Itertools;
use lib::reader::read_lines;
use priority_queue::PriorityQueue;

// Direction adresses
const DIRECTIONLESS: usize = 0;
const NORTH: usize = 1;
const WEST: usize = 2;
const SOUTH: usize = 3;
const EAST: usize = 4;

pub fn main() {
    first();
    second();
}

fn first() {
    solve();
}

fn second() {}

#[derive(Debug)]
struct NodeMetadata {
    distance: usize,
    previous: Option<(usize, usize, usize, usize)>,
}

impl NodeMetadata {
    pub fn new() -> Self {
        return Self {
            distance: 1000000000,
            previous: Option::None,
        };
    }

    pub fn get_priority(&self) -> usize {
        return usize::MAX - self.distance;
    }
}

fn solve() {
    let line_collection = read_lines("data/2023/17/input.txt");
    let grid = line_collection
        .map(|l| {
            l.chars()
                .map(|c| c.to_digit(10).unwrap() as usize)
                .collect_vec()
        })
        .collect_vec();

    let height = grid.len();
    let width = grid[0].len();
    let directions: usize = 5;
    let steps: usize = 3;

    // Dijkstra algorithm
    let mut grid_metadata = (0..height)
        .map(|_| {
            (0..width)
                .map(|_| {
                    (0..directions)
                        .map(|_| (0..steps).map(|_| NodeMetadata::new()).collect_vec())
                        .collect_vec()
                })
                .collect_vec()
        })
        .collect_vec();
    grid_metadata[0][0][DIRECTIONLESS][0].distance = 0;

    let mut queue = PriorityQueue::new();
    for j in 0..height {
        for i in 0..width {
            for d in 0..directions {
                for k in 0..steps {
                    queue.push((i, j, d, k), grid_metadata[j][i][d][k].get_priority());
                }
            }
        }
    }

    while let Some(((i, j, d, k), _)) = queue.pop() {
        let neighbours = get_neighbours(width, height, directions, steps, i, j, d, k);
        for (ni, nj, nd, nk) in neighbours {
            let alt = grid_metadata[j][i][d][k].distance + grid[nj][ni];
            let node_metadata = &mut grid_metadata[nj][ni][nd][nk];
            if alt < node_metadata.distance {
                node_metadata.distance = alt;
                node_metadata.previous = Option::Some((i, j, d, k));
                queue.change_priority(&(ni, nj, nd, nk), node_metadata.get_priority());
            }
        }
    }

    // println!("Result: {:?}", grid_metadata[width - 1][height - 1]);
    let mut min_distance: usize = usize::MAX;
    // let mut curr = (width - 1, height - 1, directions, steps);
    for d in 0..directions {
        for k in 0..steps {
            if grid_metadata[width - 1][height - 1][d][k].distance < min_distance {
                min_distance = grid_metadata[width - 1][height - 1][d][k].distance;
                // curr = (width - 1, height - 1, d, k);
            }
        }
    }

    /*
    println!(
        "Min distance: {:?}",
        grid_metadata[curr.1][curr.0][curr.2][curr.3].distance
    );
    println!("Path: {:?}", curr);
    while let Some((i, j, d, k)) = grid_metadata[curr.1][curr.0][curr.2][curr.3].previous {
        curr = (i, j, d, k);
        println!("Path: {:?}", curr);
    }
    */

    let result: usize = min_distance;
    println!("{}", result);
}

fn get_neighbours(
    width: usize,
    height: usize,
    _directions: usize,
    steps: usize,
    i: usize,
    j: usize,
    d: usize,
    k: usize,
) -> Vec<(usize, usize, usize, usize)> {
    let mut neighbours = Vec::with_capacity(4);
    // NORTH
    if j > 0 && d != SOUTH && (d != NORTH || k < steps - 1) {
        neighbours.push((i, j - 1, NORTH, if d == NORTH { k + 1 } else { 0 }));
    }
    // WEST
    if i > 0 && d != EAST && (d != WEST || k < steps - 1) {
        neighbours.push((i - 1, j, WEST, if d == WEST { k + 1 } else { 0 }));
    }
    // SOUTH
    if j < height - 1 && d != NORTH && (d != SOUTH || k < steps - 1) {
        neighbours.push((i, j + 1, SOUTH, if d == SOUTH { k + 1 } else { 0 }));
    }
    // EAST
    if i < width - 1 && d != WEST && (d != EAST || k < steps - 1) {
        neighbours.push((i + 1, j, EAST, if d == EAST { k + 1 } else { 0 }));
    }
    return neighbours;
}
