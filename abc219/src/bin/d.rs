#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use num::*;
use proconio::input;
#[allow(unused_imports)]
use proconio::marker::*;
#[allow(unused_imports)]
use std::collections::*;

#[macro_export]
macro_rules! chmin {
    ($ min : expr , $ v : expr ) => {
        if $min > $v {
            $min = $v;
            true
        } else {
            false
        }
    };
}

fn solve() {
    input! {
        n: usize,
        x: usize, y: usize,
        abv: [(usize, usize); n]
    };

    let inf = 1_i64 << 60;
    // dp[i][x][y] = 個数
    let mut dp = vec![vec![vec![inf; 300 + 1]; 300 + 1]; n + 1];
    dp[0][0][0] = 0;
    for i in 0..n {
        for j in 0..=x {
            for k in 0..=y {
                chmin!(dp[i + 1][j][k], dp[i][j][k]);
                let new_j = (j + abv[i].0).min(x);
                let new_k = (k + abv[i].1).min(y);
                chmin!(dp[i + 1][new_j][new_k], dp[i][j][k] + 1);
            }
        }
    }
    let mut res = dp[n][x][y];
    if res == inf {
        res = -1;
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
