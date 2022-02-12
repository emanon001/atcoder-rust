#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use num::*;
#[allow(unused_imports)]
use proconio::marker::*;
#[allow(unused_imports)]
use std::collections::*;
use std::convert::TryFrom;
use whiteread::parse_line;

struct Human {
    i: usize,
    j: usize,
}

struct Pet {
    i: usize,
    j: usize,
    t: PetType,
}

enum PetType {
    Ushi,
    Buta,
    Usagi,
    Inu,
    Neko,
}

impl From<usize> for PetType {
    fn from(t: usize) -> Self {
        match t {
            1 => Self::Ushi,
            2 => Self::Buta,
            3 => Self::Usagi,
            4 => Self::Inu,
            5 => Self::Neko,
            _ => unreachable!(),
        }
    }
}

struct Solver {
    n: usize,
    pets: Vec<Pet>,
    m: usize,
    humans: Vec<Human>,
    all_turn: usize,
    cur_turn: usize,
}

impl Solver {
    fn new(pets: Vec<(usize, usize, usize)>, humans: Vec<(usize, usize)>) -> Self {
        Solver {
            n: pets.len(),
            pets: pets
                .into_iter()
                .map(|p| Pet {
                    i: p.0,
                    j: p.1,
                    t: p.2.into(),
                })
                .collect(),
            m: humans.len(),
            humans: humans
                .into_iter()
                .map(|h| Human { i: p.0, j: p.1 })
                .collect(),
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
