extern crate lib;

use std::{
    collections::{HashMap, VecDeque},
    usize,
};

use itertools::Itertools;
use lib::reader::read_lines;

pub fn main() {
    first();
    second();
}

fn first() {
    let mut solver = FirstSolver::new();
    solver.solve();
}

struct FirstSolver {
    num_low: usize,
    num_high: usize,
    memory: Vec<Vec<bool>>,
    pulse_queue: VecDeque<(usize, usize, bool)>,
}

impl FirstSolver {
    pub fn new() -> Self {
        return Self {
            num_low: 0,
            num_high: 0,
            memory: Vec::new(),
            pulse_queue: VecDeque::new(),
        };
    }

    fn init_memory(&mut self, machine: &Machine) {
        self.memory = machine
            .modules
            .iter()
            .map(|module| match module.class {
                ModuleClass::FlipFlop => vec![false],
                ModuleClass::Conjunction => vec![false; module.sources.len()],
                _ => Vec::new(),
            })
            .collect();
    }

    fn queue_pulse(&mut self, source: usize, destination: usize, strength: bool) {
        if strength {
            self.num_high += 1;
        } else {
            self.num_low += 1;
        }
        self.pulse_queue.push_back((source, destination, strength));
    }

    fn get_pulse_strength(
        &mut self,
        machine: &Machine,
        source: usize,
        destination: usize,
        input: bool,
    ) -> Option<bool> {
        let ref module = machine.modules[destination];
        let ref mut destination_memory = self.memory[destination];
        return match module.class {
            ModuleClass::Broadcast => Option::Some(input),
            ModuleClass::FlipFlop => {
                if input {
                    return Option::None;
                }
                destination_memory[0] = !destination_memory[0];
                return Option::Some(destination_memory[0]);
            }
            ModuleClass::Conjunction => {
                let source_memory_index = module
                    .sources
                    .iter()
                    .find_position(|iter_source| source == **iter_source)
                    .unwrap().0;
                destination_memory[source_memory_index] = input;
                return Option::Some(!destination_memory.iter().all(|x| *x));
            }
            _ => Option::None,
        };
    }
}

impl Solver for FirstSolver {
    fn get_result(&mut self, machine: Machine) -> usize {
        self.init_memory(&machine);
        let button_index = machine.index_map["button"];
        let broadcaster_index = machine.index_map["broadcaster"];
        for _ in 0..1000 {
            // Press button
            self.queue_pulse(button_index, broadcaster_index, false);
            // Process pulses
            while let Some(pulse) = self.pulse_queue.pop_front() {
                let (source, destination, input) = pulse;
                if let Option::Some(output) =
                    self.get_pulse_strength(&machine, source, destination, input)
                {
                    let ref module = machine.modules[destination];
                    for item in module.destinations.iter() {
                        self.queue_pulse(destination, *item, output);
                    }
                }
            }
        }
        return self.num_low * self.num_high;
    }
}

fn second() {}

#[derive(Debug)]
struct Machine {
    modules: Vec<Module>,
    index_map: HashMap<String, usize>,
}

#[derive(Debug)]
struct Module {
    sources: Vec<usize>,
    destinations: Vec<usize>,
    class: ModuleClass,
}

#[derive(Debug)]
enum ModuleClass {
    Noop,
    Button,
    Broadcast,
    FlipFlop,
    Conjunction,
}

trait Solver {
    fn get_result(&mut self, machine: Machine) -> usize;

    fn solve(&mut self) {
        let machine = Self::get_machine();
        let result = self.get_result(machine);
        println!("{}", result);
    }

    fn get_machine() -> Machine {
        let line_collection = read_lines("data/2023/20/input.txt");
        let mut machine = Machine {
            modules: vec![],
            index_map: HashMap::new(),
        };
        Self::add_machine_spec(
            &mut machine,
            "button".to_owned(),
            ModuleClass::Button,
            vec!["broadcaster".to_owned()],
        );
        for line in line_collection {
            let (source, class, destinations) = Self::parse_line(line);
            Self::add_machine_spec(&mut machine, source, class, destinations);
        }
        for module in machine.modules.iter_mut() {
            module.sources.sort();
            module.destinations.sort();
        }
        return machine;
    }

    fn add_machine_spec(
        machine: &mut Machine,
        source: String,
        class: ModuleClass,
        destinations: Vec<String>,
    ) {
        let source_index = Self::get_label_index(machine, source);
        let destination_indexes = destinations
            .into_iter()
            .map(|label| Self::get_label_index(machine, label))
            .collect_vec();
        machine.modules[source_index].class = class;
        for destination_index in destination_indexes.iter() {
            machine.modules[*destination_index]
                .sources
                .push(source_index);
        }
        machine.modules[source_index].destinations = destination_indexes;
    }

    fn get_label_index(machine: &mut Machine, label: String) -> usize {
        if machine.index_map.contains_key(&label) {
            return machine.index_map[&label];
        }

        let index = machine.modules.len();
        machine.modules.push(Module {
            class: ModuleClass::Noop,
            sources: vec![],
            destinations: vec![],
        });
        machine.index_map.insert(label, index);
        return index;
    }

    fn parse_line(mut line: String) -> (String, ModuleClass, Vec<String>) {
        let split_position = line.find(" -> ").unwrap();
        let destinations = line
            .split_off(split_position + 4)
            .split(',')
            .map(|s| s.trim())
            .map(String::from)
            .collect_vec();

        let _ = line.split_off(split_position);
        if line == "broadcaster" {
            return (line, ModuleClass::Broadcast, destinations);
        }

        let name = line.split_off(1);
        if line == "%" {
            return (name, ModuleClass::FlipFlop, destinations);
        }

        if line == "&" {
            return (name, ModuleClass::Conjunction, destinations);
        }

        unreachable!()
    }
}
