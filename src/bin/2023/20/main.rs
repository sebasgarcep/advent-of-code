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
    solve(&mut solver);
}

struct FirstSolver {
    iters: usize,
    num_low: usize,
    num_high: usize,
}

impl FirstSolver {
    pub fn new() -> Self {
        return Self {
            iters: 0,
            num_low: 0,
            num_high: 0,
        };
    }
}

impl Solver for FirstSolver {
    fn process_pulse(&mut self, _source: String, _destination: String, strength: bool) {
        if strength {
            self.num_high += 1;
        } else {
            self.num_low += 1;
        }
    }

    fn get_result(&self) -> usize {
        return self.num_low * self.num_high;
    }

    fn should_continue(&self) -> bool {
        return self.iters < 1000;
    }

    fn pressed_button(&mut self) {
        self.iters += 1;
    }
}

fn second() {
    let mut solver = SecondSolver::new();
    solve(&mut solver);
}

struct SecondSolver {
    iters: usize,
    done: bool,
}

impl SecondSolver {
    pub fn new() -> Self {
        return Self {
            iters: 0,
            done: false,
        };
    }
}

impl Solver for SecondSolver {
    fn process_pulse(&mut self, _source: String, destination: String, strength: bool) {
        if !strength && destination == "rx".to_owned() {
            self.done = true;
        }
    }

    fn get_result(&self) -> usize {
        return self.iters;
    }

    fn should_continue(&self) -> bool {
        return !self.done;
    }

    fn pressed_button(&mut self) {
        self.iters += 1;
    }
}

trait Solver {
    fn process_pulse(&mut self, source: String, destination: String, strength: bool);
    fn get_result(&self) -> usize;
    fn should_continue(&self) -> bool;
    fn pressed_button(&mut self);
}

struct Module {
    destinations: Vec<String>,
    class: ModuleClass,
}

enum ModuleClass {
    Broadcast,
    FlipFlop(bool),
    Conjunction(HashMap<String, bool>),
}

struct Machine {
    pulse_queue: VecDeque<(String, String, bool)>,
    module_map: HashMap<String, Module>,
}

impl Machine {
    pub fn from_lines<I: Iterator<Item = String>>(line_collection: I) -> Self {
        let mut module_map: HashMap<String, Module> = HashMap::new();
        for line in line_collection {
            let (name, module) = parse_line(line);
            module_map.insert(name, module);
        }
        let conjunction_inserts = module_map
            .iter()
            .flat_map(|(name, module)| {
                module
                    .destinations
                    .iter()
                    .map(|destination| (name.clone(), destination.clone()))
            })
            .collect_vec();
        for (name, destination) in conjunction_inserts {
            let maybe_module = module_map.get_mut(&destination);
            if maybe_module.is_none() {
                continue;
            }
            let module = maybe_module.unwrap();
            if let ModuleClass::Conjunction(ref mut memory) = module.class {
                memory.insert(name, false);
            }
        }

        return Self {
            pulse_queue: VecDeque::new(),
            module_map,
        };
    }

    pub fn press_button<S: Solver>(&mut self, solver: &mut S) {
        solver.pressed_button();
        self.queue_pulse(solver, "button".to_owned(), "broadcaster".to_owned(), false);
        while let Some(pulse) = self.pulse_queue.pop_front() {
            let (source, destination, input) = pulse;
            let output = {
                let maybe_module: Option<&mut Module> = self.module_map.get_mut(&destination);
                if maybe_module.is_none() {
                    continue;
                }
                let module = maybe_module.unwrap();
                let output = module.get_pulse_strength(&source, input);
                if output.is_none() {
                    continue;
                }
                output.unwrap()
            };
            let module = &self.module_map[&destination];
            for item in module.destinations.iter().map(String::from).collect_vec() {
                self.queue_pulse(solver, destination.clone(), item, output);
            }
        }
    }

    fn queue_pulse<S: Solver>(
        &mut self,
        solver: &mut S,
        source: String,
        destination: String,
        strength: bool,
    ) {
        solver.process_pulse(source.clone(), destination.clone(), strength);
        self.pulse_queue.push_back((source, destination, strength));
    }
}

impl Module {
    fn get_pulse_strength(&mut self, source: &str, input: bool) -> Option<bool> {
        return match self.class {
            ModuleClass::Broadcast => Option::Some(input),
            ModuleClass::FlipFlop(ref mut state) => {
                if input {
                    return Option::None;
                }
                *state = !*state;
                return Option::Some(*state);
            }
            /* Need to do some refactoring to make this work!! */
            ModuleClass::Conjunction(ref mut memory) => {
                *memory.get_mut(source).unwrap() = input;
                return Option::Some(!memory.values().all(|x| *x));
            }
        };
    }
}

fn solve<S: Solver>(solver: &mut S) {
    let line_collection = read_lines("data/2023/20/input.txt");
    let mut machine = Machine::from_lines(line_collection);

    while solver.should_continue() {
        machine.press_button(solver);
    }
    let result = solver.get_result();
    println!("{}", result);
}

fn parse_line(mut line: String) -> (String, Module) {
    let split_position = line.find(" -> ").unwrap();
    let destinations = line
        .split_off(split_position + 4)
        .split(',')
        .map(|s| s.trim())
        .map(String::from)
        .collect_vec();

    let _ = line.split_off(split_position);
    if line == "broadcaster" {
        return (
            line,
            Module {
                destinations,
                class: ModuleClass::Broadcast,
            },
        );
    }

    let name = line.split_off(1);
    if line == "%" {
        return (
            name,
            Module {
                destinations,
                class: ModuleClass::FlipFlop(false),
            },
        );
    } else if line == "&" {
        return (
            name,
            Module {
                destinations,
                class: ModuleClass::Conjunction(HashMap::new()),
            },
        );
    }

    unreachable!();
}
