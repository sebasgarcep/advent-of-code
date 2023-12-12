extern crate lib;

use lib::reader::read_lines;

const GRID_WIDTH: usize = 140;

fn main() {
    first();
    second();
}

#[derive(Copy, Clone, Debug, PartialEq)]
enum SelectedLine {
    Prev,
    Curr,
    Next,
}

trait Consumer {
    fn consume_number(&mut self, number: i64);
    fn consume_line(&mut self);
    fn consume_symbol(&mut self, selected_line: SelectedLine, position: usize, symbol: char);
}

struct FirstConsumer {
    pub total: i64,
    should_consume: bool,
}

impl FirstConsumer {
    pub fn new() -> FirstConsumer {
        return FirstConsumer {
            total: 0,
            should_consume: false,
        };
    }

    fn should_count(symbol: char) -> bool {
        return !symbol.is_ascii_digit() && symbol != '.';
    }
}

impl Consumer for FirstConsumer {
    fn consume_number(&mut self, number: i64) {
        if self.should_consume {
            self.total += number;
            self.should_consume = false;
        }
    }

    fn consume_line(&mut self) {
        self.should_consume = false;
    }

    fn consume_symbol(&mut self, _selected_line: SelectedLine, _position: usize, symbol: char) {
        self.should_consume = self.should_consume || FirstConsumer::should_count(symbol);
    }
}

#[derive(Copy, Clone, Debug)]
struct GearStats {
    count: usize,
    value: i64,
}

impl GearStats {
    pub fn get_empty_line() -> Vec<GearStats> {
        return vec![GearStats { count: 0, value: 1 }; GRID_WIDTH];
    }
}

struct SecondConsumer {
    pub total: i64,
    prev_line: Vec<GearStats>,
    curr_line: Vec<GearStats>,
    next_line: Vec<GearStats>,
    marked_positions: Vec<(SelectedLine, usize)>,
}

impl SecondConsumer {
    pub fn new() -> SecondConsumer {
        return SecondConsumer {
            total: 0,
            prev_line: GearStats::get_empty_line(),
            curr_line: GearStats::get_empty_line(),
            next_line: GearStats::get_empty_line(),
            marked_positions: Vec::with_capacity(16),
        };
    }

    fn should_count(symbol: char) -> bool {
        return symbol == '*';
    }
}

impl Consumer for SecondConsumer {
    fn consume_number(&mut self, number: i64) {
        if number == 0 {
            return;
        }
        for (selected_line, position) in self.marked_positions.iter() {
            match *selected_line {
                SelectedLine::Prev => {
                    self.prev_line[*position].count += 1;
                    self.prev_line[*position].value *= number;
                }
                SelectedLine::Curr => {
                    self.curr_line[*position].count += 1;
                    self.curr_line[*position].value *= number;
                }
                SelectedLine::Next => {
                    self.next_line[*position].count += 1;
                    self.next_line[*position].value *= number;
                }
            }
        }
        self.marked_positions.clear();
    }

    fn consume_line(&mut self) {
        std::mem::swap(&mut self.prev_line, &mut self.curr_line);
        std::mem::swap(&mut self.curr_line, &mut self.next_line);
        let prev_line = std::mem::replace(&mut self.next_line, GearStats::get_empty_line());
        for item in prev_line.iter() {
            if item.count == 2 {
                self.total += item.value;
            }
        }
    }

    fn consume_symbol(&mut self, selected_line: SelectedLine, position: usize, symbol: char) {
        if SecondConsumer::should_count(symbol) {
            let candidate = (selected_line, position);
            if !self.marked_positions.contains(&candidate) {
                self.marked_positions.push(candidate);
            }
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
    let mut maybe_prev_line: Option<String> = Option::None;
    let mut line_iterator = read_lines("data/2023/03/input.txt").peekable();

    while let Some(curr_line) = line_iterator.next() {
        let maybe_next_line = line_iterator.peek();
        let mut curr_number: i64 = 0;

        for pos in 0..GRID_WIDTH {
            let curr_char = get_char_at_position(&curr_line, pos);
            if !curr_char.is_ascii_digit() {
                consumer.consume_number(curr_number);
                curr_number = 0;
                continue;
            }

            curr_number = curr_number * 10 + parse_ascii_digit(curr_char);

            if let Some(ref prev_line) = maybe_prev_line {
                search_symbols_line(consumer, SelectedLine::Prev, prev_line, pos);
            }
            if let Some(ref next_line) = maybe_next_line {
                search_symbols_line(consumer, SelectedLine::Next, next_line, pos);
            }
            search_symbols_line(consumer, SelectedLine::Curr, &curr_line, pos);
        }

        consumer.consume_number(curr_number);
        consumer.consume_line();
        maybe_prev_line = Option::Some(curr_line);
    }

    consumer.consume_line();
}

fn search_symbols_line<C: Consumer>(
    consumer: &mut C,
    selected_line: SelectedLine,
    line: &str,
    position: usize,
) {
    if position > 0 {
        let prev_pos_char = get_char_at_position(&line, position - 1);
        consumer.consume_symbol(selected_line, position - 1, prev_pos_char);
    }

    if position < line.len() - 1 {
        let next_pos_char = get_char_at_position(&line, position + 1);
        consumer.consume_symbol(selected_line, position + 1, next_pos_char);
    }

    let curr_pos_char = get_char_at_position(&line, position);
    consumer.consume_symbol(selected_line, position, curr_pos_char);
}

fn get_char_at_position(line: &str, position: usize) -> char {
    return line.as_bytes()[position] as char;
}

fn parse_ascii_digit(char: char) -> i64 {
    return char as i64 - '0' as i64;
}
