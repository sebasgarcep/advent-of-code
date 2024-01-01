extern crate lib;

use bit_set::BitSet;
use itertools::Itertools;
use lib::reader::read_lines;
use std::collections::HashMap;

pub fn main() {
    first();
    second();
}

fn first() {
    solve::<FirstSolver>();
}

enum FirstSolver {}

impl Solver for FirstSolver {
    fn get_start_positions(graph: &Graph) -> Vec<usize> {
        let start = graph.get_index("AAA".as_bytes().try_into().unwrap());
        return vec![start];
    }

    fn get_can_stop(graph: &Graph) -> Vec<bool> {
        let stop = graph.get_index("ZZZ".as_bytes().try_into().unwrap());
        return (0..graph.nodes.len()).map(|i| i == stop).collect_vec();
    }
}

fn second() {
    // solve::<SecondSolver>();
}

enum SecondSolver {}

impl Solver for SecondSolver {
    fn get_start_positions(graph: &Graph) -> Vec<usize> {
        return (0..graph.nodes.len())
            .filter(|i| graph.nodes[*i].label[2] == 'A' as u8)
            .collect_vec();
    }

    fn get_can_stop(graph: &Graph) -> Vec<bool> {
        return graph
            .nodes
            .iter()
            .map(|node| node.label[2] == 'Z' as u8)
            .collect_vec();
    }
}

struct Graph {
    nodes: Vec<Node>,
    index_map: HashMap<[u8; 3], usize>,
}

struct Node {
    label: [u8; 3],
    left: usize,
    right: usize,
}

impl Graph {
    pub fn from_lines<I: Iterator<Item = String>>(line_iterator: I) -> Self {
        let mut curr_index_count: usize = 0;
        let mut index_map: HashMap<[u8; 3], usize> = HashMap::new();
        let mut nodes = Vec::new();

        for line in line_iterator {
            let line_bytes = line.as_bytes();
            let source: [u8; 3] = line_bytes[0..3].try_into().unwrap();
            if !index_map.contains_key(&source) {
                index_map.insert(source.clone(), curr_index_count);
                curr_index_count += 1;
            }
            let left: [u8; 3] = line_bytes[7..10].try_into().unwrap();
            if !index_map.contains_key(&left) {
                index_map.insert(left.clone(), curr_index_count);
                curr_index_count += 1;
            }
            let right: [u8; 3] = line_bytes[12..15].try_into().unwrap();
            if !index_map.contains_key(&right) {
                index_map.insert(right.clone(), curr_index_count);
                curr_index_count += 1;
            }
            while nodes.len() < curr_index_count {
                nodes.push(Node {
                    label: [0; 3],
                    left: 0,
                    right: 0,
                });
            }
            let current_node = &mut nodes[index_map[&source]];
            current_node.label = source;
            current_node.left = *index_map.get(&left).unwrap();
            current_node.right = *index_map.get(&right).unwrap();
        }

        return Self { nodes, index_map };
    }

    pub fn get_index(&self, label: &[u8; 3]) -> usize {
        return self.index_map[label];
    }

    pub fn get_next(&self, index: usize, direction: Direction) -> usize {
        let current_node = &self.nodes[index];
        return match direction {
            Direction::Left => current_node.left,
            Direction::Right => current_node.right,
        };
    }
}

#[derive(Clone, Copy)]
enum Direction {
    Left,
    Right,
}

#[derive(Debug)]
struct SolverMetadata {
    next: usize,
    stop_positions: BitSet,
}

impl SolverMetadata {
    pub fn get_metadata(
        graph: &Graph,
        instructions: &Vec<Direction>,
        can_stop: &Vec<bool>,
        index: usize,
    ) -> Self {
        let mut current = index;
        let mut stop_positions = BitSet::new();
        for (step, direction) in instructions.iter().enumerate() {
            if can_stop[current] {
                stop_positions.insert(step);
            }
            current = graph.get_next(index, *direction);
        }
        return Self {
            next: current,
            stop_positions,
        };
    }
}

trait Solver {
    fn get_start_positions(graph: &Graph) -> Vec<usize>;
    fn get_can_stop(graph: &Graph) -> Vec<bool>;
}

fn solve<S: Solver>() {
    let mut line_iterator = read_lines("data/2023/08/input.txt");
    let instructions = line_iterator
        .next()
        .unwrap()
        .chars()
        .map(|c| match c {
            'L' => Direction::Left,
            'R' => Direction::Right,
            _ => unreachable!(),
        })
        .collect_vec();

    let _ = line_iterator.next();
    let graph = Graph::from_lines(line_iterator);
    let can_stop = S::get_can_stop(&graph);
    let start_positions = S::get_start_positions(&graph);

    let metadata = (0..graph.nodes.len())
        .map(|index| SolverMetadata::get_metadata(&graph, &instructions, &can_stop, index))
        .collect_vec();
    let full_set = BitSet::from_iter(0..instructions.len());

    let mut result: usize = 0;
    let mut current_positions = start_positions.clone();
    let mut curr_intersection: BitSet;
    loop {
        curr_intersection = current_positions.iter().map(|p| &metadata[*p]).fold(
            full_set.clone(),
            |mut acc, item| {
                acc.intersect_with(&item.stop_positions);
                acc
            },
        );

        if !curr_intersection.is_empty() {
            break;
        }

        for i in 0..current_positions.len() {
            current_positions[i] = metadata[current_positions[i]].next;
        }
        result += instructions.len();
    }

    result += curr_intersection.iter().min().unwrap();

    println!("{}", result);
}
