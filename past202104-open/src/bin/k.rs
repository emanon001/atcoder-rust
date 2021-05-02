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
    ($ max : expr , $ v : expr ) => {
        if $max < $v {
            $max = $v;
            true
        } else {
            false
        }
    };
}

fn solve() {
    input! {
        n: usize,
        puv: [(i64, i64); n]
    };

    let mut dp = vec![vec![-(1_i64 << 60); 100]; n + 1];
    dp[0][0] = 0;
    for i in 0..n {
        let (p, u) = puv[i];
        for j in 0..100 {
            // 購入しない場合
            chmax!(dp[i + 1][j], dp[i][j]);
            // 購入する場合
            let x = (j as i64 + p) / 100;
            chmax!(dp[i + 1][(j + p as usize) % 100], dp[i][j] + u - p + x * 20);
        }
    }
    let mut res = 0;
    for &x in &dp[n] {
        chmax!(res, x);
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
