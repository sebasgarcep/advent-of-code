extern crate lib;

use lib::linked_lists::SinglyLinkedList;
use lib::reader::read_lines;

const SKIPPABLE: usize = 10;
const WINNING_NUMBERS: usize = 10;
const OWNED_NUMBERS: usize = 25;

fn main() {
    first();
    second();
}

trait Consumer {
    fn consume_matches(&mut self, number: i64);
}

struct FirstConsumer {
    pub total: i64,
}

impl FirstConsumer {
    pub fn new() -> FirstConsumer {
        return FirstConsumer { total: 0 };
    }
}

impl Consumer for FirstConsumer {
    fn consume_matches(&mut self, number: i64) {
        if number == 0 {
            return;
        }
        self.total += 1 << (number - 1);
    }
}

struct SecondConsumer {
    pub total: i64,
    list: SinglyLinkedList<i64>,
}

impl SecondConsumer {
    pub fn new() -> SecondConsumer {
        return SecondConsumer {
            total: 0,
            list: SinglyLinkedList::new(),
        };
    }
}

impl Consumer for SecondConsumer {
    fn consume_matches(&mut self, number: i64) {
        let current = self.list.pop().unwrap_or(1);
        self.total += current;
        if number == 0 {
            return;
        }
        if self.list.is_empty() {
            self.list.insert_tail(1);
        }
        let mut node = self.list.get_head().unwrap();
        for _ in 0..number {
            node.set_data(node.get_data() + current);
            if node.next().is_none() {
                self.list.insert_tail(1);
            }
            node = node.next().unwrap();
        }
    }
}

fn first() {
    let mut consumer = FirstConsumer::new();
    iterate(&mut consumer);
    println!("{}", consumer.total);
}

fn second() {
    let mut consumer = SecondConsumer::new();
    iterate(&mut consumer);
    println!("{}", consumer.total);
}

fn iterate<C: Consumer>(consumer: &mut C) {
    let line_iterator = read_lines("data/2023/04/input.txt");
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
                value += 1;
            }
            pos += 3;
        }
        consumer.consume_matches(value);
    }
}
