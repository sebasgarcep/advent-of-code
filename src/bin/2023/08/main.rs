extern crate lib;

use lib::reader::read_lines;
use std::collections::HashMap;

pub fn main() {
    first();
    second();
}

fn first() {
    let mut line_iterator = read_lines("data/2023/08/input.txt");
    let instructions_line = line_iterator.next().unwrap();
    let _ = line_iterator.next();

    let mut naive_map: HashMap<[u8; 3], ([u8; 3], [u8; 3])> = HashMap::new();
    for line in line_iterator {
        let line_bytes = line.as_bytes();
        let source: [u8; 3] = line_bytes[0..3].try_into().unwrap();
        let left: [u8; 3] = line_bytes[7..10].try_into().unwrap();
        let right: [u8; 3] = line_bytes[12..15].try_into().unwrap();
        naive_map.insert(source, (left, right));
    }

    let stop: [u8; 3] = "ZZZ".as_bytes().try_into().unwrap();
    let mut smart_map: HashMap<[u8; 3], ([u8; 3], i64)> = HashMap::new();
    for start in naive_map.keys() {
        let mut num_steps: i64 = 0;
        let mut current = start.clone();
        for instruction in instructions_line.chars() {
            if current == stop {
                break;
            }
            num_steps += 1;
            let tuple = naive_map[&current];
            current = match instruction {
                'L' => tuple.0,
                'R' => tuple.1,
                _ => unreachable!(),
            };
        }
        smart_map.insert(start.clone(), (current, num_steps));
    }

    let mut result: i64 = 0;
    let mut current: [u8; 3] = "AAA".as_bytes().try_into().unwrap();
    while current != stop {
        let (next, num_steps) = smart_map[&current];
        result += num_steps;
        current = next;
    }

    println!("{}", result);
}

fn second() {}
