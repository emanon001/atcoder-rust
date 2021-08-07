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
    visited: HashSet<(usize, usize)>,
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
            visited: HashSet::new(),
            start_time: now,
            rng: rand::thread_rng(),
        }
    }

    fn solve(&mut self) {
        self.visited.insert((self.si, self.sj));
        self.move_vertical(self.si, self.sj);
        self.move_horizontal(self.si, self.sj);
        println!("{}", self.path.iter().join(""));
    }

    fn move_vertical(&mut self, i: usize, j: usize) {
        // 上
        if i > 0 {
            let mut step = 0;
            for move_i in (0..i).rev() {
                if self.grid[move_i][j] == '#' {
                    break;
                }
                if self.visited.contains(&(move_i, j)) {
                    break;
                }
                self.visited.insert((move_i, j));
                step += 1;
                self.path.push('U');
                self.move_horizontal(move_i, j);
            }
            for _ in 0..step {
                self.path.push('D');
            }
        }
        // 下
        if i < self.n - 1 {
            let mut step = 0;
            for move_i in i + 1..self.n {
                if self.grid[move_i][j] == '#' {
                    break;
                }
                if self.visited.contains(&(move_i, j)) {
                    break;
                }
                self.visited.insert((move_i, j));
                step += 1;
                self.path.push('D');
                self.move_horizontal(move_i, j);
            }
            for _ in 0..step {
                self.path.push('U');
            }
        }
    }

    fn move_horizontal(&mut self, i: usize, j: usize) {
        // 左
        if j > 0 {
            let mut step = 0;
            for move_j in (0..j).rev() {
                if self.grid[i][move_j] == '#' {
                    break;
                }
                if self.visited.contains(&(i, move_j)) {
                    break;
                }
                self.visited.insert((i, move_j));
                step += 1;
                self.path.push('L');
                self.move_vertical(i, move_j);
            }
            for _ in 0..step {
                self.path.push('R');
            }
        }
        // 右
        if j < self.n - 1 {
            let mut step = 0;
            for move_j in j + 1..self.n {
                if self.grid[i][move_j] == '#' {
                    break;
                }
                if self.visited.contains(&(i, move_j)) {
                    break;
                }
                self.visited.insert((i, move_j));
                step += 1;
                self.path.push('R');
                self.move_vertical(i, move_j);
            }
            for _ in 0..step {
                self.path.push('L');
            }
        }
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
