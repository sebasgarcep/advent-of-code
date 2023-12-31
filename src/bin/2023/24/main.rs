extern crate lib;
extern crate ndarray;
extern crate ndarray_linalg;

use lib::reader::read_lines;
use ndarray::{array, s, Array1, Array2};
use ndarray_linalg::Solve;

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

impl SecondSolver {
    /// Positions in matrix correspond to
    /// p_r_x, p_r_y, p_r_z, v_r_x, v_r_y, v_r_z
    fn get_system(e: &Entity) -> (Array2<f64>, Array1<f64>) {
        let a: Array2<f64> = array![
            [0.0, -e.vz, e.vy, 0.0, e.pz, -e.py],
            [e.vz, 0.0, -e.vx, -e.pz, 0.0, e.px],
            [-e.vy, e.vx, 0.0, e.py, -e.px, 0.0],
        ];

        let b: Array1<f64> = array![
            e.py * e.vz - e.pz * e.vy,
            e.pz * e.vx - e.px * e.vz,
            e.px * e.vy - e.py * e.vx,
        ];

        return (a, b);
    }
}

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
    /// (p_r - p_i) = -t_i * (v_r - v_i)
    /// Then (p_r - p_i) and (v_r - v_i) are colinear, which imply that
    /// (p_r - p_i) x (v_r - v_i) = 0
    /// and if we expand out each term of the resulting vector
    /// (p_r - p_i)_y * (v_r - v_i)_z - (p_r - p_i)_z * (v_r - v_i)_y = 0
    /// (p_r - p_i)_z * (v_r - v_i)_x - (p_r - p_i)_x * (v_r - v_i)_z = 0
    /// (p_r - p_i)_x * (v_r - v_i)_y - (p_r - p_i)_y * (v_r - v_i)_x = 0
    /// =>
    /// p_r_y * v_r_z - p_r_y * v_i_z - p_i_y * v_r_z + p_i_y * v_i_z - p_r_z * v_r_y + p_r_z * v_i_y + p_i_z * v_r_y - p_i_z * v_i_y = 0
    /// p_r_z * v_r_x - p_r_z * v_i_x - p_i_z * v_r_x + p_i_z * v_i_x - p_r_x * v_r_z + p_r_x * v_i_z + p_i_x * v_r_z - p_i_x * v_i_z = 0
    /// p_r_x * v_r_y - p_r_x * v_i_y - p_i_x * v_r_y + p_i_x * v_i_y - p_r_y * v_r_x + p_r_y * v_i_x + p_i_y * v_r_x - p_i_y * v_i_x = 0
    /// =>
    /// -p_r_y * v_i_z - p_i_y * v_r_z + p_i_y * v_i_z + p_r_z * v_i_y + p_i_z * v_r_y - p_i_z * v_i_y = -p_r_y * v_r_z + p_r_z * v_r_y
    /// -p_r_z * v_i_x - p_i_z * v_r_x + p_i_z * v_i_x + p_r_x * v_i_z + p_i_x * v_r_z - p_i_x * v_i_z = -p_r_z * v_r_x + p_r_x * v_r_z
    /// -p_r_x * v_i_y - p_i_x * v_r_y + p_i_x * v_i_y + p_r_y * v_i_x + p_i_y * v_r_x - p_i_y * v_i_x = -p_r_x * v_r_y + p_r_y * v_r_x
    /// Take 3 distinct data points i. Notice that the RHS of these equations are equal for all of these. Therefore we can equate the LHS's
    /// to obtain a linear system in 6 unknowns and 9 equations. We can make this a square system by dropping 3 equations, and use a linear
    /// solver to obtain a solution in time. Note that there must exist a choice of 3 data points such that the system has a unique solution,
    /// otherwise there is no solution to the problem.
    fn get_result(entities: Vec<Entity>) -> i64 {
        let i = 0;
        let j = 1;
        let k = 2;
        let (ref a_i, ref b_i) = Self::get_system(&entities[i]);
        let (ref a_j, ref b_j) = Self::get_system(&entities[j]);
        let (ref a_k, ref b_k) = Self::get_system(&entities[k]);
        let mut a: Array2<f64> = Array2::zeros((6, 6));
        a.slice_mut(s![..3, ..]).assign(&(a_i - a_j));
        a.slice_mut(s![3.., ..]).assign(&(a_i - a_k));
        let mut b: Array1<f64> = Array1::zeros(6);
        b.slice_mut(s![..3]).assign(&(b_j - b_i));
        b.slice_mut(s![3..]).assign(&(b_k - b_i));
        let x = a.solve_into(b).unwrap();
        let result = (x[0].round() as i64) + (x[1].round() as i64) + (x[2].round() as i64);
        return result;
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
