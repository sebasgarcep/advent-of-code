extern crate lib;

use lib::reader::read_lines;

pub fn main() {
    first();
    second();
}

fn first() {
    let line_iterator = read_lines("data/2023/09/input.txt");

    let mut result: i64 = 0;
    for line in line_iterator {
        let mut values: Vec<i64> = line.split(" ").map(|x| x.parse().unwrap()).collect();
        let mut differences: Vec<i64> = vec![0; values.len()];
        let mut extrapolated: i64 = 0;
        for size in (2..=values.len()).rev() {
            extrapolated += values[size - 1];
            let mut is_constant = true;
            for idx in 0..(size - 1) {
                let current = values[idx + 1] - values[idx];
                if current != 0 {
                    is_constant = false;
                }
                differences[idx] = current;
            }
            if is_constant {
                break;
            }
            (values, differences) = (differences, values);
        }
        result += extrapolated;
    }
    println!("{}", result);
}

fn second() {
    let line_iterator = read_lines("data/2023/09/input.txt");

    let mut result: i64 = 0;
    for line in line_iterator {
        let mut values: Vec<i64> = line.split(" ").map(|x| x.parse().unwrap()).collect();
        let mut differences: Vec<i64> = vec![0; values.len()];
        let mut heads: Vec<i64> = Vec::with_capacity(values.len());
        for size in (2..=values.len()).rev() {
            heads.push(values[0]);
            let mut is_constant = true;
            for idx in 0..(size - 1) {
                let current = values[idx + 1] - values[idx];
                if current != 0 {
                    is_constant = false;
                }
                differences[idx] = current;
            }
            if is_constant {
                break;
            }
            (values, differences) = (differences, values);
        }
        let mut extrapolated: i64 = 0;
        for val in heads.into_iter().rev() {
            extrapolated = val - extrapolated;
        }
        result += extrapolated;
    }
    println!("{}", result);
}
