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
}

impl FirstSolver {
    pub fn new() -> Self {
        return Self {
            num_low: 0,
            num_high: 0,
        };
    }
}

impl Solver for FirstSolver {
    fn get_result(&mut self, mut machine: Machine) -> usize {
        let button_index = machine.index_map["button"];
        let broadcaster_index = machine.index_map["broadcaster"];
        for _ in 0..1000 {
            self.run_machine(&mut machine, button_index, broadcaster_index);
        }
        return self.num_low * self.num_high;
    }

    fn on_queued_pulse(
        &mut self,
        _machine: &Machine,
        source: usize,
        destination: usize,
        strength: bool,
    ) {
        if strength {
            self.num_high += 1;
        } else {
            self.num_low += 1;
        }
    }
}

struct SecondSolver {}

impl SecondSolver {
    pub fn new() -> Self {
        return Self {};
    }
}

impl Solver for SecondSolver {
    fn get_result(&mut self, mut machine: Machine) -> usize {
        let button_index = machine.index_map["button"];
        let broadcaster_index = machine.index_map["broadcaster"];
        let output_module_position = machine.index_map["rx"];
        let bottleneck_module_position = machine
            .modules
            .iter()
            .find_position(|module| module.destinations.contains(&output_module_position))
            .unwrap()
            .0;
        loop {
            self.run_machine(&mut machine, button_index, broadcaster_index);
        }
        return 0;
    }

    fn on_queued_pulse(
        &mut self,
        machine: &Machine,
        source: usize,
        destination: usize,
        strength: bool,
    ) {
        todo!()
    }
}

fn second() {
    let mut solver = SecondSolver::new();
    solver.solve();
}

#[derive(Debug)]
struct Machine {
    modules: Vec<Module>,
    index_map: HashMap<String, usize>,
    pulse_queue: VecDeque<(usize, usize, bool)>,
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
    FlipFlop(bool),
    Conjunction(Vec<bool>),
}

trait Solver {
    fn get_result(&mut self, machine: Machine) -> usize;
    fn on_queued_pulse(
        &mut self,
        machine: &Machine,
        source: usize,
        destination: usize,
        strength: bool,
    );

    fn solve(&mut self) {
        let mut machine = Self::get_machine();
        Self::init_memory(&mut machine);
        let result = self.get_result(machine);
        println!("{}", result);
    }

    fn get_machine() -> Machine {
        let line_collection = read_lines("data/2023/20/input.txt");
        let mut machine = Machine {
            modules: vec![],
            index_map: HashMap::new(),
            pulse_queue: VecDeque::new(),
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
            return (name, ModuleClass::FlipFlop(false), destinations);
        }

        if line == "&" {
            return (name, ModuleClass::Conjunction(vec![]), destinations);
        }

        unreachable!()
    }

    fn init_memory(machine: &mut Machine) {
        for module in machine.modules.iter_mut() {
            match module.class {
                ModuleClass::FlipFlop(ref mut state) => {
                    *state = false;
                }
                ModuleClass::Conjunction(ref mut memory) => {
                    *memory = vec![false; module.sources.len()];
                }
                _ => {}
            }
        }
    }

    fn get_pulse_strength(
        machine: &mut Machine,
        source: usize,
        destination: usize,
        input: bool,
    ) -> Option<bool> {
        let ref mut module = machine.modules[destination];
        return match module.class {
            ModuleClass::Broadcast => Option::Some(input),
            ModuleClass::FlipFlop(ref mut state) => {
                if input {
                    return Option::None;
                }
                *state = !*state;
                return Option::Some(*state);
            }
            ModuleClass::Conjunction(ref mut memory) => {
                let source_memory_index = module
                    .sources
                    .iter()
                    .find_position(|iter_source| source == **iter_source)
                    .unwrap()
                    .0;
                memory[source_memory_index] = input;
                return Option::Some(!memory.iter().all(|x| *x));
            }
            _ => Option::None,
        };
    }

    fn run_machine(
        &mut self,
        machine: &mut Machine,
        button_index: usize,
        broadcaster_index: usize,
    ) {
        self.queue_pulse(machine, button_index, broadcaster_index, false);
        // Process pulses
        while let Some(pulse) = machine.pulse_queue.pop_front() {
            let (source, destination, input) = pulse;
            if let Option::Some(output) =
                Self::get_pulse_strength(machine, source, destination, input)
            {
                for i in 0..machine.modules[destination].destinations.len() {
                    let item = machine.modules[destination].destinations[i];
                    self.queue_pulse(machine, destination, item, output);
                }
            }
        }
    }

    fn queue_pulse(
        &mut self,
        machine: &mut Machine,
        source: usize,
        destination: usize,
        strength: bool,
    ) {
        machine
            .pulse_queue
            .push_back((source, destination, strength));
        self.on_queued_pulse(machine, source, destination, strength);
    }
}
