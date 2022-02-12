#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use num::*;
#[allow(unused_imports)]
use proconio::marker::*;
#[allow(unused_imports)]
use std::collections::*;
use whiteread::parse_line;

struct Solver {
    n: usize,
    m: usize,
    all_turn: usize,
    cur_turn: usize,
}

impl Solver {
    fn new(pets: Vec<(usize, usize, usize)>, humans: Vec<(usize, usize)>) -> Self {
        Solver {
            n: pets.len(),
            m: humans.len(),
            all_turn: 300,
            cur_turn: 1,
        }
    }
    fn solve(&mut self) {
        for turn in 1..=self.all_turn {
            self.cur_turn = turn;
            println!("{}", ".".repeat(self.m));
            let _: Vec<char> = parse_line().unwrap();
        }
    }
}

fn main() {
    let n: usize = parse_line().unwrap();
    let mut pets = Vec::new();
    for _ in 0..n {
        let (x, y, t): (usize, usize, usize) = parse_line().unwrap();
        pets.push((x - 1, y - 1, t));
    }
    let m: usize = parse_line().unwrap();
    let mut humans = Vec::new();
    for _ in 0..m {
        let (x, y): (usize, usize) = parse_line().unwrap();
        humans.push((x - 1, y - 1));
    }

    let mut solver = Solver::new(pets, humans);
    solver.solve();
}
