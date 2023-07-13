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
        n: usize, m: usize,
        av: [i64; n]
    };

    let mut dp = vec![vec![-(1_i64 << 60); m + 10]; n + 10];
    dp[0][0] = 0;
    for i in 0..n {
        for j in 0..=m {
            // 選ばない場合
            chmax!(dp[i + 1][j], dp[i][j]);
            // 選んだ場合
            chmax!(dp[i + 1][j + 1], dp[i][j] + av[i] * (j + 1) as i64);
        }
    }
    let res = dp[n][m];
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
