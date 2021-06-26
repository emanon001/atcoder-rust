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
    grid: Vec<Vec<char>>,
    horizontal_used_count: Vec<HashMap<usize, i64>>,
    vertical_used_count: Vec<HashMap<usize, i64>>,
    used_count: HashMap<usize, i64>,
    score: i64,
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
        let grid = vec![vec!['.'; n]; n];
        Self {
            n,
            m,
            sv,
            grid,
            used_count: HashMap::new(),
            horizontal_used_count: vec![HashMap::new(); m],
            vertical_used_count: vec![HashMap::new(); m],
            score: 0,
            start_time: now,
            rng: rand::thread_rng(),
        }
    }

    fn solve(&mut self) {
        self.sv.sort_by_key(|s| s.len());

        self.gen_initial_grid();
        self.init_score();

        let chars = vec![
            'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H'
        ];
        let mut loop_count: u64 = 0;
        loop {
            if loop_count % 100 == 0 && Instant::now() > self.start_time + Duration::from_millis(2950) {
                break;
            }
            loop_count += 1;
            let i = self.rng.gen::<usize>() % self.n;
            let j = self.rng.gen::<usize>() % self.n;
            let ch = chars[self.rng.gen::<usize>() % 8];
            let cur_score = self.score;
            let (new_score, horizontal_count, vertical_count, updated_count) = self.calc_score(i, j, ch);
            if new_score >= cur_score {
                self.update_score(i, j, ch, new_score, horizontal_count, vertical_count, updated_count);
            }
        }
        for s in &self.grid {
            println!("{}", s.iter().join(""));
        }
    }

    fn calc_score(&self, update_i: usize, update_j: usize, ch: char) -> (i64, HashMap<usize, i64>, HashMap<usize, i64>, HashMap<usize, i64>) {
        let n = self.n;
        let mut new_horizontal_count = HashMap::new();
        // 横
        for j in 0..n {
            for s_i in 0..self.sv.len() {
                let s = &self.sv[s_i];
                let mut is_ok = true;
                for s_j in 0..s.len() {
                    let gj = (j + s_j) % self.n;
                    let gch = if update_j == gj {
                        ch
                    } else {
                        self.grid[update_i][gj]
                    };
                    if s[s_j] != gch {
                        is_ok = false;
                        break;
                    }
                }
                if is_ok {
                    *new_horizontal_count.entry(s_i).or_insert(0) += 1;
                }
            }
        }

        // 縦
        let mut new_vertical_count = HashMap::new();
        for i in 0..n {
            for s_i in 0..self.m {
                let s = &self.sv[s_i];
                let mut is_ok = true;
                for s_j in 0..s.len() {
                    let gi = (i + s_j) % self.n;
                    let gch = if update_i == gi {
                        ch
                    } else {
                        self.grid[gi][update_j]
                    };
                    if s[s_j] != gch {
                        is_ok = false;
                        break;
                    }
                }
                if is_ok {
                    *new_vertical_count.entry(s_i).or_insert(0) += 1;
                }
            }
        }
        let mut updated_count = HashMap::new();
        for (&s_i, &c) in &self.horizontal_used_count[update_i] {
            if let Some(nc) = new_horizontal_count.get(&s_i) {
                *updated_count.entry(s_i).or_insert(0) = nc - c;
            } else {
                *updated_count.entry(s_i).or_insert(0) = -c;
            }
        }
        for (&s_i, &c) in &new_horizontal_count {
            if !self.horizontal_used_count[update_i].contains_key(&s_i) {
                *updated_count.entry(s_i).or_insert(0) = c;
            }
        }

        for (&s_i, &c) in &self.vertical_used_count[update_j] {
            if let Some(nc) = new_vertical_count.get(&s_i) {
                *updated_count.entry(s_i).or_insert(0) = nc - c;
            } else {
                *updated_count.entry(s_i).or_insert(0) = -c;
            }
        }
        for (&s_i, &c) in &new_vertical_count {
            if !self.vertical_used_count[update_j].contains_key(&s_i) {
                *updated_count.entry(s_i).or_insert(0) = c;
            }
        }

        let mut add_count = 0;
        for (&s_i, &c) in &updated_count {
            if let Some(nc) = self.used_count.get(&s_i) {
                if nc + c == 0 {
                    add_count -= 1;
                }
            } else {
                add_count += 1;
            }
        }

        let score = self.used_count.len() as i64 + add_count;
        (score, new_horizontal_count, new_vertical_count, updated_count)
    }

    fn update_score(&mut self, i: usize, j: usize, ch: char, score: i64,
        horizontal_count: HashMap<usize, i64>,
        vertical_count: HashMap<usize, i64>,
        updated_count: HashMap<usize, i64>) {
        self.score = score;
        self.grid[i][j] = ch;
        self.horizontal_used_count[i] = horizontal_count;
        self.vertical_used_count[j] = vertical_count;
        for (s_i, uc) in updated_count {
            if self.used_count.contains_key(&s_i) {
                let c = self.used_count[&s_i];
                let new_c = c + uc;
                if new_c == 0 {
                    self.used_count.remove(&s_i);
                } else {
                    self.used_count.insert(s_i,new_c);
                }
            } else {
                self.used_count.insert(s_i, uc);
            }
        }
    }

    fn init_score(&mut self) {
        let n = self.n;
        // 横
        for i in 0..n {
            for j in 0..n {
                for s_i in 0..self.m {
                    let s = &self.sv[s_i];
                    let mut is_ok = true;
                    for s_j in 0..s.len() {
                        if s[s_j] != self.grid[i][(j + s_j) % self.n] {
                            is_ok = false;
                            break;
                        }
                    }
                    if is_ok {
                        *self.used_count.entry(s_i).or_insert(0) += 1;
                        *self.horizontal_used_count[i].entry(s_i).or_insert(0) += 1;
                    }
                }
            }
        }

        // 縦
        for j in 0..n {
            for i in 0..n {
                for s_i in 0..self.m {
                    let s = &self.sv[s_i];
                    let mut is_ok = true;
                    for s_j in 0..s.len() {
                        if s[s_j] != self.grid[(i + s_j) % self.n][j] {
                            is_ok = false;
                            break;
                        }
                    }
                    if is_ok {
                        *self.used_count.entry(s_i).or_insert(0) += 1;
                        *self.vertical_used_count[j].entry(s_i).or_insert(0) += 1;
                    }
                }
            }
        }
        self.score = self.used_count.len() as i64;
    }

    fn gen_initial_grid(&mut self) {
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
                    self.grid[i][j + s_j] = s[s_j];
                }
                s_i += 1;
                j += s.len();
            }
        }
    }

}

fn main() {
    let mut solver = Solver::new();
    solver.solve();
}
