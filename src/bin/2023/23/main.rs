extern crate lib;
extern crate priority_queue;

use itertools::Itertools;
use lib::reader::read_lines;

pub fn main() {
    first();
    second();
}

fn first() {
    solve();
}

fn second() {}

#[derive(PartialEq, Clone, Copy)]
enum Direction {
    North,
    West,
    South,
    East,
}

#[derive(PartialEq, Clone, Copy)]
enum Tile {
    Path,
    Forest,
    Slope(Direction),
}

impl Tile {
    pub fn from_char(char: char) -> Self {
        return match char {
            '.' => Tile::Path,
            '#' => Tile::Forest,
            '^' => Tile::Slope(Direction::North),
            '<' => Tile::Slope(Direction::West),
            'v' => Tile::Slope(Direction::South),
            '>' => Tile::Slope(Direction::East),
            _ => unreachable!(),
        };
    }
}

fn solve() {
    let line_collection = read_lines("data/2023/23/input.txt");
    let grid = line_collection
        .map(|l| l.chars().map(Tile::from_char).collect_vec())
        .collect_vec();

    let height = grid.len();
    let width = grid[0].len();

    let source = (1, 0);
    let target = (width - 2, height - 1);

    let mut visited = grid
        .iter()
        .map(|row| row.iter().map(|_| false).collect_vec())
        .collect_vec();
    visited[source.1][source.0] = true;

    let result: usize = find_longest_path(&grid, width, height, source, target, visited);
    println!("{}", result);
}

fn find_longest_path(
    grid: &Vec<Vec<Tile>>,
    width: usize,
    height: usize,
    source: (usize, usize),
    target: (usize, usize),
    mut visited: Vec<Vec<bool>>,
) -> usize {
    let mut curr = source;
    let mut size = 0;
    loop {
        // println!("curr={:?}", curr);
        let neighbours = get_neighbours(&grid, width, height, &visited, curr);
        // println!("neighbours={:?}", neighbours);
        match neighbours.len() {
            0 => {
                return 0;
            }
            1 => {
                curr = neighbours[0];
                size += 1;
                visited[curr.1][curr.0] = true;
                if curr == target {
                    return size;
                }
            }
            _ => {
                size += 1;
                return size + neighbours.into_iter().map(|pos| {
                    let pos_visited = visited.clone();
                    find_longest_path(grid, width, height, pos, target, pos_visited)
                }).max().unwrap();
            }
        };
    }
}

fn get_neighbours(
    grid: &Vec<Vec<Tile>>,
    width: usize,
    height: usize,
    visited: &Vec<Vec<bool>>,
    curr: (usize, usize),
) -> Vec<(usize, usize)> {
    return vec![
        Direction::North,
        Direction::West,
        Direction::South,
        Direction::East,
    ]
    .into_iter()
    .filter_map(|direction| {
        get_neighbour_by_direction(width, height, curr, direction).map(|pos| (direction, pos))
    })
    .filter(|(direction, pos)| match grid[pos.1][pos.0] {
        Tile::Path => true,
        Tile::Slope(pos_direction) => {
            pos_direction
                != match *direction {
                    Direction::North => Direction::South,
                    Direction::West => Direction::East,
                    Direction::South => Direction::North,
                    Direction::East => Direction::West,
                }
        }
        Tile::Forest => false,
    })
    .map(|(_, pos)| pos)
    .filter(|(i, j)| !visited[*j][*i])
    .collect();
}

fn get_neighbour_by_direction(
    width: usize,
    height: usize,
    curr: (usize, usize),
    direction: Direction,
) -> Option<(usize, usize)> {
    let (i, j) = curr;
    match direction {
        Direction::North => {
            if j > 0 {
                return Option::Some((i, j - 1));
            }
        }
        Direction::West => {
            if i > 0 {
                return Option::Some((i - 1, j));
            }
        }
        Direction::South => {
            if j < height - 1 {
                return Option::Some((i, j + 1));
            }
        }
        Direction::East => {
            if i < width - 1 {
                return Option::Some((i + 1, j));
            }
        }
    };

    return Option::None;
}
