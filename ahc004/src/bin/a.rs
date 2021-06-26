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

const SCORE_BASE: f64 = 100000000_f64;

struct Solver {
    n: usize,
    m: usize,
    sv: Vec<Vec<char>>,
    grid: Vec<Vec<char>>,
    horizontal_used_count: Vec<HashMap<usize, i64>>,
    vertical_used_count: Vec<HashMap<usize, i64>>,
    used_count: HashMap<usize, i64>,
    dot_count: usize,
    score: f64,
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
            score: 0_f64,
            dot_count: n * n,
            start_time: now,
            rng: rand::thread_rng(),
        }
    }

    fn solve(&mut self) {
        self.gen_initial_grid();
        self.init_score();

        let chars = vec![
            'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H'
        ];
        // let prob_inf = 1000000000;
        // let start_temp = 50_f64;
        // let end_temp = 10_f64;

        let mut loop_count: u64 = 0;
        let mut now_time = Instant::now();
        let duration = Duration::from_millis(2950);
        loop {
            if loop_count % 100 == 0 {
                now_time = Instant::now();
                if now_time > self.start_time + duration {
                    break;
                }
            }
            loop_count += 1;
            let i = self.rng.gen::<usize>() % self.n;
            let j = self.rng.gen::<usize>() % self.n;
            let ch = if self.used_count.len() < self.m {
                chars[self.rng.gen::<usize>() % chars.len()]
            } else {
                '.'
            };
            let cur_score = self.score;
            let (new_score, horizontal_count, vertical_count, updated_count) = self.calc_score(i, j, ch);

            // let temp = start_temp + (end_temp - start_temp) * ((now_time - self.start_time).as_secs_f64() / duration.as_secs_f64()) as f64;
            // let prob = ((new_score - cur_score) / temp).exp();
            // if prob > (self.rng.gen::<usize>() % prob_inf) as f64 / prob_inf as f64 {
            if new_score >= cur_score {
                self.update_score(i, j, ch, new_score, horizontal_count, vertical_count, updated_count);
            }
        }
        for s in &self.grid {
            println!("{}", s.iter().join(""));
        }
    }

    fn calc_score(&self, update_i: usize, update_j: usize, ch: char) -> (f64, HashMap<usize, i64>, HashMap<usize, i64>, HashMap<usize, i64>) {
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
                *updated_count.entry(s_i).or_insert(0) += nc - c;
            } else {
                *updated_count.entry(s_i).or_insert(0) -= c;
            }
        }
        for (&s_i, &c) in &new_horizontal_count {
            if !self.horizontal_used_count[update_i].contains_key(&s_i) {
                *updated_count.entry(s_i).or_insert(0) += c;
            }
        }

        for (&s_i, &c) in &self.vertical_used_count[update_j] {
            if let Some(nc) = new_vertical_count.get(&s_i) {
                *updated_count.entry(s_i).or_insert(0) += nc - c;
            } else {
                *updated_count.entry(s_i).or_insert(0) -= c;
            }
        }
        for (&s_i, &c) in &new_vertical_count {
            if !self.vertical_used_count[update_j].contains_key(&s_i) {
                *updated_count.entry(s_i).or_insert(0) += c;
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

        let c = self.used_count.len() as i64 + add_count;
        let d = (self.dot_count as i64 + if self.grid[update_i][update_j] == '.' && ch != '.' {
            -1
        } else if self.grid[update_i][update_j] != '.' && ch == '.' {
            1
        } else {
            0
        }) as usize;
        let score = self._calc_score(c as usize, d);
        (score, new_horizontal_count, new_vertical_count, updated_count)
    }

    fn update_score(&mut self, i: usize, j: usize, ch: char, score: f64,
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
        self.dot_count = (self.dot_count as i64 + if self.grid[i][j] == '.' && ch != '.' {
            -1
        } else if self.grid[i][j] != '.' && ch == '.' {
            1
        } else {
            0
        }) as usize;
    }

    fn _calc_score(&self, c: usize, d: usize) -> f64 {
        let n = self.n as i64;
        if c == self.m {
            ((SCORE_BASE * (2 * n.pow(2)) as f64)) / ((2 * n.pow(2) - d as i64) as f64).round()
        } else {
            (SCORE_BASE * c as f64 / self.m as f64).round()
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
        self.score = self._calc_score(self.used_count.len(), self.dot_count).round();
    }

    fn gen_initial_grid(&mut self) {
        self.sv.sort_by_key(|s| -(s.len() as isize));

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
        let mut dot_count = 0;
        for i in 0..self.n {
            for j in 0..self.n {
                if self.grid[i][j] == '.' {
                    dot_count += 1;
                }
            }
        }
        self.dot_count = dot_count;
    }

}

fn main() {
    let mut solver = Solver::new();
    solver.solve();
}
