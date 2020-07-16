#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use num::*;
use proconio::input;
#[allow(unused_imports)]
use proconio::marker::*;
#[allow(unused_imports)]
use std::collections::*;

fn main() {
    input! {
        n: usize,
        pv: [f64; n]
    };

    let mut dp = vec![vec![0_f64; n + 1]; n + 1];
    dp[0][0] = 1.0;
    for i in 0..n {
        for j in 0..i + 1 {
            dp[i + 1][j] += dp[i][j] * (1.0 - pv[i]);
            dp[i + 1][j + 1] += dp[i][j] * pv[i];
        }
    }
    let mut res = 0_f64;
    for i in n / 2 + 1..=n {
        res += dp[n][i];
    }
    println!("{}", res);
}
