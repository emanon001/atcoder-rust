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
        h: usize, w: usize, n: usize,
        rectangles: [(usize, usize, usize, usize); n]
    };

    let mut cusum = vec![vec![0_i32; w + 2]; h + 2];
    for (y1, x1, y2, x2) in rectangles {
        cusum[y1][x1] += 1;
        cusum[y1][x2 + 1] -= 1;
        cusum[y2 + 1][x1] -= 1;
        cusum[y2 + 1][x2 + 1] += 1;
    }
    for i in 0..h + 2 {
        for j in 0..w + 1 {
            cusum[i][j + 1] += cusum[i][j];
        }
    }
    for j in 0..w + 2 {
        for i in 0..h + 1 {
            cusum[i + 1][j] += cusum[i][j];
        }
    }

    for row in cusum.into_iter().skip(1).take(h) {
        println!("{}", row.iter().skip(1).take(w).join(" "));
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
