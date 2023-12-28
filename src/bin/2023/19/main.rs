extern crate lib;

use std::collections::HashMap;

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

fn solve() {
    let mut line_collection = read_lines("data/2023/19/input.txt");

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

    let mut result: i64 = 0;
    while let Some(line) = line_collection.next() {
        let data = parse_data(line);
        let accepted = execute_workflow_map(&data, &workflow_map);
        if accepted {
            result += data.values().sum::<i64>();
        }
    }
    println!("{}", result);
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

fn parse_data(line: String) -> HashMap<String, i64> {
    return line[1..(line.len() - 1)]
        .split(',')
        .map(String::from)
        .map(parse_data_line)
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
        match execute_workflow(&data, &workflow) {
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
        if calculate_check(&data, &check) {
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
    }
}
