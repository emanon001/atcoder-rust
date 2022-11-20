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
        n: usize, s: usize,
        av: [usize; n]
    };

    let max = 10000;
    let mut dp = vec![vec![false; max + 1]; n + 1];
    dp[0][0] = true;
    for i in 0..n {
        let a = av[i];
        for j in 0..=max {
            dp[i + 1][j] |= dp[i][j];
            if j + a <= max {
                dp[i + 1][j + a] |= dp[i][j];
            }
        }
    }
    let res = if dp[n][s] { "Yes" } else { "No" };
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
