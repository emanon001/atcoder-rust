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
        n: usize, w: i64,
        wvv: [(i64, usize); n]
    };

    let maxv = n * 1000;
    let mut dp = vec![vec![1_i64 << 60; maxv + 1]; n + 1];
    dp[0][0] = 0;
    for i in 0..n {
        let (wi, vi) = wvv[i];
        for j in 0..=maxv {
            chmin!(dp[i + 1][j], dp[i][j]);
            if j + vi <= maxv {
                chmin!(dp[i + 1][j + vi], dp[i][j] + wi);
            }
        }
    }
    let mut res = 0;
    for i in 0..=maxv {
        if dp[n][i] <= w {
            res = i;
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
