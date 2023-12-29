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

struct Entity {
    px: f64,
    py: f64,
    pz: f64,
    vx: f64,
    vy: f64,
    vz: f64,
}

impl Entity {
    pub fn from_line(mut line: String) -> Self {
        let split_position = line.find(" @ ").unwrap();
        let velocity_line = line.split_off(split_position + 3);
        let mut velocity = velocity_line
            .split(',')
            .map(|s| s.trim().parse::<f64>().unwrap());
        let _ = line.split_off(split_position);
        let mut position = line.split(',').map(|s| s.trim().parse::<f64>().unwrap());
        return Self {
            px: position.next().unwrap(),
            py: position.next().unwrap(),
            pz: position.next().unwrap(),
            vx: velocity.next().unwrap(),
            vy: velocity.next().unwrap(),
            vz: velocity.next().unwrap(),
        };
    }
}

fn solve() {
    let line_collection = read_lines("data/2023/24/input.txt");
    let mut entities = vec![];
    for line in line_collection {
        let entity = Entity::from_line(line);
        entities.push(entity);
    }

    let mut result: usize = 0;
    for j in 1..entities.len() {
        let ej = &entities[j];
        for i in 0..j {
            let ei = &entities[i];
            /*
            x(t) = ei.px + t * ei.vx
            y(t) = ei.py + t * ei.vy
            x(s) = ej.px + s * ej.vx
            y(s) = ej.py + s * ej.vy
            x(t) = x(s)
            y(t) = y(s)
            =>
            ei.px + t * ei.vx = ej.px + s * ej.vx
            ei.py + t * ei.vy = ej.py + s * ej.vy
            =>
            t * ei.vx - s * ej.vx = ej.px - ei.px
            t * ei.vy - s * ej.vy = ej.py - ei.py
            */
            let determinant = -ei.vx * ej.vy + ej.vx * ei.vy;
            if determinant == 0.0 {
                continue;
            }
            let t = (-(ej.px - ei.px) * ej.vy + (ej.py - ei.py) * ej.vx) / determinant;
            let s = (ei.vx * (ej.py - ei.py) - (ej.px - ei.px) * ei.vy) / determinant;
            if t < 0.0 || s < 0.0 {
                continue;
            }
            let x = ei.px + t * ei.vx;
            let y = ei.py + t * ei.vy;
            let bound_min = 200000000000000.0;
            let bound_max = 400000000000000.0;
            if bound_min <= x && x <= bound_max && bound_min <= y && y <= bound_max {
                result += 1;
            }
        }
    }
    println!("{}", result);
}
