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
        board: [[i32; w]; h],
        q: usize,
        rectangles: [(usize, usize, usize, usize); q]
    };

    // build cusum
    let mut cusum = vec![vec![0_i32; w + 1]; h + 1];
    for i in 0..h {
        for j in 0..w {
            cusum[i + 1][j + 1] = board[i][j];
        }
    }
    for i in 0..h + 1 {
        for j in 0..w {
            cusum[i][j + 1] += cusum[i][j];
        }
    }
    for j in 0..w + 1 {
        for i in 0..h {
            cusum[i + 1][j] += cusum[i][j];
        }
    }

    // solve query
    for (a, b, c, d) in rectangles {
        let res = cusum[c][d] - cusum[c][b - 1] - cusum[a - 1][d] + cusum[a - 1][b - 1];
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
