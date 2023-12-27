use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

pub fn read_lines<P: AsRef<Path>>(path: P) -> impl Iterator<Item = String> {
    let handler = File::open(path).unwrap();
    let reader = BufReader::new(handler);
    return reader.lines().map(|l| l.unwrap());
}

pub fn read_split<P: AsRef<Path>>(path: P, split_char: char) -> impl Iterator<Item = String> {
    let handler = File::open(path).unwrap();
    let reader = BufReader::new(handler);
    return reader
        .split(split_char as u8)
        .map(|bs| bs.unwrap())
        .map(|bs| String::from_utf8(bs).unwrap());
}
