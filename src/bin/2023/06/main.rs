extern crate lib;

use lib::reader::read_lines;

const SKIP_TIME_LINE: usize = 5;
const SKIP_DISTANCE_LINE: usize = 9;

/*
Let t be the time the button is pressed, T the race time and D the least amount
of distance that needs to be covered (The distance of the previous record holder
plus one). Let d be the distance covered. Then d = (T-t)*t. But d has to be
greater than or equal to D. Therefore we have an inequality for t given by:
D <= d = (T-t)*t <-> t^2 - T*t + D <= 0 which can be solved using the
quadratic formula: (T - sqrt(T^2 - 4D))/2 <= t <= (T + sqrt(T^2 - 4D))/2
*/
pub fn main() {
    first();
    second();
}

struct Solver {
    pub total: i64,
}

impl Solver {
    pub fn new() -> Self {
        return Self { total: 1 };
    }

    fn consume_race(&mut self, race_time: f64, race_distance: f64) {
        let sqrt_disc = (race_time * race_time - 4.0 * race_distance).sqrt();
        let lower_bound: i64 = ((race_time - sqrt_disc) / 2.0).ceil() as i64;
        let upper_bound: i64 = ((race_time + sqrt_disc) / 2.0).floor() as i64;
        self.total *= std::cmp::max(1, upper_bound - lower_bound + 1);
    }
}

/**
 * FIXME: Can we make this trait work by returning iterators instead of vecs?
 */
trait Parser {
    fn parse(&self, time_line: String, distance_line: String) -> Vec<(f64, f64)>;
}

struct FirstParser {}

impl FirstParser {
    pub fn new() -> Self {
        return Self {};
    }
}

impl Parser for FirstParser {
    fn parse(&self, time_line: String, distance_line: String) -> Vec<(f64, f64)> {
        let time_stream = time_line
            .split_whitespace()
            .map(|v| v.parse::<f64>().unwrap());
        let distance_stream = distance_line
            .split_whitespace()
            .map(|v| v.parse::<f64>().unwrap() + 1.0);
        return time_stream.zip(distance_stream).collect();
    }
}

struct SecondParser {}

impl SecondParser {
    pub fn new() -> Self {
        return Self {};
    }
}

impl Parser for SecondParser {
    fn parse(&self, time_line: String, distance_line: String) -> Vec<(f64, f64)> {
        let time_data = time_line.replace(" ", "").parse::<f64>().unwrap();
        let distance_data = distance_line.replace(" ", "").parse::<f64>().unwrap() + 1.0;
        return vec![(time_data, distance_data)]
    }
}

fn first() {
    let parser = FirstParser::new();
    iterate(&parser);
}

fn second() {
    let parser = SecondParser::new();
    iterate(&parser);
}

fn iterate<P: Parser>(parser: &P) {
    let mut line_iterator = read_lines("./data/2023/06/input.txt");
    let time_line = line_iterator.next().unwrap();
    let time_line = time_line[SKIP_TIME_LINE..].trim().to_owned();
    let distance_line = line_iterator.next().unwrap();
    let distance_line = distance_line[SKIP_DISTANCE_LINE..].trim().to_owned();
    let mut solver = Solver::new();
    for (race_time, race_distance) in parser.parse(time_line, distance_line) {
        solver.consume_race(race_time, race_distance);
    }
    println!("{}", solver.total);
}
