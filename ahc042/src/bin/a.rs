#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use num::*;
use proconio::input_interactive;
#[allow(unused_imports)]
use proconio::marker::*;
#[allow(unused_imports)]
use std::collections::*;

struct Solver {
    n: usize,
    board: Vec<Vec<char>>,
}

impl Solver {
    fn new(n: usize, board: Vec<Vec<char>>) -> Self {
        Self { n, board }
    }
    fn solve(&mut self) {
        todo!();
    }
}

#[allow(non_snake_case)]
fn solve() {
    input_interactive! {
        N: usize,
        C: [Chars; N],
    };

    let mut solver = Solver::new(N, C);
    solver.solve();
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
