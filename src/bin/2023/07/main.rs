extern crate lib;

use lib::reader::read_lines;

pub fn main() {
    first();
    second();
}

trait Sorter {
    fn get_value_of_hand(&self, hand: &Vec<char>) -> i64;
    fn get_value_of_card(&self, card: char) -> i64;
}

struct FirstSorter {}

impl FirstSorter {
    pub fn new() -> Self {
        return FirstSorter {};
    }
}

impl Sorter for FirstSorter {
    fn get_value_of_hand(&self, hand: &Vec<char>) -> i64 {
        let mut hand = hand.clone();
        hand.sort();

        let mut counts: Vec<i64> = Vec::with_capacity(5);
        for idx in 0..hand.len() {
            if idx == 0 || hand[idx - 1] != hand[idx] {
                counts.push(1);
            } else {
                let size = counts.len();
                counts[size - 1] += 1;
            }
        }
        counts.sort();

        if counts[0] == 5 {
            return 7;
        }
        if counts[0] == 1 && counts[1] == 4 {
            return 6;
        }
        if counts[0] == 2 && counts[1] == 3 {
            return 5;
        }
        if counts[counts.len() - 1] == 3 {
            return 4;
        }
        if counts[0] == 1 && counts[1] == 2 && counts[2] == 2 {
            return 3;
        }
        if counts[counts.len() - 1] == 2 {
            return 2;
        }
        return 1;
    }

    fn get_value_of_card(&self, card: char) -> i64 {
        if card.is_digit(10) {
            return card as i64 - '0' as i64;
        }

        if card == 'T' {
            return 10;
        }

        if card == 'J' {
            return 11;
        }

        if card == 'Q' {
            return 12;
        }

        if card == 'K' {
            return 13;
        }

        if card == 'A' {
            return 14;
        }

        unreachable!();
    }
}

fn first() {
    let sorter = FirstSorter::new();
    solve(&sorter);
}

struct SecondSorter {}

impl SecondSorter {
    pub fn new() -> Self {
        return Self {};
    }
}

impl Sorter for SecondSorter {
    fn get_value_of_hand(&self, hand: &Vec<char>) -> i64 {
        let mut hand = hand.clone();
        hand.sort();

        let mut wildcards: i64 = 0;
        let mut counts: Vec<i64> = Vec::with_capacity(5);
        for idx in 0..hand.len() {
            if hand[idx] == 'J' {
                wildcards += 1;
                continue;
            }
            if idx == 0 || hand[idx - 1] != hand[idx] {
                counts.push(1);
            } else {
                let size = counts.len();
                counts[size - 1] += 1;
            }
        }
        counts.sort_by(|a, b| b.cmp(a));
        if counts.len() == 0 {
            counts.push(0);
        }
        counts[0] += wildcards;

        if counts[0] == 5 {
            return 7;
        }
        if counts[0] == 4 && counts[1] == 1 {
            return 6;
        }
        if counts[0] == 3 && counts[1] == 2 {
            return 5;
        }
        if counts[0] == 3 {
            return 4;
        }
        if counts[0] == 2
            && counts[1] == 2
        {
            return 3;
        }
        if counts[0] == 2 {
            return 2;
        }
        return 1;
    }

    fn get_value_of_card(&self, card: char) -> i64 {
        if card.is_digit(10) {
            return card as i64 - '0' as i64;
        }

        if card == 'T' {
            return 10;
        }

        if card == 'J' {
            return 1;
        }

        if card == 'Q' {
            return 12;
        }

        if card == 'K' {
            return 13;
        }

        if card == 'A' {
            return 14;
        }

        unreachable!();
    }
}

fn second() {
    let sorter = SecondSorter::new();
    solve(&sorter);
}

fn solve<S: Sorter>(sorter: &S) {
    let line_iterator = read_lines("data/2023/07/input.txt");

    let mut data: Vec<(Vec<char>, i64)> = line_iterator.map(parse_line).collect::<Vec<_>>();
    data.sort_by_key(|(h, _)| get_sort_key(sorter, h));

    let result: i64 = data
        .iter()
        .enumerate()
        .map(|(idx, (_, b))| (idx as i64 + 1) * b)
        .sum();

    println!("{}", result);
}

fn parse_line(line: String) -> (Vec<char>, i64) {
    let h = line[..5].to_owned().chars().collect::<Vec<_>>();
    let b = line[6..].parse::<i64>().unwrap();
    return (h, b);
}

fn get_sort_key<S: Sorter>(sorter: &S, hand: &Vec<char>) -> i64 {
    return sorter.get_value_of_hand(hand) * (10 as i64).pow(10)
        + sorter.get_value_of_card(hand[0]) * (10 as i64).pow(8)
        + sorter.get_value_of_card(hand[1]) * (10 as i64).pow(6)
        + sorter.get_value_of_card(hand[2]) * (10 as i64).pow(4)
        + sorter.get_value_of_card(hand[3]) * (10 as i64).pow(2)
        + sorter.get_value_of_card(hand[4]);
}
