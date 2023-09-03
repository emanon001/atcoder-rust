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

struct Plan {
    k: usize,
    i: usize,
    j: usize,
    s: usize,
}
struct Output {
    m: usize,
    plans: Vec<Plan>,
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
    fn solve(self) -> Output {
        Output {
            m: 0,
            plans: vec![],
        }
    }
}

fn main() {
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
    let solver = Solver::new(input);
    let output = solver.solve();
    println!("{}", output.m);
    println!(
        "{}",
        output
            .plans
            .iter()
            .map(|p| format!("{} {} {} {}", p.k, p.i, p.j, p.s))
            .join("\n")
    );
}
