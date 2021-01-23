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
        sv: [String; n]
    };

    let mut dp = vec![vec![0_i64; 2]; n + 1];
    dp[0][0] = 1;
    dp[0][1] = 1;
    for i in 0..n {
        if sv[i] == "AND" {
            // x_i == 0 の場合
            dp[i + 1][0] += dp[i][0];
            dp[i + 1][0] += dp[i][1];
            // x_i == 1 の場合
            dp[i + 1][0] += dp[i][0];
            dp[i + 1][1] += dp[i][1];
        } else {
            // OR
            // x_i == 0 の場合
            dp[i + 1][0] += dp[i][0];
            dp[i + 1][1] += dp[i][1];
            // x_i == 1 の場合
            dp[i + 1][1] += dp[i][0];
            dp[i + 1][1] += dp[i][1];
        }
    }
    println!("{}", dp[n][1]);
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
