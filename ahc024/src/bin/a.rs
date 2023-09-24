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

struct CityMap {
    n: usize,
    m: usize,
    grid: Grid,
}

impl CityMap {
    fn new(n: usize, m: usize, grid: Grid) -> Self {
        Self { n, m, grid }
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

struct Output {
    grid: Grid,
}

struct Input {
    n: usize,
    m: usize,
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
