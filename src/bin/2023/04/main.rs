extern crate lib;

use lib::reader::read_lines;

const SKIPPABLE: usize = 10;
const WINNING_NUMBERS: usize = 10;
const OWNED_NUMBERS: usize = 25;

fn main() {
    let mut line_iterator = read_lines("data/2023/04/input.txt");
    let mut total: i64 = 0;
    for line in line_iterator {
        let mut pos: usize = SKIPPABLE;
        let mut winning_flag: u128 = 0;
        let mut value: i64 = 0;
        for _ in 0..WINNING_NUMBERS {
            let item: i64 = line[pos..(pos + 2)].trim().parse().unwrap();
            winning_flag |= 1 << item;
            pos += 3;
        }
        pos += 2;
        for _ in 0..OWNED_NUMBERS {
            let item: i64 = line[pos..(pos + 2)].trim().parse().unwrap();
            if winning_flag & (1 << item) != 0 {
                if value == 0 {
                    value += 1
                } else {
                    value *= 2;
                }
            }
            pos += 3;
        }
        total += value;
    }
    println!("{}", total);
}
