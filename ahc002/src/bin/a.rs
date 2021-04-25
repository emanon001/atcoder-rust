#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use num::*;
use proconio::input;
#[allow(unused_imports)]
use proconio::marker::*;
#[allow(unused_imports)]
use std::collections::*;

fn score(path: &[(usize, usize)], grid: &[Vec<i32>]) -> i32 {
    let mut res = 0;
    for &(i, j) in path {
        res += grid[i][j];
    }
    res
}

fn solve() {
    input! {
        si: usize, sj: usize,
        tgrid: [[i32; 50]; 50],
        pgrid: [[i32; 50]; 50],
    };
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
