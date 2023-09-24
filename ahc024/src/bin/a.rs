#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use num::*;
use proconio::input;
#[allow(unused_imports)]
use proconio::marker::*;
#[allow(unused_imports)]
use std::collections::*;

type Grid = Vec<Vec<usize>>;

type Alignment = Vec<Vec<bool>>;

struct CityMap {
    n: usize,
    m: usize,
    grid: Grid,
    alignment: Alignment,
}

impl CityMap {
    fn new(n: usize, m: usize, grid: Grid) -> Self {
        let alignment = Self::calculate_alignment(n, m, &grid);
        Self {
            n,
            m,
            grid,
            alignment,
        }
    }

    fn calculate_alignment(n: usize, m: usize, grid: &Grid) -> Alignment {
        let mut alignment = vec![vec![false; m]; m];
        for i in 0..n {
            for j in 0..j {
                let color1 = grid[i][j];
                // 右
                if j + 1 < n {
                    let color2 = grid[i][j + 1];
                    alignment[color1][color2] = true;
                    alignment[color2][color1] = true;
                }
                // 下
                if i + 1 < n {
                    let color2 = grid[i + 1][j];
                    alignment[color1][color2] = true;
                    alignment[color2][color1] = true;
                }
            }
        }
        alignment
    }
}

struct Solver {
    n: usize,
    m: usize,
    city_map: CityMap,
}

impl Solver {
    fn new(input: Input) -> Self {
        Solver {
            n: input.n,
            m: input.m,
            city_map: CityMap::new(input.n, input.m, input.grid),
        }
    }

    fn solve(self) -> Output {
        Output {
            grid: vec![vec![0; self.n]; self.n],
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
        grid: [[Usize1; n]; n]
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
