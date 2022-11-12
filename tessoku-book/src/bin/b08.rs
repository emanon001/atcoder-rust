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
        n: usize,
        points: [(usize, usize); n],
        q: usize,
        rectangles: [(usize, usize, usize, usize); q]
    };

    let size = 1500;
    let mut cusum = vec![vec![0; size + 1]; size + 1];
    for (x, y) in points {
        cusum[y][x] += 1;
    }
    for i in 0..size + 1 {
        for j in 0..size {
            cusum[i][j + 1] += cusum[i][j];
        }
    }
    for j in 0..size + 1 {
        for i in 0..size {
            cusum[i + 1][j] += cusum[i][j];
        }
    }

    for (a, b, c, d) in rectangles {
        let res = cusum[d][c] - cusum[d][a - 1] - cusum[b - 1][c] + cusum[b - 1][a - 1];
        println!("{}", res);
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
