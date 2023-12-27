extern crate lib;

use lib::reader::read_split;

pub fn main() {
    first();
    second();
}

fn first() {
    solve();
}

fn second() {}

fn solve() {
    let line_collection: Vec<String> = read_split("data/2023/15/input.txt", ',').collect();

    let mut result: i64 = 0;
    for line in line_collection {
        let mut hash: i64 = 0;
        for char in line.chars() {
            hash += char as i64;
            hash *= 17;
            hash = hash % 256;
        }
        result += hash;
    }
    println!("{}", result);
}
