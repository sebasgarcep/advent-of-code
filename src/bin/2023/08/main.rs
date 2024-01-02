extern crate lib;
extern crate num;

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
    solve::<SecondSolver>();
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

#[derive(Debug)]
struct Graph {
    nodes: Vec<Node>,
    index_map: HashMap<[u8; 3], usize>,
}

#[derive(Debug)]
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

#[derive(Clone, Copy, Debug)]
enum Direction {
    Left,
    Right,
}

trait Solver {
    fn get_start_positions(graph: &Graph) -> Vec<usize>;
    fn get_can_stop(graph: &Graph) -> Vec<bool>;
}

#[derive(Debug)]
struct SolverMetadata {
    start_position: usize,
    stop_position: usize,
    steps: usize,
}

impl SolverMetadata {
    pub fn from_data(
        graph: &Graph,
        instructions: &Vec<Direction>,
        can_stop: &Vec<bool>,
        start_position: usize,
    ) -> Self {
        let mut steps = 0;
        let mut current_position = start_position;
        while steps == 0 || !can_stop[current_position] {
            let direction = instructions[steps % instructions.len()];
            current_position = graph.get_next(current_position, direction);
            steps += 1;
        }
        assert!(steps % instructions.len() == 0);
        return SolverMetadata {
            start_position,
            stop_position: current_position,
            steps,
        };
    }
}

/// We use the following assumptions to ensure the code is correct.
/// 1. The number of steps between a starting/stop position and a stop position
/// is a multiple of the number of instructions. This ensures stability when
/// mapping nodes forward through the graph.
/// 2. The only stop position reachable from a stop position is itself. This ensures
/// that a simple loop exists after reaching a stop position for the first time.
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

    // Get metadata
    let metadata = start_positions
        .iter()
        .map(|start_position| {
            let md_start =
                SolverMetadata::from_data(&graph, &instructions, &can_stop, *start_position);
            let md_stop =
                SolverMetadata::from_data(&graph, &instructions, &can_stop, md_start.stop_position);
            assert!(md_stop.start_position == md_stop.stop_position);
            return (md_start, md_stop);
        })
        .collect_vec();

    let min_steps = metadata
        .iter()
        .map(|(md_start, _)| md_start.steps)
        .max()
        .unwrap();

    // Get Align Metadata (gets rid of first metadata)
    let metadata = metadata
        .into_iter()
        .map(|(md_start, md_stop)| (md_stop.steps, min_steps % md_start.steps))
        .collect_vec();

    // LCM
    let lcm = metadata
        .iter()
        .fold(1, |acc, (steps, _)| num::integer::lcm(acc, *steps));

    // Align
    let min_step_size = instructions.len();
    let mut values = vec![min_step_size; metadata.len()];
    let mut align_steps = min_step_size;
    let mut step_size = min_step_size;
    while !values
        .iter()
        .zip(metadata.iter())
        .all(|(value, (_, target_value))| *value == *target_value)
    {
        align_steps += step_size;
        for i in 0..values.len() {
            let (cycle_length, target_value) = metadata[i];
            let previous_value = values[i];
            values[i] += step_size;
            values[i] %= cycle_length;
            if previous_value != target_value && values[i] == target_value {
                step_size = num::integer::lcm(step_size, cycle_length);
            }
        }
    }

    println!("{}", min_steps + lcm - align_steps);
}
