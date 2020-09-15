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

    let mut cusum = vec![vec![0; w + 1]; h + 1];
    for i in 0..h {
        for j in 0..w {
            let mut p = grid[i][j];
            if i % 2 == 0 {
                p *= if j % 2 == 0 { 1 } else { -1 };
            } else {
                p *= if j % 2 == 0 { -1 } else { 1 };
            };
            let c = cusum[i + 1][j] + cusum[i][j + 1] - cusum[i][j] + p;
            cusum[i + 1][j + 1] = c;
        }
    }
    let mut res = 0;
    for t in 0..h {
        for l in 0..w {
            for b in t..h {
                for r in l..w {
                    let p = cusum[b + 1][r + 1] - cusum[b + 1][l] - cusum[t][r + 1] + cusum[t][l];
                    let c = (b - t + 1) * (r - l + 1);
                    if p == 0 && c > res {
                        res = c;
                    }
                }
            }
        }
    }
    println!("{}", res);
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
