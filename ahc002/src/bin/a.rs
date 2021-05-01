#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use num::*;
use proconio::input;
#[allow(unused_imports)]
use proconio::marker::*;
#[allow(unused_imports)]
use std::collections::*;
use std::time::{Instant, Duration};

struct Program {
    pos: (usize, usize),
    path: Vec<(usize, usize, char)>,
    used: Vec<bool>,
    score: i32,
    check_count: u32,
    res: (i32, Vec<(usize, usize, char)>),
    tgrid: Vec<usize>,
    pgrid: Vec<i32>,
    now: Instant,
}

impl Program {
    fn new(si: usize, sj: usize, tgrid: Vec<usize>, pgrid: Vec<i32>) -> Self {
        let now = Instant::now();
        let path = Vec::new();
        let mut used = vec![false; 50 * 50];
        used[tgrid[si * 50 + sj]] = true;
        let check_count = 0;
        let score = pgrid[si * 50 + sj];
        let res = (0, vec![(si, sj, ' ')]);
        Self {
            pos: (si, sj),
            path,
            used,
            score,
            check_count,
            res,
            tgrid,
            pgrid,
            now
        }
    }

    fn dfs(&mut self) {
        if self.check_count == 1000 {
            let duration = Instant::now() - self.now;
            let stop = duration >= Duration::from_millis(1990);
            if stop {
                if self.score > self.res.0 {
                    self.res.0 = self.score;
                    self.res.1 = self.path.clone();
                }
                return;
            }
            self.check_count = 0;
        }
        self.check_count += 1;
        let mut dirs = vec![
            (-1, 0, 'U'),
            (0, 1, 'R'),
            (1, 0, 'D'),
            (0, -1, 'L')
        ];
        dirs.sort_by_key(|(i, j, _)| {{
            let new_i = self.pos.0 as isize + i;
            if new_i < 0 || new_i >= 50 {
                return usize::max_value();
            }
            let new_j = self.pos.1 as isize + j;
            if new_j < 0 || new_j >= 50 {
                return usize::max_value();
            }
            let new_i = new_i as usize;
            let new_j = new_j as usize;
            if self.used[self.tgrid[new_i * 50 + new_j]] {
                return usize::max_value();
            }
            new_i.min(49 - new_i) + new_j.min(49 - new_j)
        }});
        let mut moved = false;
        for (add_i, add_j, d) in dirs {
            let new_i = self.pos.0 as isize + add_i;
            if new_i < 0 || new_i >= 50 {
                continue;
            }
            let new_j = self.pos.1 as isize + add_j;
            if new_j < 0 || new_j >= 50 {
                continue;
            }
            let new_i = new_i as usize;
            let new_j = new_j as usize;
            if self.used[self.tgrid[new_i * 50 + new_j]] {
                continue;
            }
            moved = true;
            let new_pos = (new_i, new_j);
            let saved_pos = self.pos;
            self.pos = new_pos;
            self.score += self.pgrid[new_pos.0 * 50 + new_pos.1];
            self.path.push((new_i, new_j, d));
            self.used[self.tgrid[new_i * 50 + new_j]] = true;
            self.dfs();
            self.pos = saved_pos;
            self.score -= self.pgrid[new_pos.0 * 50 + new_pos.1];
            self.path.pop();
            self.used[self.tgrid[new_i * 50 + new_j]] = false;
        }
        if !moved {
            if self.score > self.res.0 {
                self.res.0 = self.score;
                self.res.1 = self.path.clone();
            }
        }
    }
}

fn solve() {
    input! {
        si: usize, sj: usize,
        tgrid: [usize; 50 * 50],
        pgrid: [i32; 50 * 50],
    };

    let mut prog = Program::new(si, sj, tgrid, pgrid);
    prog.dfs();
    let res = prog.res.1.iter().map(|(_, _, d)| *d).join("");
    println!("{}", res.trim());
}

fn main() {
    std::thread::Builder::new()
        .name("big stack size".into())
        .stack_size(256 * 1024 * 1024)
        .spawn(|| {
            solve();
        })
        .unwrap()
        .join()
        .unwrap();
}
