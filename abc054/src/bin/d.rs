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
        n: usize, ma: usize, mb: usize,
        abcv: [(usize, usize, usize); n]
    };

    let inf = 1_usize << 30;
    let max_g = 10 * n;
    let mut dp = vec![vec![vec![inf; max_g + 1]; max_g + 1]; n + 1];
    dp[0][0][0] = 0;
    for i in 0..n {
        let (a, b, c) = abcv[i];
        for j in 0..max_g + 1 {
            for k in 0..max_g + 1 {
                dp[i + 1][j][k] = dp[i + 1][j][k].min(dp[i][j][k]);
                if j + a > max_g || k + b > max_g {
                    continue;
                }
                dp[i + 1][j + a][k + b] = dp[i + 1][j + a][k + b].min(dp[i][j][k] + c);
            }
        }
    }
    let mut res = inf;
    for i in 1..max_g + 1 {
        for j in 1..max_g + 1 {
            let c = dp[n][i][j];
            if c == inf {
                continue;
            }
            let gcd = i.gcd(&j);
            if i / gcd == ma && j / gcd == mb {
                if c < res {
                    res = c;
                }
            }
        }
    }
    let res = if res == inf { -1 } else { res as isize };
    println!("{}", res);
}
