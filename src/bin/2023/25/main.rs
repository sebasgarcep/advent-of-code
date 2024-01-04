extern crate lib;

use std::collections::HashMap;

use bit_set::BitSet;
use itertools::Itertools;
use lib::reader::read_lines;
use rand::Rng;

pub fn main() {
    first();
    second();
}

fn first() {
    FirstSolver::solve();
}

enum FirstSolver {}

impl Solver for FirstSolver {}

fn second() {}

enum SecondSolver {}

impl Solver for SecondSolver {}

struct CompressedGraph {
    nodes: Vec<CompressedGraphNode>,
    index_map: HashMap<[u8; 3], usize>,
}

struct CompressedGraphNode {
    label: [u8; 3],
    edges: BitSet,
}

impl CompressedGraph {
    pub fn from_lines<I: Iterator<Item = String>>(line_collection: I) -> Self {
        let mut nodes = Vec::new();
        let mut index_map = HashMap::new();
        for mut line in line_collection {
            let split_position = line.find(": ").unwrap();
            let right = line.split_off(split_position + 2);
            let _ = line.split_off(split_position);
            let left = line;
            let left_pos = Self::get_position(&mut nodes, &mut index_map, &left);
            for s in right.split(' ') {
                let right_pos = Self::get_position(&mut nodes, &mut index_map, s);
                nodes[left_pos].edges.insert(right_pos);
                nodes[right_pos].edges.insert(left_pos);
            }
        }
        return Self { nodes, index_map };
    }

    fn get_position(
        nodes: &mut Vec<CompressedGraphNode>,
        index_map: &mut HashMap<[u8; 3], usize>,
        label: &str,
    ) -> usize {
        let short_label: [u8; 3] = label.as_bytes().try_into().unwrap();
        if !index_map.contains_key(&short_label) {
            index_map.insert(short_label, nodes.len());
            nodes.push(CompressedGraphNode {
                label: short_label,
                edges: BitSet::new(),
            });
        }
        return index_map[&short_label];
    }
}

struct KargersAlgorithm {
    // Indexed by node idx
    node_groups: Vec<BitSet>,
    edges_map: Vec<BitSet>,
    // Indexed by edge idx
    edges: Vec<(usize, usize)>,
}

impl KargersAlgorithm {
    pub fn new(graph: &CompressedGraph) -> Self {
        let mut node_groups: Vec<BitSet> =
            vec![BitSet::with_capacity(graph.nodes.len()); graph.nodes.len()];
        for i in 0..graph.nodes.len() {
            node_groups[i].insert(i);
        }

        let mut edges_map: Vec<BitSet> = vec![BitSet::new(); graph.nodes.len()];
        let mut edges = Vec::new();
        for j in 1..graph.nodes.len() {
            for i in 0..j {
                if graph.nodes[i].edges.contains(j) {
                    edges_map[i].insert(edges.len());
                    edges_map[j].insert(edges.len());
                    edges.push((i, j));
                }
            }
        }

        return Self {
            node_groups,
            edges_map,
            edges,
        };
    }

    pub fn execute(&mut self) {
        let mut rng = rand::thread_rng();
        let mut node_group_count = self.node_groups.len();
        while node_group_count > 2 {
            // Select edge at random
            let selected_edge_idx = rng.gen_range(0..self.edges.len());
            let selected_edge = self.edges[selected_edge_idx].clone();

            // Remove edges between first and second
            let mut removable_edges: Vec<usize> = self.edges_map[selected_edge.0]
                .intersection(&self.edges_map[selected_edge.1])
                .sorted()
                .collect();
            while let Option::Some(idx) = removable_edges.pop() {
                self.remove_edge(idx);
            }

            // Merge nodes
            let merged_node_group = self.node_groups[selected_edge.1].clone();
            self.node_groups[selected_edge.0].union_with(&merged_node_group);
            self.node_groups[selected_edge.1].clear();

            // All existing nodes pointing to second now will point to first
            for pos in self.edges_map[selected_edge.1].iter().collect_vec() {
                let ref mut edge = self.edges[pos];
                if edge.0 == selected_edge.1 {
                    edge.0 = selected_edge.0;
                } else if edge.1 == selected_edge.1 {
                    edge.1 = selected_edge.0;
                }
                self.edges_map[selected_edge.0].insert(pos);
                self.edges_map[selected_edge.1].remove(pos);
            }

            // One less node
            node_group_count -= 1;
        }
    }

    fn remove_edge(&mut self, idx: usize) {
        let a = idx;
        let b = self.edges.len() - 1;

        let (source, target) = self.edges[a];
        self.edges_map[source].remove(a);
        self.edges_map[target].remove(a);

        if a != b {
            let (source, target) = self.edges[b];
            self.edges_map[source].remove(b);
            self.edges_map[target].remove(b);
            self.edges_map[source].insert(a);
            self.edges_map[target].insert(a);

            self.edges.swap(a, b);
        }

        self.edges.pop();
    }

    fn is_done(&self) -> bool {
        return self.edges.len() == 3;
    }

    fn get_result(&self) -> usize {
        return self.node_groups.iter().map(|s| s.len()).filter(|s| *s > 0).product();
    }
}

trait Solver {
    fn solve() {
        let line_collection = read_lines("data/2023/25/input.txt");
        let graph = CompressedGraph::from_lines(line_collection);

        let mut kargers_algorithm;
        loop {
            // Karger's algorithm
            kargers_algorithm = KargersAlgorithm::new(&graph);
            kargers_algorithm.execute();
            if kargers_algorithm.is_done() {
                break;
            }
        }

        let result = kargers_algorithm.get_result();
        println!("{:?}", result);
    }
}
