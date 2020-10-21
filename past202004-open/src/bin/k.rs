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
        s: Chars,
        cv: [i64; n],
        dv: [i64; n],
    };

    let mut dp = vec![vec![1_i64 << 60; n + 1]; n + 1];
    dp[0][0] = 0_i64;
    for i in 0..n {
        let ch = s[i];
        for j in 0..n {
            match ch {
                '(' => {
                    // not change
                    dp[i + 1][j + 1] = dp[i + 1][j + 1].min(dp[i][j]);
                    // change
                    if j > 0 {
                        dp[i + 1][j - 1] = dp[i + 1][j - 1].min(dp[i][j] + cv[i]);
                    }
                    // delete
                    dp[i + 1][j] = dp[i + 1][j].min(dp[i][j] + dv[i]);
                }
                ')' => {
                    // not change
                    if j > 0 {
                        dp[i + 1][j - 1] = dp[i + 1][j - 1].min(dp[i][j]);
                    }
                    // change
                    dp[i + 1][j + 1] = dp[i + 1][j + 1].min(dp[i][j] + cv[i]);
                    // delete
                    dp[i + 1][j] = dp[i + 1][j].min(dp[i][j] + dv[i]);
                }
                _ => unreachable!(),
            };
        }
    }
    println!("{}", dp[n][0]);
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
