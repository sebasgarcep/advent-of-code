extern crate lib;
extern crate priority_queue;

use std::collections::HashMap;

use bit_set::BitSet;
use itertools::Itertools;
use lib::reader::read_lines;

pub fn main() {
    first();
    second();
}

fn first() {
    solve::<FirstSolver>();
}

enum FirstSolver {}

impl Solver for FirstSolver {
    fn parse_lines<I: Iterator<Item = String>>(line_collection: I) -> Vec<Vec<Tile>> {
        return line_collection
            .map(|l| l.chars().map(Tile::from_char).collect_vec())
            .collect_vec();
    }
}

fn second() {
    solve::<SecondSolver>();
}

enum SecondSolver {}

impl Solver for SecondSolver {
    fn parse_lines<I: Iterator<Item = String>>(line_collection: I) -> Vec<Vec<Tile>> {
        let grid = FirstSolver::parse_lines(line_collection);
        return grid
            .into_iter()
            .map(|row| {
                row.into_iter()
                    .map(|t| if t == Tile::Forest { t } else { Tile::Path })
                    .collect_vec()
            })
            .collect_vec();
    }
}

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

#[derive(Debug)]
struct CompressedGraph {
    source_index: usize,
    target_index: usize,
    nodes: Vec<CompressedGraphNode>,
}

#[derive(Debug)]
struct CompressedGraphNode {
    edges: Vec<CompressedGraphEdge>,
}

#[derive(Debug)]
struct CompressedGraphEdge {
    node_index: usize,
    length: usize,
}

impl CompressedGraph {
    pub fn get_longest_path(&self) -> usize {
        let visited = BitSet::new();
        return self.get_longest_path_from_node(self.source_index, &visited);
    }

    fn get_longest_path_from_node(&self, curr_index: usize, visited: &BitSet) -> usize {
        if curr_index == self.target_index {
            return 0;
        }
        let mut next_visited = visited.clone();
        next_visited.insert(curr_index);
        let node = &self.nodes[curr_index];
        return node
            .edges
            .iter()
            .filter(|edge| !next_visited.contains(edge.node_index))
            .map(|edge| {
                edge.length + self.get_longest_path_from_node(edge.node_index, &next_visited)
            })
            .max()
            .unwrap_or(0);
    }

    pub fn from_grid(
        grid: &Vec<Vec<Tile>>,
        width: usize,
        height: usize,
        source: (usize, usize),
        target: (usize, usize),
    ) -> Self {
        let mut index_map: HashMap<(usize, usize), usize> = HashMap::new();
        let mut nodes_map: HashMap<usize, (usize, usize)> = HashMap::new();
        let mut nodes = Vec::with_capacity(256);

        for j in 0..height {
            for i in 0..width {
                let curr = (i, j);
                if !Self::is_choice_node(grid, width, height, source, target, curr) {
                    continue;
                }
                index_map.insert(curr, nodes.len());
                nodes_map.insert(nodes.len(), curr);
                let node = CompressedGraphNode {
                    edges: Vec::with_capacity(4),
                };
                nodes.push(node);
            }
        }

        for (idx, node) in nodes.iter_mut().enumerate() {
            let curr = &nodes_map[&idx];
            node.edges.extend(
                Self::get_reachable_choice_nodes(grid, width, height, source, target, *curr)
                    .into_iter()
                    .map(|(pos, length)| CompressedGraphEdge {
                        node_index: index_map[&pos],
                        length,
                    }),
            );
        }

        return Self {
            source_index: index_map[&source],
            target_index: index_map[&target],
            nodes,
        };
    }

    fn get_reachable_choice_nodes(
        grid: &Vec<Vec<Tile>>,
        width: usize,
        height: usize,
        source: (usize, usize),
        target: (usize, usize),
        curr: (usize, usize),
    ) -> Vec<((usize, usize), usize)> {
        return Self::get_neighbours(grid, width, height, curr, false)
            .into_iter()
            .filter_map(|mut node| {
                let mut prev = curr;
                let mut length = 1;
                while !Self::is_choice_node(grid, width, height, source, target, node) {
                    length += 1;
                    let neighbours = Self::get_neighbours(grid, width, height, node, false);
                    let maybe_next = neighbours.into_iter().find(|&pos| pos != prev);
                    if let Option::Some(next) = maybe_next {
                        prev = node;
                        node = next;
                    } else {
                        return Option::None;
                    }
                }
                return Option::Some((node, length));
            })
            .collect_vec();
    }

    fn is_choice_node(
        grid: &Vec<Vec<Tile>>,
        width: usize,
        height: usize,
        source: (usize, usize),
        target: (usize, usize),
        curr: (usize, usize),
    ) -> bool {
        let neighbours = Self::get_neighbours(&grid, width, height, curr, true);
        return curr == source || curr == target || neighbours.len() > 2;
    }

    fn get_neighbours(
        grid: &Vec<Vec<Tile>>,
        width: usize,
        height: usize,
        curr: (usize, usize),
        allow_unreachable: bool,
    ) -> Vec<(usize, usize)> {
        return vec![
            Direction::North,
            Direction::West,
            Direction::South,
            Direction::East,
        ]
        .into_iter()
        .filter_map(|direction| {
            Self::get_neighbour_by_direction(width, height, curr, direction)
                .map(|pos| (direction, pos))
        })
        .filter(|(direction, pos)| match grid[pos.1][pos.0] {
            Tile::Path => true,
            Tile::Slope(pos_direction) => {
                allow_unreachable || pos_direction
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
}

trait Solver {
    fn parse_lines<I: Iterator<Item = String>>(line_collection: I) -> Vec<Vec<Tile>>;
}

fn solve<S: Solver>() {
    let line_collection = read_lines("data/2023/23/input.txt");
    let grid = S::parse_lines(line_collection);

    let height = grid.len();
    let width = grid[0].len();

    let source = (1, 0);
    let target = (width - 2, height - 1);

    let compressed_graph = CompressedGraph::from_grid(&grid, width, height, source, target);

    let result = compressed_graph.get_longest_path();
    println!("{}", result);
}
