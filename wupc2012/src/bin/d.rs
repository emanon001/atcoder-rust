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
    };

    let mut triangle = vec![vec![0_u32; n + 1]; n + 1];
    for i in 0..n {
        input! {
            v: [u32; i + 1]
        };
        for j in 0..v.len() {
            triangle[i][j] = v[j];
        }
    }
    let mut dp = vec![vec![0_u32; n + 1]; n + 1];
    dp[0][0] = triangle[0][0];
    for i in 0..n - 1 {
        for j in 0..n {
            chmax!(dp[i + 1][j], dp[i][j] + triangle[i + 1][j]);
            chmax!(dp[i + 1][j + 1], dp[i][j] + triangle[i + 1][j + 1]);
        }
    }
    let mut res = 0_u32;
    for i in 0..n {
        chmax!(res, dp[n - 1][i]);
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
