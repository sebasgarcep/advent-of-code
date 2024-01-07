extern crate lib;
extern crate ndarray;
extern crate ndarray_linalg;

use lib::reader::read_lines;
use ndarray::{array, Array, Array1, Array2};
use ndarray_linalg::{Solve, Determinant};

pub fn main() {
    first();
    second();
}

fn first() {
    solve::<FirstSolver>();
}

enum FirstSolver {}

impl Solver for FirstSolver {
    fn get_result(entities: Vec<Entity>) -> i64 {
        let mut result = 0;
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
                let matrix = array![[ei.vx, -ej.vx], [ei.vy, -ej.vy]];
                let vector = array![ej.px - ei.px, ej.py - ei.py];
                let x = if let Result::Ok(solution) = matrix.solve_into(vector) {
                    solution
                } else {
                    continue;
                };

                let t = x[0];
                let s = x[1];
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
        return result;
    }
}

fn second() {
    solve::<SecondSolver>();
}

enum SecondSolver {}

impl Solver for SecondSolver {
    /// Let:
    /// - N = number of hailstones in the dataset
    /// - p_i = initial position of the i-th hailstone
    /// - v_i = velocity of the i-th hailstone
    /// - x_i(t) = position of the i-th hailstone at time t
    /// - p_r = initial position of the rock
    /// - v_r = velocity of the rock
    /// - x_r(t) = position of the rock at time t
    /// - t_i = time of impact between the rock and hailstone i
    /// Therefore:
    /// - x_i(t) = p_i + t * v_i
    /// - x_r(t) = p_r + t * v_r
    /// - x_i(t_i) = x_r(t_i)
    /// Which implies:
    /// p_r = p_i - t_i * (v_r - v_i)
    /// Let w_i = t_i * (v_r - v_i).
    /// Thus: p_r = p_i - w_i
    /// And because p_r is stable for each i, for every pair i < j we have:
    /// p_i - w_i = p_j - w_j <-> w_i - w_j = p_i - p_j
    /// Let's suppose we have N data points. Then we have 3N unknowns and
    /// 3N(N-1)/2 equations. Thus for the system to be completely determined
    /// we need at least 7 data points (3N=3N(N-1)/2 <-> N=3). This leads to
    /// the following system of equations. Suppose we want to express this
    /// as Ax=B. Then:
    /// A = (
    ///   1, -1, 0, ..., 0, 0
    ///   1, 0, -1, ..., 0, 0
    ///   ...
    ///   1, 0, 0, ..., 0, -1
    ///   0, 1, -1, ..., 0, 0
    ///   ...
    ///   0, 0, 0, ..., 1, -1
    /// )
    /// [1's and 0's are 3x3 blocks]
    ///
    /// x = (
    ///   w_1
    ///   ...
    ///   w_N
    /// )
    ///
    /// B = (
    ///   p_1 - p_2
    ///   p_1 - p_3
    ///   ...
    ///   p_1 - p_N
    ///   p_2 - p_3
    ///   ...
    ///   p_{N-1} - p_N
    /// )
    fn get_result(entities: Vec<Entity>) -> i64 {
        let n = 3;
        let d = 3;
        let mut a: Array2<f64> = Array::zeros((d * n * (n - 1) >> 1, d * n));
        let mut b: Array1<f64> = Array::zeros(d * n * (n - 1) >> 1);
        let mut pos = 0;
        for i in 0..(n - 1) {
            for j in (i + 1)..n {
                // Set a
                for s in 0..d {
                    for t in 0..d {
                        a[(d * pos + t, d * i + s)] = 1.0;
                        a[(d * pos + t, d * j + s)] = -1.0;
                    }
                }
                // Set b
                b[d * pos + 0] = entities[i].px - entities[j].px;
                b[d * pos + 1] = entities[i].py - entities[j].py;
                b[d * pos + 2] = entities[i].pz - entities[j].pz;
                // Increase counter
                pos += 1;
            }
        }
        println!("d={}", a.det().unwrap());
        return 0;
    }
}

#[derive(Debug)]
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

trait Solver {
    fn get_result(entities: Vec<Entity>) -> i64;
}

fn solve<S: Solver>() {
    let line_collection = read_lines("data/2023/24/input.txt");
    let mut entities = vec![];
    for line in line_collection {
        let entity = Entity::from_line(line);
        entities.push(entity);
    }

    let result = S::get_result(entities);
    println!("{}", result);
}
