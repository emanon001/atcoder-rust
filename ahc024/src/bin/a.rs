#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use num::*;
use petgraph::visit::VisitMap;
use proconio::input;
#[allow(unused_imports)]
use proconio::marker::*;
use rand::prelude::*;
#[allow(unused_imports)]
use std::collections::*;
use std::time::{Duration, Instant};

/// 上下左右 (i, j)
pub const UDLR_DIRS: [(isize, isize); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

type Grid = Vec<Vec<usize>>;

type Alignment = Vec<Vec<bool>>;

struct SimulateResult {
    can_update: bool,
    score: usize,
}

struct CityMap {
    raw_n: usize,
    n: usize,
    m: usize,
    grid: Grid,
    alignment: Alignment,
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
        }
    }

    fn simulate_update(&mut self, i: usize, j: usize, k: usize) -> bool {
        let i = i + 1;
        let j = j + 1;
        let bk_k = self.grid[i][j];
        self.grid[i][j] = k;
        let new_alignment = Self::calculate_alignment(self.n, self.m, &self.grid);
        let is_connected = Self::is_connected(self.n, self.m, &self.grid);
        self.grid[i][j] = bk_k;
        is_connected && self.alignment == new_alignment
    }

    fn update(&mut self, i: usize, j: usize, k: usize) {
        let i = i + 1;
        let j = j + 1;
        self.grid[i][j] = k;
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
            if count % 50 == 0 && Instant::now() - self.started_at >= Duration::from_millis(1950) {
                break;
            }
            let i = self.rng.gen_range(0..self.n);
            let j = self.rng.gen_range(0..self.n);
            if self.city_map.simulate_update(i, j, 0) {
                self.city_map.update(i, j, 0);
            }

            count += 1;
        }
        eprintln!("time: {:?}", Instant::now() - self.started_at);
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
