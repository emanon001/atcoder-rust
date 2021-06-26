#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use num::*;
#[allow(unused_imports)]
use proconio::*;
use proconio::marker::*;
#[allow(unused_imports)]
use std::collections::*;
use rand::prelude::*;
use std::time::{Instant, Duration};

struct Solver {
    n: usize,
    m: usize,
    sv: Vec<Vec<char>>,
    res: Vec<Vec<char>>,
    start_time: Instant,
    rng: ThreadRng,
}

impl Solver {
    fn new() -> Self {
        let now = Instant::now();
        input! {
            n: usize, m: usize,
            sv: [Chars; m]
        };
        let res = vec![vec!['.'; n]; n];
        Self {
            n,
            m,
            sv,
            res,
            start_time: now,
            rng: rand::thread_rng(),
        }
    }

    fn solve(&mut self) {
        // 各行について左から右に配置していく
        self.sv.sort_by_key(|s| s.len());
        let mut used: HashSet<String> = HashSet::new();
        let mut s_i = 0;
        for i in 0..self.n {
            let mut j = 0;
            while s_i < self.m && j + self.sv[s_i].len() < self.n {
                let s = &self.sv[s_i];
                let st = s.iter().join("");
                if used.contains(&st) {
                    s_i += 1;
                    continue;
                }
                let mut contains = false;
                for us in &used {
                    if us.contains(&st) {
                        contains = true;
                        break;
                    }
                }
                if contains {
                    used.insert(st);
                    s_i += 1;
                    continue;
                }
                used.insert(st);
                for s_j in 0..s.len() {
                    self.res[i][j + s_j] = s[s_j];
                }
                s_i += 1;
                j += s.len();
            }
        }
        for s in &self.res {
            println!("{}", s.iter().join(""));
        }
    }
}

fn main() {
    let mut solver = Solver::new();
    solver.solve();
}
