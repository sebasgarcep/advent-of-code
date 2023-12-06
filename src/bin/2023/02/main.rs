use std::cmp::max;

use lib::reader::read_lines;

extern crate lib;

pub fn main() {
    first();
    second();
}

fn first() {
    let mut total: i64 = 0;
    let limits: [i64; 3] = [12, 13, 14];
    for game in read_lines("data/2023/02/input.txt") {
        let maybe_value = get_id_if_possible(&limits, &game);
        if let Some(value) = maybe_value {
            total += value;
        }
    }
    println!("{:?}", total);
}

fn second() {
    let mut total: i64 = 0;
    for game in read_lines("data/2023/02/input.txt") {
        let mut item: i64 = 1;
        let values = get_minimum_required(&game);
        for idx in 0..3 {
            item *= values[idx];
        }
        total += item;
    }
    println!("{:?}", total);
}

fn parse_game_id(game: &str) -> i64 {
    let main_split = game.find(':').unwrap();
    let id_start = game.find(' ').unwrap() + 1;
    let game_id: i64 = game[id_start..main_split].parse().unwrap();
    return game_id;
}

fn parse_game(game: &str) -> impl Iterator<Item = [i64; 3]> + '_ {
    let main_split = game.find(':').unwrap();
    let rounds = game[(main_split + 1)..].split(";");
    return rounds.map(|round| parse_round(round));
}

fn parse_round(round: &str) -> [i64; 3] {
    let mut result = [0; 3];
    for information in round.split(',').map(|s| s.trim()) {
        let information_split = information.find(' ').unwrap();
        let amount: i64 = information[0..information_split].parse().unwrap();
        let color = &information[(information_split + 1)..];
        let color_key = get_color_key(&color);
        result[color_key] = amount;
    }
    return result;
}

fn get_id_if_possible(limits: &[i64; 3], game: &str) -> Option<i64> {
    for round in parse_game(game) {
        for idx in 0..3 {
            if round[idx] > limits[idx] {
                return Option::None;
            }
        }
    }

    return Option::Some(parse_game_id(game));
}

fn get_minimum_required(game: &str) -> [i64; 3] {
    let mut result = [0; 3];
    for round in parse_game(game) {
        for idx in 0..3 {
            result[idx] = max(result[idx], round[idx]);
        }
    }
    return result;
}

fn get_color_key(key: &str) -> usize {
    if key == "red" {
        return 0;
    }
    if key == "green" {
        return 1;
    }
    if key == "blue" {
        return 2;
    }
    unreachable!();
}
