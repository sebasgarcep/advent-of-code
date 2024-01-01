extern crate lib;

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
    fn get_result(graph: &Graph, instructions: &Vec<Direction>) -> usize {
        let stop = graph.get_index("ZZZ".as_bytes().try_into().unwrap());
        let mut smart_map: HashMap<usize, (usize, usize)> = HashMap::new();
        for start in 0..graph.nodes.len() {
            let mut num_steps = 0;
            let mut current = start.clone();
            for direction in instructions.iter() {
                if current == stop {
                    break;
                }
                num_steps += 1;
                current = graph.get_next(current, *direction);
            }
            smart_map.insert(start.clone(), (current, num_steps));
        }

        let mut result: usize = 0;
        let mut current = graph.get_index("AAA".as_bytes().try_into().unwrap());
        while current != stop {
            let (next, num_steps) = smart_map[&current];
            result += num_steps;
            current = next;
        }

        return result;
    }
}

fn second() {}

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

trait Solver {
    fn get_result(graph: &Graph, instructions: &Vec<Direction>) -> usize;
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

    let result = S::get_result(&graph, &instructions);
    println!("{}", result);
}
