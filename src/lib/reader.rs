use std::{
    fs::File,
    io::{BufRead, BufReader}, path::Path,
};

pub fn read_lines<P: AsRef<Path>>(path: P) -> impl Iterator<Item = String> {
    let handler = File::open(path).unwrap();
    let reader = BufReader::new(handler);
    return reader.lines().map(|l| l.unwrap());
}
