use lib::reader::read_lines;

extern crate lib;

pub fn main() {
    first();
}

fn first() {
    let mut total: i64 = 0;
    let limits: [i64; 3] = [12, 13, 14];
    for line in read_lines("data/2023/02/input.txt") {
        let maybe_value = get_id_if_possible(&limits, line);
        if let Some(value) = maybe_value {
            total += value;
        }
    }
    println!("{:?}", total);
}

fn get_id_if_possible(limits: &[i64; 3], line: String) -> Option<i64> {
    let main_split = line.find(':').unwrap();
    // Find ID
    let id_start = line.find(' ').unwrap() + 1;
    let game_id: i64 = line[id_start..main_split].parse().unwrap();

    // Test limits
    let rounds = line[(main_split + 1)..].split(";");
    for round in rounds {
        for information in round.split(',').map(|s| s.trim()) {
            let information_split = information.find(' ').unwrap();
            let amount: i64 = information[0..information_split].parse().unwrap();
            let color = &information[(information_split + 1)..];
            let color_key = get_color_key(&color);
            if amount > limits[color_key] {
                return Option::None;
            }
        }
    }

    return Option::Some(game_id);
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
