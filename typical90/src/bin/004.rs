#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use num::*;
use proconio::input;
#[allow(unused_imports)]
use proconio::marker::*;
#[allow(unused_imports)]
use std::collections::*;

fn solve() {
    input! {
        h: usize, w: usize,
        grid: [[i64; w]; h]
    };

    let mut vertical_cusum = vec![0_i64; w];
    for j in 0..w {
        let mut sum = 0_i64;
        for i in 0..h {
            sum += grid[i][j];
        }
        vertical_cusum[j] = sum;
    }
    let mut horizontal_cusum = vec![0_i64; h];
    for i in 0..h {
        let mut sum = 0_i64;
        for j in 0..w {
            sum += grid[i][j];
        }
        horizontal_cusum[i] = sum;
    }
    for i in 0..h {
        let mut res = Vec::new();
        for j in 0..w {
            let x = vertical_cusum[j] + horizontal_cusum[i] - grid[i][j];
            res.push(x);
        }
        println!("{}", res.iter().join(" "));
    }
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
