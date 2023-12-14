extern crate lib;

use lib::reader::read_lines;

pub fn main() {
    let line_iterator = read_lines("data/2023/07/input.txt");

    let mut data = line_iterator.map(parse_line).collect::<Vec<_>>();
    data.sort_by_key(get_sort_key);

    let result: i64 = data.iter().enumerate().map(|(idx, (_, b))| (idx as i64 + 1) * b).sum();
    println!("{}", result);
}

fn parse_line(line: String) -> (Vec<char>, i64) {
    let h = line[..5].to_owned().chars().collect::<Vec<_>>();
    let b = line[6..].parse::<i64>().unwrap();
    return (h, b);
}

fn get_sort_key(t: &(Vec<char>, i64)) -> i64 {
    let (h, _) = t;
    return get_value_of_hand(h) * (10 as i64).pow(10)
        + get_value_of_card(h[0]) * (10 as i64).pow(8)
        + get_value_of_card(h[1]) * (10 as i64).pow(6)
        + get_value_of_card(h[2]) * (10 as i64).pow(4)
        + get_value_of_card(h[3]) * (10 as i64).pow(2)
        + get_value_of_card(h[4]);
}

fn get_value_of_hand(hand: &Vec<char>) -> i64 {
    let mut hand = hand.clone();
    hand.sort();

    let mut counts = Vec::with_capacity(5);
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

fn get_value_of_card(card: char) -> i64 {
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
