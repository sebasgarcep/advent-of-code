extern crate lib;

use lib::reader::read_lines;

pub fn main() {
    first();
    second();
}

fn first() {
    solve();
}

fn second() {}

#[derive(PartialEq, Clone, Copy)]
enum Tile {
    Empty,
    VerticalSplitter,
    HorizontalSplitter,
    LeftMirror,
    RightMirror,
}

#[derive(PartialEq, Clone, Copy)]
enum Direction {
    North,
    West,
    South,
    East,
}

impl Direction {
    fn get_bit(&self) -> u8 {
        return match self {
            Direction::North => 1,
            Direction::West => 2,
            Direction::South => 4,
            Direction::East => 8,
        };
    }
}

fn solve() {
    let line_collection = read_lines("data/2023/16/input.txt");

    /* Indexed map[j][i] */
    let map: Vec<Vec<Tile>> = line_collection
        .map(|row| {
            row.chars()
                .map(|c| match c {
                    '.' => Tile::Empty,
                    '|' => Tile::VerticalSplitter,
                    '-' => Tile::HorizontalSplitter,
                    '\\' => Tile::LeftMirror,
                    '/' => Tile::RightMirror,
                    _ => {
                        unreachable!();
                    }
                })
                .collect()
        })
        .collect();

    /* Indexed energy[j][i] */
    let mut energy: Vec<Vec<u8>> = map
        .iter()
        .map(|row| row.iter().map(|_| 0).collect())
        .collect();

    let height = map.len() as i64;
    let width = map[0].len() as i64;

    let mut candidates: Vec<(i64, i64, Direction)> = Vec::with_capacity(256);
    candidates.push((0, 0, Direction::East));

    while let Some((start_i, start_j, direction)) = candidates.pop() {
        let mut i = start_i;
        let mut j = start_j;
        loop {
            if energy[j as usize][i as usize] & direction.get_bit() != 0 {
                break;
            }
            energy[j as usize][i as usize] |= direction.get_bit();
            if !can_treat_as_empty(&map, i, j, direction) {
                break;
            }
            match direction {
                Direction::North => {
                    if j > 0 {
                        j -= 1;
                    } else {
                        break;
                    }
                }
                Direction::West => {
                    if i > 0 {
                        i -= 1;
                    } else {
                        break;
                    }
                }
                Direction::South => {
                    if j < height - 1 {
                        j += 1;
                    } else {
                        break;
                    }
                }
                Direction::East => {
                    if i < width - 1 {
                        i += 1;
                    } else {
                        break;
                    }
                }
            };
        }

        match map[j as usize][i as usize] {
            Tile::VerticalSplitter => match direction {
                Direction::West | Direction::East => {
                    propose_candidate(&mut candidates, width, height, i, j - 1, Direction::North);
                    propose_candidate(&mut candidates, width, height, i, j + 1, Direction::South);
                }
                _ => {}
            },
            Tile::HorizontalSplitter => match direction {
                Direction::North | Direction::South => {
                    propose_candidate(&mut candidates, width, height, i - 1, j, Direction::West);
                    propose_candidate(&mut candidates, width, height, i + 1, j, Direction::East);
                }
                _ => {}
            },
            Tile::LeftMirror => match direction {
                Direction::North => {
                    propose_candidate(&mut candidates, width, height, i - 1, j, Direction::West);
                }
                Direction::West => {
                    propose_candidate(&mut candidates, width, height, i, j - 1, Direction::North);
                }
                Direction::South => {
                    propose_candidate(&mut candidates, width, height, i + 1, j, Direction::East);
                }
                Direction::East => {
                    propose_candidate(&mut candidates, width, height, i, j + 1, Direction::South);
                }
            },
            Tile::RightMirror => match direction {
                Direction::North => {
                    propose_candidate(&mut candidates, width, height, i + 1, j, Direction::East);
                }
                Direction::West => {
                    propose_candidate(&mut candidates, width, height, i, j + 1, Direction::South);
                }
                Direction::South => {
                    propose_candidate(&mut candidates, width, height, i - 1, j, Direction::West);
                }
                Direction::East => {
                    propose_candidate(&mut candidates, width, height, i, j - 1, Direction::North);
                }
            },
            _ => {}
        };
    }

    let mut result: i64 = 0;
    for j in 0..height {
        for i in 0..width {
            if energy[j as usize][i as usize] != 0 {
                result += 1;
            }
        }
    }
    println!("{}", result);
}

fn can_treat_as_empty(map: &Vec<Vec<Tile>>, i: i64, j: i64, direction: Direction) -> bool {
    return match map[j as usize][i as usize] {
        Tile::Empty => true,
        Tile::VerticalSplitter => match direction {
            Direction::North | Direction::South => true,
            _ => false,
        },
        Tile::HorizontalSplitter => match direction {
            Direction::West | Direction::East => true,
            _ => false,
        },
        _ => false,
    };
}

fn propose_candidate(
    candidates: &mut Vec<(i64, i64, Direction)>,
    width: i64,
    height: i64,
    i: i64,
    j: i64,
    direction: Direction,
) {
    if i < 0 || i >= width || j < 0 || j >= height {
        return;
    }
    candidates.push((i, j, direction));
}
