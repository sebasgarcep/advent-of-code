extern crate lib;

use std::collections::HashMap;

use itertools::Itertools;
use lib::reader::read_lines;

pub fn main() {
    first();
    second();
}

fn first() {
    solve::<FirstSolver>();
}

fn second() {
    solve::<SecondSolver>();
}

enum FirstSolver {}

impl FirstSolver {
    fn parse_data(line: String) -> HashMap<String, i64> {
        return line[1..(line.len() - 1)]
            .split(',')
            .map(String::from)
            .map(Self::parse_data_line)
            .collect();
    }

    fn parse_data_line(mut line: String) -> (String, i64) {
        let split_position = line.find('=').unwrap();
        let value: i64 = line.split_off(split_position + 1).parse().unwrap();
        line.truncate(line.len() - 1);
        return (line, value);
    }

    fn execute_workflow_map(
        data: &HashMap<String, i64>,
        workflow_map: &HashMap<String, Workflow>,
    ) -> bool {
        let mut label = "in".to_owned();

        loop {
            let workflow = &workflow_map[&label];
            match Self::execute_workflow(&data, &workflow) {
                Decision::Workflow(next_label) => {
                    label = next_label;
                }
                Decision::Accepted => {
                    return true;
                }
                Decision::Rejected => {
                    return false;
                }
            };
        }
    }

    fn execute_workflow(data: &HashMap<String, i64>, workflow: &Workflow) -> Decision {
        for check in workflow.checks.iter() {
            if Self::calculate_check(&data, &check) {
                return check.decision.clone();
            }
        }

        return workflow.fallback.clone();
    }

    fn calculate_check(data: &HashMap<String, i64>, check: &Check) -> bool {
        let var_value = data[&check.var_name];
        return match check.operation {
            Operation::GreaterThan => var_value > check.value,
            Operation::LessThan => var_value < check.value,
        };
    }
}

impl Solver for FirstSolver {
    fn get_result<I: Iterator<Item = String>>(line_collection: &mut I) -> i64 {
        let workflow_map = parse_workflow_map(line_collection);

        let mut result: i64 = 0;
        while let Some(line) = line_collection.next() {
            let data = Self::parse_data(line);
            let accepted = Self::execute_workflow_map(&data, &workflow_map);
            if accepted {
                result += data.values().sum::<i64>();
            }
        }

        return result;
    }
}

enum SecondSolver {}

impl SecondSolver {
    fn get_result_from_workflow(
        workflow_map: &HashMap<String, Workflow>,
        mut range_set: HashMap<String, (i64, i64)>,
        key: &str,
    ) -> i64 {
        let workflow = &workflow_map[key];
        let mut total = 0;
        for check in workflow.checks.iter() {
            let current_range = range_set[&check.var_name];
            match check.operation {
                Operation::GreaterThan => {
                    if current_range.1 <= check.value {
                        continue;
                    }
                }
                Operation::LessThan => {
                    if current_range.0 >= check.value {
                        continue;
                    }
                }
            };
            let mut next_range_set = range_set.clone();
            match check.operation {
                Operation::GreaterThan => {
                    next_range_set.get_mut(&check.var_name).unwrap().0 = check.value + 1;
                    range_set.get_mut(&check.var_name).unwrap().1 = check.value;
                }
                Operation::LessThan => {
                    next_range_set.get_mut(&check.var_name).unwrap().1 = check.value - 1;
                    range_set.get_mut(&check.var_name).unwrap().0 = check.value;
                }
            };
            total += Self::get_result_from_decision(workflow_map, next_range_set, &check.decision);
        }
        total += Self::get_result_from_decision(workflow_map, range_set, &workflow.fallback);
        return total;
    }

    fn get_result_from_decision(
        workflow_map: &HashMap<String, Workflow>,
        range_set: HashMap<String, (i64, i64)>,
        decision: &Decision,
    ) -> i64 {
        return match decision {
            Decision::Accepted => range_set
                .values()
                .map(|(start, end)| std::cmp::max(0, end - start + 1))
                .product(),
            Decision::Rejected => 0,
            Decision::Workflow(label) => {
                Self::get_result_from_workflow(workflow_map, range_set, label)
            }
        };
    }
}

impl Solver for SecondSolver {
    fn get_result<I: Iterator<Item = String>>(line_collection: &mut I) -> i64 {
        let workflow_map = parse_workflow_map(line_collection);
        let ranges: HashMap<String, (i64, i64)> = HashMap::from([
            ("x".to_owned(), (1, 4000)),
            ("m".to_owned(), (1, 4000)),
            ("a".to_owned(), (1, 4000)),
            ("s".to_owned(), (1, 4000)),
        ]);
        return Self::get_result_from_workflow(&workflow_map, ranges, "in");
    }
}

trait Solver {
    fn get_result<I: Iterator<Item = String>>(line_collection: &mut I) -> i64;
}

struct Workflow {
    checks: Vec<Check>,
    fallback: Decision,
}

enum Operation {
    LessThan,
    GreaterThan,
}

struct Check {
    var_name: String,
    operation: Operation,
    value: i64,
    decision: Decision,
}

#[derive(Clone)]
enum Decision {
    Workflow(String),
    Accepted,
    Rejected,
}

fn solve<S: Solver>() {
    let mut line_collection = read_lines("data/2023/19/input.txt");
    let result = S::get_result(&mut line_collection);
    println!("{}", result);
}

fn parse_workflow_map<I: Iterator<Item = String>>(
    line_collection: &mut I,
) -> HashMap<String, Workflow> {
    let mut workflow_map: HashMap<String, Workflow> = HashMap::new();
    while let Some(line) = line_collection.next() {
        if line.is_empty() {
            break;
        }

        let first_split = line.find('{').unwrap();
        let workflow_name = line[..first_split].to_owned();
        let mut contents = line[(first_split + 1)..(line.len() - 1)]
            .split(',')
            .map(String::from)
            .collect_vec();
        let fallback_label = contents.pop().unwrap();
        let fallback = parse_decision(fallback_label);
        let checks = contents.into_iter().map(|l| parse_check(l)).collect_vec();

        let workflow = Workflow { checks, fallback };

        workflow_map.insert(workflow_name, workflow);
    }
    return workflow_map;
}

fn parse_decision(label: String) -> Decision {
    return match label.as_bytes()[0] as char {
        'A' => Decision::Accepted,
        'R' => Decision::Rejected,
        _ => Decision::Workflow(label),
    };
}

fn parse_check(line: String) -> Check {
    let operation_position = line.find(|c| c == '>' || c == '<').unwrap();
    let split_position = line.find(':').unwrap();
    let var_name = line[..operation_position].to_owned();
    let operation = match line.as_bytes()[operation_position] as char {
        '>' => Operation::GreaterThan,
        '<' => Operation::LessThan,
        _ => unreachable!(),
    };
    let value: i64 = line[(operation_position + 1)..split_position]
        .parse()
        .unwrap();
    return Check {
        var_name,
        operation,
        value,
        decision: parse_decision(line[(split_position + 1)..].to_owned()),
    };
}
