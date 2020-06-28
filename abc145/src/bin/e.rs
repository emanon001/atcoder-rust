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
        n: usize, t: usize,
        mut abv: [(usize, usize); n]
    };

    abv.sort();
    let mut dp = vec![vec![0; t + 1]; n + 1];
    for i in 0..n {
        let (a, b) = abv[i];
        for j in 0..=t {
            let to_i = i + 1;
            let from_v = dp[i][j];

            // 食べない場合
            let to_j = j;
            let to_v = dp[to_i][to_j];
            dp[to_i][to_j] = to_v.max(from_v);

            // 食べた場合
            if j < t {
                let to_j = (j + a).min(t);
                let to_v = dp[to_i][to_j];
                dp[to_i][to_j] = to_v.max(from_v + b);
            }
        }
    }
    println!("{}", dp[n][t]);
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
