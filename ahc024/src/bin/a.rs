#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use num::*;
use proconio::input;
#[allow(unused_imports)]
use proconio::marker::*;
use rand::prelude::*;
#[allow(unused_imports)]
use std::collections::*;
use std::time::{Duration, Instant};

/// 上下左右 (i, j)
pub const UDLR_DIRS: [(isize, isize); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

type Color = usize;

type Pos = (usize, usize);

type Grid = Vec<Vec<usize>>;

type Alignment = Vec<Vec<bool>>;

struct SimulateResult {
    can_update: bool,
    score: isize,
}

struct CityMap {
    raw_n: usize,
    n: usize,
    m: usize,
    grid: Grid,
    alignment: Alignment,
    score: isize,
}

impl CityMap {
    fn new(n: usize, m: usize, grid: Grid) -> Self {
        let new_n = n + 2;
        let mut new_grid = vec![vec![0; new_n]; new_n];
        for i in 0..n {
            for j in 0..n {
                new_grid[i + 1][j + 1] = grid[i][j];
            }
        }
        let alignment = Self::calculate_alignment(new_n, m, &new_grid);
        Self {
            raw_n: n,
            n: new_n,
            m,
            grid: new_grid,
            alignment,
            score: 0,
        }
    }

    fn adjacent_colors(&self, i: usize, j: usize) -> Vec<Color> {
        let i = i + 1;
        let j = j + 1;
        let color = self.grid[i][j];
        let mut res = Vec::new();
        for (c, connected) in self.alignment[color].iter().enumerate() {
            if *connected {
                res.push(c);
            }
        }
        res
    }

    fn adjacent_color_positions(&self, i: usize, j: usize) -> Vec<Pos> {
        let color = self.grid[i][j];
        let mut res = HashSet::new();
        let mut visited = vec![vec![false; self.n]; self.n];
        visited[i][j] = true;
        let mut que = VecDeque::new();
        que.push_back((i, j));
        while let Some((i, j)) = que.pop_front() {
            for (di, dj) in &UDLR_DIRS {
                let new_i = i as isize + di;
                let new_j = j as isize + dj;
                if new_i < 0 || new_i >= self.n as isize || new_j < 0 || new_j >= self.n as isize {
                    continue;
                }
                let new_i = new_i as usize;
                let new_j = new_j as usize;
                let color2 = self.grid[new_i][new_j];
                if color2 != color {
                    res.insert((new_i, new_j));
                    continue;
                }
                if visited[new_i][new_j] {
                    continue;
                }
                visited[new_i][new_j] = true;
                que.push_back((new_i, new_j));
            }
        }
        res.into_iter().collect::<Vec<_>>()
    }

    fn simulate_update(&mut self, i: usize, j: usize, k: usize) -> SimulateResult {
        let i = i + 1;
        let j = j + 1;
        let bk_k = self.grid[i][j];
        self.grid[i][j] = k;
        let new_alignment = Self::calculate_alignment(self.n, self.m, &self.grid);
        let is_connected = Self::is_connected(self.n, self.m, &self.grid);
        self.grid[i][j] = bk_k;
        let score = Self::calculate_score(self.score, bk_k, k);
        SimulateResult {
            can_update: is_connected && self.alignment == new_alignment,
            score,
        }
    }

    fn calculate_score(cur_score: isize, before_color: usize, after_color: usize) -> isize {
        let before_color = before_color.max(1) as isize;
        let after_color = after_color.max(1) as isize;
        (cur_score + before_color - after_color).min(0)
    }

    fn update(&mut self, i: usize, j: usize, k: usize) {
        let i = i + 1;
        let j = j + 1;
        let bk_k = self.grid[i][j];
        self.grid[i][j] = k;
        self.score = Self::calculate_score(self.score, bk_k, k);
    }

    fn output_grid(self) -> Grid {
        let mut res = vec![vec![0; self.raw_n]; self.raw_n];
        for i in 1..self.n - 1 {
            for j in 1..self.n - 1 {
                res[i - 1][j - 1] = self.grid[i][j];
            }
        }
        res
    }

    fn calculate_alignment(n: usize, m: usize, grid: &Grid) -> Alignment {
        let mut alignment = vec![vec![false; m + 1]; m + 1];
        for i in 0..n {
            for j in 0..n {
                let color1 = grid[i][j];
                // 右
                if j + 1 < n {
                    let color2 = grid[i][j + 1];
                    if color1 != color2 {
                        alignment[color1][color2] = true;
                        alignment[color2][color1] = true;
                    }
                }
                // 下
                if i + 1 < n {
                    let color2 = grid[i + 1][j];
                    if color1 != color2 {
                        alignment[color1][color2] = true;
                        alignment[color2][color1] = true;
                    }
                }
            }
        }
        alignment
    }

    fn is_connected(n: usize, m: usize, grid: &Grid) -> bool {
        let mut color_count_map = vec![0; m + 1];
        for i in 0..n {
            for j in 0..n {
                let color = grid[i][j];
                color_count_map[color] += 1;
            }
        }
        let mut visited = vec![vec![false; n]; n];
        for i in 0..n {
            for j in 0..n {
                if visited[i][j] {
                    continue;
                }
                visited[i][j] = true;
                let color = grid[i][j];
                let mut color_count = 0;
                let mut que = VecDeque::new();
                que.push_back((i, j));
                while let Some((i, j)) = que.pop_front() {
                    color_count += 1;
                    for (di, dj) in &UDLR_DIRS {
                        let new_i = i as isize + di;
                        let new_j = j as isize + dj;
                        if new_i < 0 || new_i >= n as isize || new_j < 0 || new_j >= n as isize {
                            continue;
                        }
                        let new_i = new_i as usize;
                        let new_j = new_j as usize;
                        let color2 = grid[new_i][new_j];
                        if color2 != color {
                            continue;
                        }
                        if visited[new_i][new_j] {
                            continue;
                        }
                        visited[new_i][new_j] = true;
                        que.push_back((new_i, new_j));
                    }
                }
                if color_count != color_count_map[color] {
                    // eprintln!(
                    //     "i:{:?}, j:{:?}, color:{:?}, count:{:?}, count_expected:{:?}",
                    //     i, j, color, color_count, color_count_map[color]
                    // );
                    return false;
                }
            }
        }
        true
    }
}

struct Solver {
    n: usize,
    m: usize,
    city_map: CityMap,
    started_at: Instant,
    rng: ThreadRng,
}

impl Solver {
    fn new(input: Input) -> Self {
        Solver {
            n: input.n,
            m: input.m,
            city_map: CityMap::new(input.n, input.m, input.grid),
            started_at: Instant::now(),
            rng: thread_rng(),
        }
    }

    fn solve(mut self) -> Output {
        let mut count = 0;
        loop {
            if count % 100 == 0 && Instant::now() - self.started_at >= Duration::from_millis(1950) {
                break;
            }
            let i = self.rng.gen_range(0..self.n);
            let j = self.rng.gen_range(0..self.n);
            let k = if count < 10000 || count % 2 == 0 {
                0
            } else {
                let colors = self.city_map.adjacent_colors(i, j);
                *colors.choose(&mut self.rng).unwrap_or(&0)
            };
            let res = self.city_map.simulate_update(i, j, k);
            if res.can_update && res.score >= self.city_map.score {
                self.city_map.update(i, j, k);
            }

            count += 1;
        }
        eprintln!(
            "time: {:?}, count: {}",
            Instant::now() - self.started_at,
            count
        );
        Output {
            grid: self.city_map.output_grid(),
        }
    }
}

struct Input {
    n: usize,
    m: usize,
    grid: Grid,
}

struct Output {
    grid: Grid,
}

fn solve() {
    input! {
        n: usize,
        m: usize,
        grid: [[usize; n]; n]
    };

    let solver = Solver::new(Input { n, m, grid });
    let output = solver.solve();
    println!(
        "{}",
        output
            .grid
            .into_iter()
            .map(|row| row.into_iter().join(" "))
            .join("\n")
    );
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
