extern crate lib;

use lib::reader::read_lines;

fn main() {
    let mut total: i64 = 0;
    let mut maybe_prev_line: Option<String> = Option::None;
    let mut line_iterator = read_lines("data/2023/03/input.txt").peekable();

    while let Some(curr_line) = line_iterator.next() {
        let maybe_next_line = line_iterator.peek();
        let mut curr_number: i64 = 0;
        let mut curr_should_count: bool = false;

        for pos in 0..curr_line.len() {
            let curr_char = get_char_at_position(&curr_line, pos);
            if !curr_char.is_ascii_digit() {
                if curr_should_count {
                    total += curr_number;
                }

                curr_number = 0;
                curr_should_count = false;
                continue;
            }

            curr_number = curr_number * 10 + parse_ascii_digit(curr_char);

            if let Some(ref prev_line) = maybe_prev_line {
                curr_should_count = curr_should_count || should_count_line(prev_line, pos, true);
            }

            if let Some(ref next_line) = maybe_next_line {
                curr_should_count = curr_should_count || should_count_line(next_line, pos, true);
            }

            curr_should_count = curr_should_count || should_count_line(&curr_line, pos, false);
        }

        if curr_should_count {
            total += curr_number;
        }

        maybe_prev_line = Option::Some(curr_line);
    }

    println!("{}", total);
}

fn should_count_line(line: &str, position: usize, count_digits: bool) -> bool {
    return should_count(get_char_at_position(&line, position), count_digits)
        || (position > 0 && should_count(get_char_at_position(&line, position - 1), count_digits))
        || (position < line.len() - 1
            && should_count(get_char_at_position(&line, position + 1), count_digits));
}

fn get_char_at_position(line: &str, position: usize) -> char {
    return line.as_bytes()[position] as char;
}

fn should_count(char: char, count_digits: bool) -> bool {
    if count_digits {
        return char != '.';
    }
    return !char.is_ascii_digit() && char != '.';
}

fn parse_ascii_digit(char: char) -> i64 {
    return char as i64 - '0' as i64;
}
