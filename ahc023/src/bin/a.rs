#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use num::*;
use proconio::input;
#[allow(unused_imports)]
use proconio::marker::*;
#[allow(unused_imports)]
use std::collections::*;

struct Solver {
    t: usize,
    h: usize,
    w: usize,
    i0: usize,
    h_grid: Vec<Vec<char>>,
    v_grid: Vec<Vec<char>>,
    k: usize,
    sdv: Vec<(usize, usize)>,
}

struct Input {
    t: usize,
    h: usize,
    w: usize,
    i0: usize,
    h_grid: Vec<Vec<char>>,
    v_grid: Vec<Vec<char>>,
    k: usize,
    sdv: Vec<(usize, usize)>,
}

impl Solver {
    fn new(input: Input) -> Self {
        Self {
            t: input.t,
            h: input.h,
            w: input.w,
            i0: input.i0,
            h_grid: input.h_grid,
            v_grid: input.v_grid,
            k: input.k,
            sdv: input.sdv,
        }
    }
    fn solve(mut self) {}
}

fn solve() {
    input! {
        t: usize, h: usize, w: usize, i0: usize,
        h_grid: [Chars; h - 1],
        v_grid: [Chars; h],
        k: usize,
        sdv: [(usize, usize); k]
    };

    let input = Input {
        t,
        h,
        w,
        i0,
        h_grid,
        v_grid,
        k,
        sdv,
    };
    let mut solver = Solver::new(input);
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
