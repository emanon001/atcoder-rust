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
        n: usize, x: usize,
        abv: [(usize, usize); n]
    };

    let max_pos = 100 * 100;
    let mut dp = vec![vec![false; max_pos + 10]; n + 1];
    dp[0][0] = true;
    for i in 0..n {
        let (a, b) = abv[i];
        for j in 0..=max_pos {
            if j + a <= max_pos {
                dp[i + 1][j + a] |= dp[i][j];
            }
            if j + b <= max_pos {
                dp[i + 1][j + b] |= dp[i][j];
            }
        }
    }
    let res = if dp[n][x] { "Yes" } else { "No" };
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
