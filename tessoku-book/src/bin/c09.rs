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
macro_rules! chmax {
    ($ max : expr , $ v : expr ) => {{
        let v = $v;
        if $max < v {
            $max = v;
            true
        } else {
            false
        }
    }};
}

fn solve() {
    input! {
        n: usize,
        av: [i64; n]
    };

    let mut dp = vec![vec![0; 2]; n + 1];
    for i in 0..n {
        // 勉強しない
        chmax!(dp[i + 1][0], dp[i][0]);
        chmax!(dp[i + 1][0], dp[i][1]);

        // 勉強する
        chmax!(dp[i + 1][1], dp[i][0] + av[i]);
    }

    let res = dp[n][0].max(dp[n][1]);
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
