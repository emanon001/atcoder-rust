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
    si: usize,
    sj: usize,
    grid: Vec<Vec<char>>,
    r: usize,
    score: f64,
    path: Vec<char>,
    start_time: Instant,
    rng: ThreadRng,
}

impl Solver {
    fn new() -> Self {
        let now = Instant::now();
        input! {
            n: usize, si: usize, sj: usize,
            grid: [Chars; n]
        };
        let mut r = 0;
        for i in 0..n {
            for j in 0..n {
                if grid[i][j] == '.' {
                    r += 1;
                }
            }
        }
        let path = Vec::new();
        Self {
            n,
            si,
            sj,
            grid,
            r,
            score: 0_f64,
            path,
            start_time: now,
            rng: rand::thread_rng(),
        }
    }

    fn solve(&mut self) {
        // let mut loop_count: u64 = 0;
        // let tl = Duration::from_millis(2950);
        // loop {
        //     loop_count += 1;
        //     if loop_count % 100 == 0 {
        //         let d = Instant::now() - self.start_time;
        //         if d >= tl {
        //             break;
        //         }
        //     }
        // }
        // 上下左右にまっすぐ移動して戻ってくる
        if self.si > 0 {
            let mut step = 0;
            for i in (0..self.si).rev() {
                if self.grid[i][self.sj] == '.' {
                    step += 1;
                    self.path.push('U');
                }
            }
            for _ in 0..step {
                self.path.push('D');
            }
        }
        if self.si < self.n - 1 {
            let mut step = 0;
            for i in self.si..self.n - 1 {
                if self.grid[i][self.sj] == '.' {
                    step += 1;
                    self.path.push('D');
                }
            }
            for _ in 0..step {
                self.path.push('U');
            }
        }
        if self.sj > 0 {
            let mut step = 0;
            for j in (0..self.sj).rev() {
                if self.grid[self.si][j] == '.' {
                    step += 1;
                    self.path.push('L');
                }
            }
            for _ in 0..step {
                self.path.push('R');
            }
        }
        if self.sj < self.n - 1 {
            let mut step = 0;
            for j in self.sj..self.n - 1 {
                if self.grid[self.si][j] == '.' {
                    step += 1;
                    self.path.push('R');
                }
            }
            for _ in 0..step {
                self.path.push('L');
            }
        }

        println!("{}", self.path.iter().join(""));
    }

    fn calc_score(&self, t: usize) {
    }

    fn update_score(&mut self) {
    }
}

fn main() {
    let mut solver = Solver::new();
    solver.solve();
}
