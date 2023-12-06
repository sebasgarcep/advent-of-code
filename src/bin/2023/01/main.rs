extern crate lib;

use std::fs::File;
use std::io::{BufRead, BufReader};

use lib::trie::Trie;

fn build_trie(words: Vec<(String, i64)>) -> Trie {
    let mut trie = Trie::new();
    for (word, value) in words {
        trie.insert(word, value);
    }
    return trie;
}

fn get_lines() -> impl Iterator<Item = String> {
    let handler = File::open("data/2023/01/input.txt").unwrap();
    let reader = BufReader::new(handler);
    return reader.lines().map(|l| l.unwrap());
}

fn search_digits(trie: &Trie, char_vec: &Vec<char>, pos: usize) -> Option<i64> {
    let mut offset: usize = 0;
    let mut current_pointer = trie.get_root();

    while pos + offset < char_vec.len() {
        let char = char_vec[pos + offset];
        let maybe_child_pointer = trie.get_child(&current_pointer, char);
        if maybe_child_pointer.is_none() {
            return Option::None;
        }

        let child_pointer = maybe_child_pointer.unwrap();
        let value = trie.get_value(&child_pointer);
        if value.is_some() {
            return value;
        }

        offset += 1;
        current_pointer = child_pointer;
    }

    return Option::None;
}

fn get_digits(trie: &Trie, line: String) -> (i64, i64) {
    let char_vec: Vec<_> = line.chars().collect();

    let mut first_digit: Option<i64> = Option::None;
    let mut second_digit: Option<i64> = Option::None;

    for pos in 0..char_vec.len() {
        let value = search_digits(&trie, &char_vec, pos);
        if value.is_some() {
            if first_digit.is_none() {
                first_digit = value;
            }
            second_digit = value;
        }
    }

    if first_digit.is_none() || second_digit.is_none() {
        println!("Found problem with line: {}", line);
        unreachable!();
    }

    return (first_digit.unwrap(), second_digit.unwrap());
}

fn get_value(words: Vec<(String, i64)>) -> i64 {
    let mut total: i64 = 0;
    let trie = build_trie(words);
    for line in get_lines() {
        let (first_digit, second_digit) = get_digits(&trie, line);
        total += first_digit * 10 + second_digit;
    }
    return total;
}

fn first() {
    let words_first: Vec<(String, i64)> = vec![
        ("0".to_owned(), 0),
        ("1".to_owned(), 1),
        ("2".to_owned(), 2),
        ("3".to_owned(), 3),
        ("4".to_owned(), 4),
        ("5".to_owned(), 5),
        ("6".to_owned(), 6),
        ("7".to_owned(), 7),
        ("8".to_owned(), 8),
        ("9".to_owned(), 9),
    ];
    println!("{}", get_value(words_first));
}

fn second() {
    let words_second: Vec<(String, i64)> = vec![
        ("0".to_owned(), 0),
        ("1".to_owned(), 1),
        ("2".to_owned(), 2),
        ("3".to_owned(), 3),
        ("4".to_owned(), 4),
        ("5".to_owned(), 5),
        ("6".to_owned(), 6),
        ("7".to_owned(), 7),
        ("8".to_owned(), 8),
        ("9".to_owned(), 9),
        ("one".to_owned(), 1),
        ("two".to_owned(), 2),
        ("three".to_owned(), 3),
        ("four".to_owned(), 4),
        ("five".to_owned(), 5),
        ("six".to_owned(), 6),
        ("seven".to_owned(), 7),
        ("eight".to_owned(), 8),
        ("nine".to_owned(), 9),
    ];
    println!("{}", get_value(words_second));
}

fn main() {
    first();
    second();
}
