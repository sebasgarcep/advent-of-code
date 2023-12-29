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
    solve();
}

fn second() {}

struct Module {
    destinations: Vec<String>,
    class: ModuleClass,
}

enum ModuleClass {
    Broadcast,
    FlipFlop(bool),
    Conjunction(HashMap<String, bool>),
}

impl Module {
    pub fn process_pulse(
        &mut self,
        pulse_queue: &mut VecDeque<(String, String, bool)>,
        num_low: &mut usize,
        num_high: &mut usize,
        source: String,
        destination: String,
        input: bool,
    ) {
        let output = self.get_pulse_strength(&source, input);
        if output.is_none() {
            return;
        }
        let output = output.unwrap();
        match output {
            false => {
                *num_low += self.destinations.len();
            }
            true => {
                *num_high += self.destinations.len();
            }
        }
        for item in self.destinations.iter() {
            pulse_queue.push_back((destination.clone(), item.clone(), output));
        }
    }

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

fn solve() {
    let line_collection = read_lines("data/2023/20/input.txt");

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

    let mut num_low: usize = 0;
    let mut num_high: usize = 0;
    let mut pulse_queue: VecDeque<(String, String, bool)> = VecDeque::new();
    for _ in 0..1000 {
        pulse_queue.push_back(("button".to_owned(), "broadcaster".to_owned(), false));
        num_low += 1;
        while let Some(pulse) = pulse_queue.pop_front() {
            let (source, destination, input) = pulse;
            let maybe_module: Option<&mut Module> = module_map.get_mut(&destination);
            if maybe_module.is_none() {
                continue;
            }
            let module = maybe_module.unwrap();
            module.process_pulse(
                &mut pulse_queue,
                &mut num_low,
                &mut num_high,
                source,
                destination,
                input,
            );
        }
    }
    let result = num_low * num_high;
    println!("{} {} {}", num_low, num_high, result);
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
